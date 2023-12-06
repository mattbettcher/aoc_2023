use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day6 = Day6 {};

    day6.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day6 = Day6 {};

    day6.test(
        Part::One,
        "Time:      7  15   30
Distance:  9  40  200",
        288,
    );
}

#[test]
fn test_part2() {
    let mut day6 = Day6 {};

    day6.test(
        Part::Two,
        "Time:      7  15   30
Distance:  9  40  200",
        71503,
    );
}

pub struct Day6;

#[derive(Debug)]
pub struct Race {
    pub time: usize,
    pub dist: usize,
}

impl Race {
    pub fn calculate(&self) -> i32 {
        let mut wins = 0;
        // for every millisecond in the race the button is held
        for t in 0..=self.time {
            // calculate speed and distance
            let time_left = self.time - t;
            let dist = t * time_left;
            if dist > self.dist {
                wins += 1
            }
        }
        wins
    }
}

impl Runner for Day6 {
    fn part1(&mut self, input: &str) -> i32 {
        let data = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .skip(1)
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let mut races = vec![];
        for i in 0..data[0].len() {
            races.push(Race {
                time: data[0][i],
                dist: data[1][i],
            });
        }
        //dbg!(&races);
        races.into_iter().map(|race| race.calculate()).product()
    }

    fn part2(&mut self, input: &str) -> i32 {
        let mut time = 0;
        let mut distance = 0;
        let (time_str, distance_str) = input.split_once('\n').unwrap();
        time_str[10..].chars().map(|c|{
            if c.is_ascii_digit() {
                time = time * 10 + c.to_digit(10).unwrap() as usize;
            }
        }).for_each(drop);
        distance_str[10..].chars().map(|c|{
            if c.is_ascii_digit() {
                distance = distance * 10 + c.to_digit(10).unwrap() as usize;
            }
        }).for_each(drop);
        
        let race = Race{time, dist: distance};
        dbg!(&race);
        race.calculate()
    }

    fn year_and_day(&self) -> (usize, usize) {
        todo!()
    }

    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
