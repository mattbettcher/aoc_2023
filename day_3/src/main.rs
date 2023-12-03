use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day3 = Day3 {};

    day3.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day3 = Day3 {};

    day3.test(
        Part::One,
        "............
.467..114...
....*.......
...35..633..
.......#....
.617*.......
......+.58..
...592......
.......755..
....$.*.....
..664.598...
............",
        4361,
    );
}

#[test]
fn test_part2() {
    let mut day3 = Day3 {};

    day3.test(
        Part::Two,
        "............
.467..114...
....*.......
...35..633..
.......#....
.617*.......
......+.58..
...592......
.......755..
....$.*.....
..664.598...
............",
        467835,
    );
}

pub struct Day3;

#[derive(Debug,Clone)]
pub struct Span(pub usize, pub usize, pub u32, pub u32);
#[derive(Debug)]
pub struct Gear(pub usize, pub usize, pub Vec<Span>);

impl Runner for Day3 {
    fn year_and_day(&self) -> (usize, usize) {
        (2023, 3)
    }

    fn part1(&mut self, input: &str) -> i32 {
        let w = input.lines().nth(0).and_then(|l| Some(l.len())).unwrap() + 1;
        let mut y = 0;
        let mut s = 0;
        let mut e = 0;
        let mut spans = vec![];
        input
            .lines()
            .map(|l| {
                let nums: Vec<(usize, char)> = l
                    .char_indices()
                    .filter(|(_, c)| c.is_ascii_digit())
                    .collect();
                let len = nums.len();
                let mut val = 0;
                if len > 0 {
                    // if we have numbers in this line
                    s = nums[0].0; // set first span start to first char index
                    e = nums[0].0;
                    for (i, (ci, c)) in nums.clone().into_iter().enumerate() {
                        val = val * 10 + c.to_digit(10).unwrap();
                        if i + 1 < len {
                            let (next_i, _) = nums[i + 1];
                            if next_i - 1 == ci {
                                // consecutive index == increase span
                                e = ci + 1;
                            } else {
                                // end of span
                                spans.push(Span(s, e, val, y));
                                val = 0;
                                // set start of the new span
                                s = nums[i + 1].0;
                                e = nums[i + 1].0;
                            }
                        } else {
                            spans.push(Span(s, e, val, y));
                        }
                    }
                }
                y += 1;
            })
            .for_each(drop);

        // we know the index on each line of each number and its value and what line its on
        // now we just need to check the box of chars around it for any symbols other then .
        // input must be modified to allow easy checking of numbers of edges
        let mut total = 0;
        for span in spans {
            let y1 = (span.3 - 1) as usize;
            let y2 = (span.3 + 1) as usize;
            let x1 = span.0 - 1;
            let x2 = span.1 + 1;
            'first: for y in y1..=y2 {
                for x in x1..=x2 {
                    let i = w * y + x;
                    let c = &input[i..i + 1].chars().collect::<Vec<char>>();
                    let c = c.first().unwrap();
                    if !c.is_ascii_digit() && c != &'.' {
                        total += span.2;
                        break 'first;
                    }
                }
            }
        }

        total as i32
    }

    fn part2(&mut self, input: &str) -> i32 {
        let w = input.lines().nth(0).and_then(|l| Some(l.len())).unwrap() + 1;
        let mut y = 0;
        let mut s = 0;
        let mut e = 0;
        let mut spans = vec![];
        input
            .lines()
            .map(|l| {
                let nums: Vec<(usize, char)> = l
                    .char_indices()
                    .filter(|(_, c)| c.is_ascii_digit())
                    .collect();
                let len = nums.len();
                let mut val = 0;
                if len > 0 {
                    // if we have numbers in this line
                    s = nums[0].0; // set first span start to first char index
                    e = nums[0].0;
                    for (i, (ci, c)) in nums.clone().into_iter().enumerate() {
                        val = val * 10 + c.to_digit(10).unwrap();
                        if i + 1 < len {
                            let (next_i, _) = nums[i + 1];
                            if next_i - 1 == ci {
                                // consecutive index == increase span
                                e = ci + 1;
                            } else {
                                // end of span
                                spans.push(Span(s, e, val, y));
                                val = 0;
                                // set start of the new span
                                s = nums[i + 1].0;
                                e = nums[i + 1].0;
                            }
                        } else {
                            spans.push(Span(s, e, val, y));
                        }
                    }
                }
                y += 1;
            })
            .for_each(drop);

        // we know the index on each line of each number and its value and what line its on
        // now we just need to check the box of chars around it for any symbols other then .
        // input must be modified to allow easy checking of numbers of edges
        let mut total = 0;
        let mut gear_pos: Vec<(usize, usize)> = vec![];
        let mut gears: Vec<Gear> = vec![];
        for span in &spans {
            let y1 = (span.3 - 1) as usize;
            let y2 = (span.3 + 1) as usize;
            let x1 = span.0 - 1;
            let x2 = span.1 + 1;
            'first: for y in y1..=y2 {
                for x in x1..=x2 {
                    let i = w * y + x;
                    let c = &input[i..i + 1].chars().collect::<Vec<char>>();
                    let c = c.first().unwrap();
                    if c == &'*' {
                        if !gear_pos.contains(&(x, y)) {
                            gear_pos.push((x, y));
                        }
                        break 'first;
                    }
                }
            }
        }
        //println!("Possible gears:");
        //for (x, y) in &gear_pos {
        //    println!("X:{x} Y:{y}");
        //}
        for (x, y) in &gear_pos {
            let mut gear = Gear(*x, *y, vec![]);
            for span in &spans {
                if Day3::check_span_has_gear(span, (*x, *y), w, input) {
                    gear.2.push(span.clone());
                }
            }
            if gear.2.len() > 1 {
                gears.push(gear);
            }
        }

        //println!("Gears:");
        for gear in &gears {
            //println!("X:{} Y:{} Spans:{:?}", gear.0, gear.1, gear.2);
            total += gear.2.clone().into_iter().map(|span|{span.2}).product::<u32>() as i32;
        }

        total as i32
    }

    fn part1_description(&self) -> &str {
        r#"--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?"#
    }

    fn part2_description(&self) -> &str {
        r#"--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?"#
    }
}

impl Day3 {
    pub fn check_span_has_gear(
        span: &Span,
        gear: (usize, usize),
        width: usize,
        input: &str,
    ) -> bool {
        let y1 = (span.3 - 1) as usize;
        let y2 = (span.3 + 1) as usize;
        let x1 = span.0 - 1;
        let x2 = span.1 + 1;
        for y in y1..=y2 {
            for x in x1..=x2 {
                let i = width * y + x;
                let c = &input[i..i + 1].chars().collect::<Vec<char>>();
                let c = c.first().unwrap();
                if c == &'*' {
                    if gear.0 == x && gear.1 == y {
                        return true;
                    }
                }
            }
        }
        false
    }
}
