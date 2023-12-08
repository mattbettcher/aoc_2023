use std::collections::HashMap;

use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day7 = Day7 {};
    //day7.run(Part::One, input);   // 250347426
    day7.run(Part::Two, input);     // 251224870
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
        let mut map: HashMap<Card, usize> = HashMap::with_capacity(7);

        for c in &self.cards {
            if map.contains_key(c) {
                if let Some(v) = map.get_mut(c) {
                    *v += 1;
                }
            } else {
                map.insert(*c, 1);
            }
        }
        let has_joker = map.contains_key(&Card::Joker);
        // if we have one or more jokers
        self.remove_jokers();
        if has_joker {
            map.remove(&Card::Joker);
        }

        let mut card_counts = map.clone().into_iter().collect::<Vec<(Card, usize)>>();
        card_counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        match (has_joker, map.len()) {
            // all jokers
            (true, 0) => {
                self.adj_cards = vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
                self.kind = HandType::FiveOfAKind;
            }
            (false, 1) => self.kind = HandType::FiveOfAKind,
            (true, 1) => {
                let c = self.adj_cards[0];
                self.adj_cards = vec![c, c, c, c, c];
                self.kind = HandType::FiveOfAKind;
            }
            (false, 2) => {
                match &card_counts.iter().map(|(_, i)| *i).collect::<Vec<usize>>()[..] {
                    [1, 4] => self.kind = HandType::FourOfAKind,
                    [2, 3] => self.kind = HandType::FullHouse,
                    _ => unreachable!(),
                }
            }
            (true, 2) => {
                match &card_counts[..] {
                    [(first, 1), (pair, 2)] => {
                        self.adj_cards = vec![*first, *pair, *pair, *pair, *pair];
                        self.kind = HandType::FourOfAKind;
                    }
                    [(first, 1), (tok, 3)] => {
                        self.adj_cards = vec![*first, *tok, *tok, *tok, *tok];
                        self.kind = HandType::FourOfAKind;
                    }
                    [(pair1, 2), (pair2, 2)] => {
                        // need to determine the higher of the 2 pairs
                        if pair1 > pair2 {
                            self.adj_cards = vec![*pair1, *pair1, *pair1, *pair2, *pair2];
                            self.kind = HandType::ThreeOfAKind;
                        } else {
                            self.adj_cards = vec![*pair2, *pair2, *pair2, *pair1, *pair1];
                            self.kind = HandType::ThreeOfAKind;
                        }
                    }
                    [(first, 1), (second, 1)] => {
                        // need to determine the higher of the 2 cards
                        if first > second {
                            self.adj_cards = vec![*first, *first, *first, *first, *second];
                            self.kind = HandType::FourOfAKind;
                        } else {
                            self.adj_cards = vec![*second, *second, *second, *second, *first];
                            self.kind = HandType::FourOfAKind;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            (false, 3) => match &card_counts.iter().map(|(_, i)| *i).collect::<Vec<usize>>()[..] {
                [1, 2, 2] => self.kind = HandType::TwoPair,
                [1, 1, 3] => self.kind = HandType::ThreeOfAKind,
                _ => unreachable!(),
            },
            (true, 3) => match &card_counts[..] {
                [(first, 1), (second, 1), (pair, 2)] => {
                    self.adj_cards = vec![*pair, *pair, *pair, *first, *second];
                    self.kind = HandType::ThreeOfAKind;
                }
                [(first, 1), (sec, 1), (third, 1)] => {
                    // only one where high card isn't correct
                    if first > sec && first > third {
                        self.adj_cards = vec![*first, *first, *first, *sec, *third];
                    } else if sec > first && sec > third {
                        self.adj_cards = vec![*sec, *sec, *sec, *first, *third];
                    } else {
                        self.adj_cards = vec![*third, *third, *third, *first, *sec];
                    }
                    self.kind = HandType::ThreeOfAKind;
                }
                _ => unreachable!(),
            },
            (false, 4) => self.kind = HandType::OnePair,
            (true, 4) => {
                // get highest card
                let mut cc = self.cards.clone();
                cc.sort();
                cc.reverse();
                let hc = cc[0];
                self.adj_cards = vec![hc, hc, cc[1], cc[2], cc[3]];
                self.kind = HandType::OnePair;
            }
            (false, 5) => self.kind = HandType::HighCard,
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
