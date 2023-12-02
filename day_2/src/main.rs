fn main() {
    let input = include_str!("../input");
    let answer = process_part_2(input);
    println!("{answer}");
}

#[derive(Debug)]
struct Game {
    pub id: i32,
    pub handfulls: Vec<Vec<Color>>,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Blue(u32),
    Green(u32),
}

fn process_part_1(input: &str) -> i32 {
    let mut games = vec![];
    let output: Vec<_> = input
        .lines()
        .map(|line| {
            let sub_line = line[5..].split_terminator(':').collect::<Vec<&str>>();
            let id: i32 = sub_line.first().unwrap().parse::<i32>().unwrap();

            let handfulls: Vec<Color> = vec![];
            let colors: Vec<Vec<Color>> = sub_line[1]
                .split_terminator(';')
                .map(|l| {
                    let mut colors = vec![];
                    l.split(',').for_each(|mut handfull| {
                        handfull = handfull.trim(); // remove leading ws
                        let handfull = handfull.split_whitespace().collect::<Vec<&str>>();
                        let count = handfull[0].parse::<u32>().unwrap();
                        let color = match handfull[1] {
                            "red" => Color::Red(count),
                            "green" => Color::Green(count),
                            "blue" => Color::Blue(count),
                            _ => panic!(),
                        };
                        colors.push(color);
                    });
                    colors
                }).collect();

            let mut game = Game {
                id,
                handfulls: colors,
            };
            games.push(game);
        })
        .collect();
    //dbg!(games);

    let mut possible_games = 0;

    for game in games {
        let mut imposible = false;
        for handfull in game.handfulls {
            for color in handfull {
                match color {
                    Color::Red(r) => if r > 12 { imposible = true },
                    Color::Green(g) => if g > 13 { imposible = true },
                    Color::Blue(b) => if b > 14 { imposible = true },
                }
            }
        }
        if !imposible {
            possible_games += game.id
        }
    }
    possible_games
}

#[test]
fn part_1_test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let val = process_part_1(input);

    assert_eq!(8, val);
}


fn process_part_2(input: &str) -> i32 {
    let mut games = vec![];
    let output: Vec<_> = input
        .lines()
        .map(|line| {
            let sub_line = line[5..].split_terminator(':').collect::<Vec<&str>>();
            let id: i32 = sub_line.first().unwrap().parse::<i32>().unwrap();

            let handfulls: Vec<Color> = vec![];
            let colors: Vec<Vec<Color>> = sub_line[1]
                .split_terminator(';')
                .map(|l| {
                    let mut colors = vec![];
                    l.split(',').for_each(|mut handfull| {
                        handfull = handfull.trim(); // remove leading ws
                        let handfull = handfull.split_whitespace().collect::<Vec<&str>>();
                        let count = handfull[0].parse::<u32>().unwrap();
                        let color = match handfull[1] {
                            "red" => Color::Red(count),
                            "green" => Color::Green(count),
                            "blue" => Color::Blue(count),
                            _ => panic!(),
                        };
                        colors.push(color);
                    });
                    colors
                }).collect();

            let mut game = Game {
                id,
                handfulls: colors,
            };
            games.push(game);
        })
        .collect();
    //dbg!(games);

    let mut min_cubes = 0;

    for game in games {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for handfull in game.handfulls {
            for color in handfull {
                match color {
                    Color::Red(r) => min_red = min_red.max(r),
                    Color::Green(g) => min_green = min_green.max(g),
                    Color::Blue(b) => min_blue = min_blue.max(b),
                }
            }
        }
        min_cubes += (min_red * min_blue * min_green) as i32;
    }
    min_cubes
}

#[test]
fn part_2_test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let val = process_part_2(input);

    assert_eq!(2286, val);
}