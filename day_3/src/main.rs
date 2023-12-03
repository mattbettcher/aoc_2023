use aoc::{Runner, Part};

fn main() {
    let input = include_str!("../input");

    let mut day3 = Day3 {};

    day3.run(Part::One, input);
}

#[test]
fn test_part1() {
    let mut day3 = Day3 {};

    day3.test(
        Part::One,
        "",
        0,
    );
}

#[test]
fn test_part2() {
    let mut day3 = Day3 {};

    day3.test(
        Part::Two,
        "",
        0,
    );
}

pub struct Day3;

impl Runner for Day3 {
    fn year_and_day(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn part1(&mut self, input: &str) -> i32 {
        todo!()
    }
    
    fn part2(&mut self, input: &str) -> i32 {
        todo!()
    }
    
    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}