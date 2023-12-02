
use std::vec;

fn main() {
    let input = include_str!("../input");
    let val = process_part_2(input);
    let answer: i32 = val.into_iter().sum();
    println!("{answer}");
}

/*
--- Day 1: Trebuchet?! ---
You try to ask why they can't just use a weather machine ("not powerful enough") and
where they're even sending you ("the sky") and why your map looks mostly blank ("you
sure ask a lot of questions") and hang on did you just say the sky ("of course, where
do you think snow comes from") when you realize that the Elves are already loading
you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document
(your puzzle input) has been amended by a very young Elf who was apparently just excited
to show off her art skills. Consequently, the Elves are having trouble reading the
values on the document.

The newly-improved calibration document consists of lines of text; each line originally
contained a specific calibration value that the Elves now need to recover. On each line,
the calibration value can be found by combining the first digit and the last digit
(in that order) to form a single two-digit number.
*/
fn process_part_1(input: &str) -> Vec<i32> {
    let mut res = vec![];
    res = input.split_whitespace().collect();
    let mut nums = vec![];
    for s in res {
        let n: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
        nums.push(n);
    }
    //dbg!(nums);
    let mut coords = vec![];
    for n in nums {
        match n.len() {
            1 => coords.push(
                n[0].to_string().parse::<i32>().unwrap() * 10
                    + n[0].to_string().parse::<i32>().unwrap(),
            ),
            2 => coords.push(
                n[0].to_string().parse::<i32>().unwrap() * 10
                    + n[1].to_string().parse::<i32>().unwrap(),
            ),
            _ => coords.push(
                n[0].to_string().parse::<i32>().unwrap() * 10
                    + n[n.len() - 1].to_string().parse::<i32>().unwrap(),
            ),
        }
    }
    return coords;
}

#[test]
fn part_1_test() {
    let input = r"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    let val = process_part_1(input);

    // In this example, the calibration values of these four lines are
    // 12, 38, 15, and 77. Adding these together produces 142.
    assert_eq!(vec![12, 38, 15, 77], val);
    assert_eq!(142, val.into_iter().sum());
}

/*
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled
out with letters: one, two, three, four, five, six, seven, eight, and nine also count as
valid "digits".

Equipped with this new information, you now need to find the real first and last digit on
each line. For example:
 */
fn process_part_2(input: &str) -> Vec<i32> {
    let mut lines = vec![];
    lines = input.split_whitespace().collect();
    let mut word_nums = vec![];
    let mut coords = vec![];

    for line in lines {
        word_nums = line
            .match_indices("one")
            .map(|(i, _)| (i, 1))
            .collect::<Vec<(usize, i32)>>();
        word_nums.append(
            &mut line
                .match_indices("two")
                .map(|(i, _)| (i, 2))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("three")
                .map(|(i, _)| (i, 3))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("four")
                .map(|(i, _)| (i, 4))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("five")
                .map(|(i, _)| (i, 5))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("six")
                .map(|(i, _)| (i, 6))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("seven")
                .map(|(i, _)| (i, 7))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("eight")
                .map(|(i, _)| (i, 8))
                .collect::<Vec<(usize, i32)>>(),
        );
        word_nums.append(
            &mut line
                .match_indices("nine")
                .map(|(i, _)| (i, 9))
                .collect::<Vec<(usize, i32)>>(),
        );

        word_nums.append(
            &mut line.char_indices()
                .filter_map(|(i, c)| {
                    if c.is_ascii_digit() {
                        match c {
                            '1' => Some((i, 1)),
                            '2' => Some((i, 2)),
                            '3' => Some((i, 3)),
                            '4' => Some((i, 4)),
                            '5' => Some((i, 5)),
                            '6' => Some((i, 6)),
                            '7' => Some((i, 7)),
                            '8' => Some((i, 8)),
                            '9' => Some((i, 9)),
                            _ => panic!(),
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, i32)>>());
        word_nums.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
        //dbg!(word_nums);
        match word_nums.len() {
            1 => coords.push(word_nums[0].1 * 10 + word_nums[0].1),
            2 => coords.push(word_nums[0].1 * 10 + word_nums[1].1),
            _ => coords.push(word_nums[0].1 * 10 + word_nums[word_nums.len()-1].1),
        }
    }
    
    return coords;
    
}

#[test]
fn part_2_test() {
    let input = r"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    let val = process_part_2(input);

    assert_eq!(vec![29, 83, 13, 24, 42, 14, 76], val);
    assert_eq!(281, val.into_iter().sum());
}
