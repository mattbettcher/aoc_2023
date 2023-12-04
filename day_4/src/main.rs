use std::collections::HashMap;

use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day4 = Day4 {};

    day4.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day4 = Day4 {};

    day4.test(
        Part::One,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        13,
    );
}

#[test]
fn test_part2() {
    let mut day4 = Day4 {};

    day4.test(
        Part::Two,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        30,
    );
}

pub struct Day4;
#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub winners: Vec<u32>,
    pub num_you_have: Vec<u32>,
    pub copies: Vec<Card>,
}

impl Card {
    pub fn new(id: usize, winners: Vec<u32>, num_you_have: Vec<u32>) -> Self {
        Card {
            id,
            winners,
            num_you_have,
            copies: vec![]
        }
    }

    pub fn points(&self) -> u32 {
        let mut val = 0;
        for nyh in &self.num_you_have {
            if self.winners.contains(nyh) {
                if val == 0 {
                    val = 1;
                } else {
                    val *= 2;
                }
            }
        }
        val
    }

    pub fn matches(&self) -> u32 {
        self.num_you_have
            .iter()
            .map(|nyh| if self.winners.contains(&nyh) { 1 } else { 0 })
            .sum::<u32>()
    }

    pub fn get_copies(&self, cards: &Vec<Card>) -> Vec<Self> {
        let mut copies = vec![];
        let (id, matches) = (self.id, self.matches());
        for i in id..id+matches as usize {
            copies.push(cards[i].clone());
        }
        copies
    }

    pub fn make_copies_from_winners(ori_cards: &Vec<Card>, cards: &mut Vec<Card>) {
        for i in 0..cards.len() {
            let winners = cards[i].get_copies(ori_cards);
            cards[i].copies = winners;
            Card::make_copies_from_winners(ori_cards, &mut cards[i].copies);
        }
    }

    pub fn count_instances(cards: &Vec<Card>, hs: &mut HashMap<usize, usize>) {
        for i in 0..cards.len() {
            if hs.contains_key(&cards[i].id) {
                if let Some(c) = hs.get_mut(&cards[i].id) {
                    *c += 1;
                }
            } else if cards[i].copies.len() > 0 {
                hs.insert(cards[i].id, 1);
            } else {
                hs.insert(cards[i].id, 1);
            }
            Card::count_instances(&cards[i].copies, hs);
        }
    }

    pub fn print(&self) {
        print!("{}", self.id);
        for c in &self.copies {
            print!(" "); 
            c.print();
        }
        println!();
    }
}

impl Runner for Day4 {
    fn part1(&mut self, input: &str) -> i32 {
        let cards: Vec<Card> = input
            .lines()
            .map(|line| {
                let (id, line) = line[4..].split_once(':').unwrap();
                let (winners, nums_you_have) = line.split_once('|').unwrap();
                let winners: Vec<u32> = winners
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                let nums_you_have: Vec<u32> = nums_you_have
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                Card::new(id.trim().parse().unwrap(), winners, nums_you_have)
            })
            .collect();
        cards.into_iter().map(|c| c.points()).sum::<u32>() as i32
    }

    fn part2(&mut self, input: &str) -> i32 {
        let mut cards: Vec<Card> = input
            .lines()
            .map(|line| {
                let (id, line) = line[4..].split_once(':').unwrap();
                let (winners, nums_you_have) = line.split_once('|').unwrap();
                let winners: Vec<u32> = winners
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                let nums_you_have: Vec<u32> = nums_you_have
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                Card::new(id.trim().parse().unwrap(), winners, nums_you_have)
            })
            .collect();
        let mut copies = cards.clone();
        Card::make_copies_from_winners(&cards, &mut copies);
        let mut hs = HashMap::new();
        Card::count_instances(&copies, &mut hs);
        //dbg!(&hs);
        hs.into_iter().map(|(_,v)|v).sum::<usize>() as i32
    }

    fn year_and_day(&self) -> (usize, usize) {
        (20234, 4)
    }

    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
