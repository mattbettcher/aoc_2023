
pub enum Part {
    One,
    Two,
}

pub trait Runner {
    fn year_and_day(&self) -> (usize, usize);
    fn part1_description(&self) -> &str;
    fn part2_description(&self) -> &str;

    fn part1(&mut self, input: &str) -> i32;
    fn part2(&mut self, input: &str) -> i32;

    fn part1_optimized(&mut self, _input: &str) -> i32 { todo!() }
    fn part2_optimized(&mut self, _input: &str) -> i32 { todo!() }
    
    fn test(&mut self, part: Part, test_input: &str, answer: i32) {
        let (res, part_str) = match part {
            Part::One => (self.part1(test_input), "Part 1"),
            Part::Two => (self.part2(test_input), "Part 2"),
        };
        if res == answer {
            println!("{part_str} --- Correct answer found: {answer}");
        } else {
            println!("{part_str} --- Correct answer is: {answer}, found {res}");
        }
    }

    fn run(&mut self, part: Part, input: &str) -> i32 {
        let (res, part_str) = match part {
            Part::One => (self.part1(input), "Part 1"),
            Part::Two => (self.part2(input), "Part 2"),
        };
        println!("{part_str} --- Answer found: {res}");
        res
    }

    fn run_both_parts(&mut self, input: &str) -> (i32, i32) {
        let (part1_res, part2_res) = (self.part1(input), self.part2(input));

        println!("Part 1 --- Answer found: {part1_res}");
        println!("Part 2 --- Answer found: {part2_res}");

        (part1_res, part2_res)
    }
}