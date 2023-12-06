use aoc::{Part, Runner};

fn main() {
    let input = include_str!("../input");

    let mut day5 = Day5 {};

    day5.run(Part::Two, input);
}

#[test]
fn test_part1() {
    let mut day5 = Day5 {};

    day5.test(
        Part::One,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        35,
    );
}

#[test]
fn test_part2() {
    let mut day5 = Day5 {};

    day5.test(
        Part::Two,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        46,);
}

pub struct Day5;
#[derive(Debug, Clone)]
pub struct Mapper {
    pub dest: usize,
    pub src: usize,
    pub src_len: usize,
}

impl Mapper {
    pub fn new(dest: usize, src: usize, src_len: usize) -> Self {
        Self { dest, src, src_len }
    }

    pub fn map(&self, input: usize) -> Option<usize> {
        // if input is in range
        if input >= self.src && input < self.src + self.src_len {
            Some(self.dest + (input - self.src))
        } else {
            None
        }
    }
}

pub struct Map {
    pub maps: Vec<Mapper>,
}

impl Map {
    pub fn new(maps: &Vec<Mapper>) -> Self {
        Self {
            maps: maps.to_vec(),
        }
    }

    pub fn map(&self, input: usize) -> usize {
        for map in &self.maps {
            if let Some(output) = map.map(input) {
                return output;
            }
        }
        input
    }
}

impl Runner for Day5 {
    fn part1(&mut self, input: &str) -> i32 {
        let (seeds_str, data_str) = input.split_once('\n').unwrap();
        let seeds: Vec<usize> = seeds_str
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let data = data_str[1..] // to skip \n
            .split("\n\n")
            .map(|block| block.split('\n').skip(1).collect::<Vec<&str>>())
            .map(|n| {
                let mut outer = vec![];
                for b in n {
                    let mut inner = vec![];
                    b.split_whitespace()
                        .filter(|s| if *s != "map:" { true } else { false })
                        .for_each(|i| {
                            inner.push(i.parse::<usize>().unwrap());
                        });
                    outer.push(inner);
                }
                outer
            })
            .filter(|v| !v.is_empty())
            .collect::<Vec<Vec<Vec<_>>>>();

        //dbg!(&data);

        let mut mappers = vec![];
        for mapper in &data[0] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let seed_to_soil = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[1] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let soil_to_fertilizer = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[2] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let fertilizer_to_water = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[3] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let water_to_light = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[4] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let light_to_temperature = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[5] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let temperature_to_humidity = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[6] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let humidity_to_location = Map::new(&mappers);

        let mut locs = vec![];
        for seed in &seeds {
            let seed = seed_to_soil.map(*seed);
            let seed = soil_to_fertilizer.map(seed);
            let seed = fertilizer_to_water.map(seed);
            let seed = water_to_light.map(seed);
            let seed = light_to_temperature.map(seed);
            let seed = temperature_to_humidity.map(seed);
            let seed = humidity_to_location.map(seed);
            locs.push(seed);
        }

        locs.into_iter().min().unwrap() as i32
    }

    fn part2(&mut self, input: &str) -> i32 {
        let (seeds_str, data_str) = input.split_once('\n').unwrap();
        let seeds_iter = seeds_str
            .split_whitespace()
            .skip(1);

        let mut seeds = seeds_iter.clone()
            .zip(seeds_iter.skip(1))
            .map(|(seed, len)| (seed.parse::<usize>().unwrap(), len.parse::<usize>().unwrap()))
            .collect::<Vec<(usize, usize)>>();

        for i in (1..seeds.len()).step_by(2) {
            if i < seeds.len() {
                seeds.remove(i);
            }
        }

        let data = data_str[1..] // to skip \n
            .split("\n\n")
            .map(|block| block.split('\n').skip(1).collect::<Vec<&str>>())
            .map(|n| {
                let mut outer = vec![];
                for b in n {
                    let mut inner = vec![];
                    b.split_whitespace()
                        .filter(|s| if *s != "map:" { true } else { false })
                        .for_each(|i| {
                            inner.push(i.parse::<usize>().unwrap());
                        });
                    outer.push(inner);
                }
                outer
            })
            .filter(|v| !v.is_empty())
            .collect::<Vec<Vec<Vec<_>>>>();

        //dbg!(&data);

        let mut mappers = vec![];
        for mapper in &data[0] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let seed_to_soil = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[1] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let soil_to_fertilizer = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[2] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let fertilizer_to_water = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[3] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let water_to_light = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[4] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let light_to_temperature = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[5] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let temperature_to_humidity = Map::new(&mappers);
        mappers.clear();
        for mapper in &data[6] {
            mappers.push(Mapper::new(mapper[0], mapper[1], mapper[2]))
        }
        let humidity_to_location = Map::new(&mappers);

        let mut locs = vec![];
        let mut min_locs = vec![];
        for (seed_start, len) in &seeds {
            let seed_end = seed_start + len;
            for seed in *seed_start..seed_end {
                let seed = seed_to_soil.map(seed);
                let seed = soil_to_fertilizer.map(seed);
                let seed = fertilizer_to_water.map(seed);
                let seed = water_to_light.map(seed);
                let seed = light_to_temperature.map(seed);
                let seed = temperature_to_humidity.map(seed);
                let seed = humidity_to_location.map(seed);
                locs.push(seed);
            }
            let min_loc = *locs.iter().min().unwrap();
            min_locs.push(min_loc);
            locs.clear();
        }

        min_locs.into_iter().min().unwrap() as i32
    }

    fn year_and_day(&self) -> (usize, usize) {
        (2023, 5)
    }
    fn part1_description(&self) -> &str {
        todo!()
    }

    fn part2_description(&self) -> &str {
        todo!()
    }
}
