use std::collections::HashMap;

use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day7 = Day7 {};

    day7.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day7 = Day7 {};

    day7.test(
        Part::One,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        6440,
    );
}

#[test]
fn test_part2() {
    let mut day7 = Day7 {};

    day7.test(
        Part::Two,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        5905,
    );
}

pub struct Day7;
//#[derive(Debug, Copy, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
//pub enum Card {
//    Two = 2,
//    Three = 3,
//    Four = 4,
//    Five = 5,
//    Six = 6,
//    Seven = 7,
//    Eight = 8,
//    Nine = 9,
//    Ten = 10,
//    Jack = 11,
//    Queen = 12,
//    King = 13,
//    Ace = 14,
//}

#[derive(Debug, Copy, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
pub enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Copy, Clone, Ord, PartialEq, PartialOrd, Eq, Hash)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    Unknown = 0,
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    adj_cards: Vec<Card>,
    bid: usize,
    kind: HandType,
}

impl Hand {
    pub fn compute(&mut self) {
        let mut map: HashMap<Card, usize> = HashMap::with_capacity(12);

        for c in &self.cards {
            if map.contains_key(c) {
                if let Some(v) = map.get_mut(c) {
                    *v += 1;
                }
            } else {
                map.insert(*c, 1);
            }
        }
        let mut card_counts = map.clone().into_values().collect::<Vec<usize>>();
        card_counts.sort_unstable();

        match map.len() {
            // only found one type of card in the hand
            1 => self.kind = HandType::FiveOfAKind,
            // found two types of cards in the hand
            // either 4 + 1 or 3 + 2
            2 => {
                debug_assert!(card_counts.len() == 2);
                match &card_counts[..] {
                    [1, 4] => self.kind = HandType::FourOfAKind,
                    [2, 3] => self.kind = HandType::FullHouse,
                    _ => unreachable!(),
                }
            }
            // found 3 types of cards in the hand
            // either 2 + 2 + 1 or 3 + 1 + 1
            3 => {
                debug_assert!(card_counts.len() == 3);
                match &card_counts[..] {
                    [1, 2, 2] => self.kind = HandType::TwoPair,
                    [1, 1, 3] => self.kind = HandType::ThreeOfAKind,
                    _ => unreachable!(),
                }
            }
            // found 4 types of cards in the hand
            // 2 + 1 + 1 + 1
            4 => self.kind = HandType::OnePair,
            // found 5 types of cards in the hand
            // 1 + 1 + 1 + 1 + 1
            5 => self.kind = HandType::HighCard,
            _ => unreachable!(),
        }
    }

    pub fn compute_with_joker(&mut self) {
        let mut map: HashMap<Card, usize> = HashMap::with_capacity(63);

        for c in &self.cards {
            if map.contains_key(c) {
                if let Some(v) = map.get_mut(c) {
                    *v += 1;
                }
            } else {
                map.insert(*c, 1);
            }
        }

        let mut card_counts = map.clone().into_iter().collect::<Vec<(Card, usize)>>();
        card_counts.sort_unstable();

        // if we have one or more jokers
        if map.contains_key(&Card::Joker) {
            let jokers = *map.get(&Card::Joker).unwrap();
            map.remove(&Card::Joker);
            // need to update card_counts
            card_counts = map.clone().into_iter().collect::<Vec<(Card, usize)>>();
            card_counts.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
            self.remove_jokers();
            match (jokers, map.len()) {
                // only found one type of card in the hand and they were all jokers
                (_, 0) => {
                    self.adj_cards = vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]
                }
                // only one other card and 4 jacks
                (_, 1) => {
                    let c = self.cards[0];
                    self.adj_cards = vec![c, c, c, c, c]
                }
                // found two types of cards in the hand + a joker
                // either 3 + 1 or 2 + 2
                // should be one joker?
                (_, 2) => {
                    match &card_counts[..] {
                        [(single_card, 1), (pair, 2)] => {
                            self.adj_cards = vec![*single_card, *pair, *pair, *pair, *pair]
                        }
                        [(single_card, 1), (tok, 3)] => {
                            self.adj_cards = vec![*single_card, *tok, *tok, *tok, *tok]
                        }
                        [(pair1, 2), (pair2, 2)] => {
                            // need to determine the higher of the 2 pairs
                            if pair1 > pair2 {
                                self.adj_cards = vec![*pair1, *pair1, *pair1, *pair2, *pair2]
                            } else {
                                self.adj_cards = vec![*pair2, *pair2, *pair2, *pair1, *pair1]
                            }
                        }
                        [(first, 1), (second, 1)] => {
                            // need to determine the higher of the 2 pairs
                            if first > second {
                                self.adj_cards = vec![*first, *first, *first, *first, *second]
                            } else {
                                self.adj_cards = vec![*second, *second, *second, *second, *first]
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                // found 3 types of cards in the hand + a joker
                // 1 + 1 + 2 + J
                (_, 3) => match &card_counts[..] {
                    [(first_single, 1), (second_single, 1), (pair, 2)] 
                    | [(pair, 2), (first_single, 1), (second_single, 1)]
                    | [(first_single, 1) ,(pair, 2),  (second_single, 1)] => {
                        self.adj_cards = vec![*pair, *pair, *pair, *first_single, *second_single]
                    }
                    [(first_single, 1), (second_single, 1), (third_single, 1)] => {
                        self.adj_cards = vec![*first_single, *first_single, *first_single, *second_single, *third_single]
                    }
                    _ => unreachable!(),
                },
                // found 4 types of cards in the hand
                // 1 + 1 + 1 + 1 + J
                (_, 4) => {
                    // get highest card
                    let mut cc = self.cards.clone();
                    cc.sort();
                    let hc = cc[0];
                    self.adj_cards = vec![hc, hc, cc[1], cc[2], cc[3]];
                }
                _ => unreachable!(),
            }
            // be sure to reset the map and card_counts if we had jokers!
            map.clear();
            for c in &self.adj_cards {
                if map.contains_key(c) {
                    if let Some(v) = map.get_mut(c) {
                        *v += 1;
                    }
                } else {
                    map.insert(*c, 1);
                }
            }
            card_counts = map.clone().into_iter().collect::<Vec<(Card, usize)>>();
            card_counts.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        }

        match map.len() {
            // only found one type of card in the hand
            1 => self.kind = HandType::FiveOfAKind,
            // found two types of cards in the hand
            // either 4 + 1 or 3 + 2
            2 => match &card_counts[..] {
                [(_, 1), (_, 4)] | [(_, 4), (_, 1)] => self.kind = HandType::FourOfAKind,
                [(_, 2), (_, 3)] | [(_, 3), (_, 2)] => self.kind = HandType::FullHouse,
                _ => unreachable!(),
            },
            // found 3 types of cards in the hand
            // either 2 + 2 + 1 or 3 + 1 + 1
            3 => match &card_counts[..] {
                [(_, 1), (_, 2), (_, 2)] | [(_, 2), (_, 2), (_, 1)] | [(_, 2), (_, 1), (_, 2)] => self.kind = HandType::TwoPair,
                [(_, 1), (_, 1), (_, 3)] | [(_, 3), (_, 1), (_, 1)] | [(_, 1), (_, 3), (_, 1)] => self.kind = HandType::ThreeOfAKind,
                _ => unreachable!(),
            },
            // found 4 types of cards in the hand
            // 2 + 1 + 1 + 1
            4 => self.kind = HandType::OnePair,
            // found 5 types of cards in the hand
            // 1 + 1 + 1 + 1 + 1
            5 => self.kind = HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn remove_jokers(&mut self) {
        for i in 0..self.cards.len() {
            if self.cards[i] != Card::Joker {
                self.adj_cards.push(self.cards[i]);
            }
        }
    }
}

impl Runner for Day7 {
    fn part1(&mut self, input: &str) -> i32 {
        let mut hands = vec![];
        let lines = input.lines().collect::<Vec<&str>>();
        for line in lines {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards
                .chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            let bid = bid.parse::<usize>().unwrap();
            hands.push(Hand {
                cards,
                adj_cards: vec![],
                bid,
                kind: HandType::Unknown,
            });
        }

        for hand in &mut hands {
            hand.compute();
        }
        // all hands now have their type computed
        // now we sort by type
        hands.sort_by(|a, b| a.kind.partial_cmp(&b.kind).unwrap());
        //dbg!(&hands);
        let mut map: HashMap<HandType, Vec<&Hand>> = HashMap::with_capacity(8);
        for hand in &hands {
            if map.contains_key(&hand.kind) {
                if let Some(v) = map.get_mut(&hand.kind) {
                    v.push(hand);
                }
            } else {
                map.insert(hand.kind, vec![hand]);
            }
        }
        //dbg!(&map);
        // now we sort types by card values
        for (_kind, hands) in map.iter_mut() {
            hands.sort_by(|a, b| {
                for (ls, rs) in a.cards.iter().zip(b.cards.iter()) {
                    if let Some(ord) = ls.partial_cmp(rs) {
                        match ord {
                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        }
                    }
                }
                std::cmp::Ordering::Equal
            })
        }
        //dbg!(&map);
        // build full list of hands
        let mut all_hands = map.values().into_iter().flatten().collect::<Vec<&&Hand>>();
        all_hands.sort_by(|a, b| a.kind.partial_cmp(&b.kind).unwrap());
        let mut score = 0;
        for (i, hand) in all_hands.iter().enumerate() {
            score += (i + 1) * hand.bid;
        }
        score as i32
    }

    fn part2(&mut self, input: &str) -> i32 {
        let mut hands = vec![];
        let lines = input.lines().collect::<Vec<&str>>();
        for line in lines {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards
                .chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Joker, // Jacks are out, Jokers are in!
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            let bid = bid.parse::<usize>().unwrap();
            hands.push(Hand {
                cards,
                adj_cards: vec![],
                bid,
                kind: HandType::Unknown,
            });
        }

        for hand in &mut hands {
            hand.compute_with_joker();
        }
        // all hands now have their type computed
        // now we sort by type
        hands.sort_by(|a, b| a.kind.partial_cmp(&b.kind).unwrap());
        //dbg!(&hands);
        let mut map: HashMap<HandType, Vec<&Hand>> = HashMap::with_capacity(8);
        for hand in &hands {
            if map.contains_key(&hand.kind) {
                if let Some(v) = map.get_mut(&hand.kind) {
                    v.push(hand);
                }
            } else {
                map.insert(hand.kind, vec![hand]);
            }
        }
        //dbg!(&map);
        // now we sort types by card values
        for (_kind, hands) in map.iter_mut() {
            hands.sort_by(|a, b| {
                for (ls, rs) in a.cards.iter().zip(b.cards.iter()) {
                    if let Some(ord) = ls.partial_cmp(rs) {
                        match ord {
                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        }
                    }
                }
                std::cmp::Ordering::Equal
            })
        }
        //dbg!(&map);
        // build full list of hands
        let mut all_hands = map.values().into_iter().flatten().collect::<Vec<&&Hand>>();
        all_hands.sort_by(|a, b| a.kind.partial_cmp(&b.kind).unwrap());
        let mut score = 0;
        for (i, hand) in all_hands.iter().enumerate() {
            score += (i + 1) * hand.bid;
        }
        score as i32
    }

    fn year_and_day(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
