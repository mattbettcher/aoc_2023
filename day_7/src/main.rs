use std::collections::HashMap;

use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day7 = Day7 {};
    day7.run(Part::One, input);   // 250347426
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
    cards: [Card; 5],
    bid: usize,
    kind: HandType,
}

impl Hand {
    fn count_cards(&self) -> (bool, Vec<usize>, usize) {
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
        let joker = map.contains_key(&Card::Joker);
        if joker { map.remove(&Card::Joker); }
        let mut cc = map.values().map(|v|*v).collect::<Vec<usize>>();
        cc.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        (joker, cc, map.len())
    }

    pub fn compute_with_joker(&mut self) {
        let (has_joker, cc, cards) = self.count_cards();
        match (has_joker, cards) {
            (true, 0) => self.kind = HandType::FiveOfAKind,
            (false, 1) => self.kind = HandType::FiveOfAKind,
            (true, 1) => self.kind = HandType::FiveOfAKind,
            (false, 2) => {
                match &cc[..] {
                    [1, 4] => self.kind = HandType::FourOfAKind,
                    [2, 3] => self.kind = HandType::FullHouse,
                    _ => unreachable!(),
                }
            }
            (true, 2) => {
                match &cc[..] {
                    [1, 2] => self.kind = HandType::FourOfAKind,
                    [1, 3] => self.kind = HandType::FourOfAKind,
                    [2, 2] => self.kind = HandType::FullHouse,
                    [1, 1] => self.kind = HandType::FourOfAKind,
                    _ => unreachable!(),
                }
            }
            (false, 3) => match &cc[..] {
                [1, 2, 2] => self.kind = HandType::TwoPair,
                [1, 1, 3] => self.kind = HandType::ThreeOfAKind,
                _ => unreachable!(),
            },
            (true, 3) => self.kind = HandType::ThreeOfAKind,
            (false, 4) => self.kind = HandType::OnePair,
            (true, 4) => self.kind = HandType::OnePair,
            (false, 5) => self.kind = HandType::HighCard,
            _ => unreachable!(),
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
            // we could count the cards here and not have to keep them around at all...
            let bid = bid.parse::<usize>().unwrap();
            hands.push(Hand {
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
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
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
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