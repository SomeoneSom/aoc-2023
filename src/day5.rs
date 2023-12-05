use aoc_runner_derive::{aoc_generator, aoc};
use regex::Regex;

pub struct Input {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let re = Regex::new(r"\d+").unwrap();
    let parts = input.split(':').collect::<Vec<_>>();
    let seeds = re.find_iter(parts[1]).map(|c| c.as_str().parse::<i64>().unwrap()).collect();
    let mut maps = Vec::new();
    for part in parts[2..].iter() {
        let matches = re.find_iter(part).map(|c| c.as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();
        maps.push(matches.chunks(3).map(|m| 
            (m[1], m[1] + m[2] - 1, m[0] - m[1]) // start, end, offset
        ).collect());
    }
    Input {
        seeds,
        maps,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    let locations = input.seeds.iter().map(|s| {
        let mut seed = *s;
        for map in &input.maps {
            for range in map {
                if seed >= range.0 && seed <= range.1 {
                    seed += range.2;
                    break;
                }
            }
        }
        seed
    });
    locations.min().unwrap()
}
