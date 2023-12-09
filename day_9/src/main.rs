use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day9 = Day9 {};
    //day9.run(Part::One, input);     // 1993300041 is correct
    day9.run(Part::Two, input); // 1038 correct
}

#[test]
fn test_part1() {
    let mut day9 = Day9 {};

    day9.test(
        Part::One,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        114,
    );
}

#[test]
fn test_part2() {
    let mut day9 = Day9 {};

    day9.test(
        Part::Two,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        2,
    );
}

pub struct Day9;

fn extrapolate(list: &[i32]) -> i32 {
    if list.iter().all(|v| *v == 0) {
        return 0;
    }
    let deltas = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    let diff = extrapolate(&deltas);
    return list[list.len() - 1] + diff;
}

impl Runner for Day9 {
    fn part1(&mut self, input: &str) -> i32 {
        let lines = input.lines().collect::<Vec<_>>();

        let vals = lines
            .into_iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        let mut total = 0;
        for v in &vals {
            total += extrapolate(&v);
        }
        total
    }

    fn part2(&mut self, input: &str) -> i32 {
        let lines = input.lines().collect::<Vec<_>>();

        let mut vals = lines
            .into_iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        let mut total = 0;
        for v in &mut vals {
            v.reverse();
            total += extrapolate(&v);
        }
        total
    }

    fn year_and_day(&self) -> (usize, usize) {
        (2023, 9)
    }

    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
