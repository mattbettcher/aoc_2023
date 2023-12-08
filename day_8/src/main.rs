use std::collections::HashMap;

use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day8 = Day8 {};
    //day8.run(Part::One, input);
    day8.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day8 = Day8 {};

    day8.test(
        Part::One,
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6,
    );
}

#[test]
fn test_part2() {
    let mut day8 = Day8 {};

    day8.test(Part::Two, "", 5905);
}

pub struct Day8;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn gcd_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let sec = *iter.next().unwrap();
    let mut ans = gcd(first, sec);
    while let Some(&next) = iter.next() {
        ans = gcd(ans, next);
    }
    ans
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn lcm_of(list: &[usize]) -> usize {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let sec = *iter.next().unwrap();
    let mut ans = lcm(first, sec);
    while let Some(&next) = iter.next() {
        ans = lcm(ans, next);
    }
    ans
}

impl Runner for Day8 {
    fn part1(&mut self, input: &str) -> i32 {
        let (dir, map_str) = input.split_once("\n\n").unwrap();

        let mut dirs = dir
            .chars()
            .cycle()
            .map(|c| if c == 'L' { Dir::Left } else { Dir::Right });

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        map_str
            .lines()
            .map(|l| {
                let (loc, conn) = l.split_once(" = ").unwrap();
                let (l, r) = conn[1..conn.len() - 1].split_once(", ").unwrap();
                map.insert(loc, (l, r));
            })
            .for_each(drop);

        let mut cur = "AAA";    // always start on this
        let mut moves = 0;
        loop {
            if cur == "ZZZ" {       // always end on this
                break;
            }
            if let Some(dir) = dirs.next() {
                match dir {
                    Dir::Left => {
                        (cur, _) = *map.get(cur).unwrap();
                    }
                    Dir::Right => {
                        (_, cur) = *map.get(cur).unwrap();
                    }
                }
                moves += 1;
            }
        }
        moves
    }

    fn part2(&mut self, input: &str) -> i32 {
        let (dir, map_str) = input.split_once("\n\n").unwrap();

        let mut dirs = dir
            .chars()
            .cycle()
            .map(|c| if c == 'L' { Dir::Left } else { Dir::Right });

        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        map_str
            .lines()
            .map(|l| {
                let (loc, conn) = l.split_once(" = ").unwrap();
                let (l, r) = conn[1..conn.len() - 1].split_once(", ").unwrap();
                map.insert(loc, (l, r));
            })
            .for_each(drop);

        let mut start_queue = map.clone().into_keys().into_iter().filter(|k| k.ends_with('A')).collect::<Vec<_>>();

        let mut moves_list = vec![];
        while start_queue.len() > 0 {
            let mut moves = 0;
            let mut cur = start_queue.pop().unwrap();
            loop {
                if cur.ends_with('Z') {       // always end on this
                    break;
                }
                if let Some(dir) = dirs.next() {
                    match dir {
                        Dir::Left => {
                            (cur, _) = *map.get(cur).unwrap();
                        }
                        Dir::Right => {
                            (_, cur) = *map.get(cur).unwrap();
                        }
                    }
                    moves += 1;
                }
            }
            moves_list.push(moves);
        }
        // answer is too big to fit in i32 lol
        println!("{}", lcm_of(&moves_list[..]));
        0
    }

    fn year_and_day(&self) -> (usize, usize) {
        (2023, 8)
    }

    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
