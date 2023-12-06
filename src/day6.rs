use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Races {
    times: Vec<u32>,
    distances: Vec<u32>,
}

#[aoc_generator(day6, part1)]
pub fn input_generator1(input: &str) -> Races {
    let parts = input.split(':').collect::<Vec<_>>();
    let re = Regex::new(r"\d+").unwrap();
    let times = re
        .find_iter(parts[1])
        .map(|c| c.as_str().parse::<u32>().unwrap())
        .collect();
    let distances = re
        .find_iter(parts[2])
        .map(|c| c.as_str().parse::<u32>().unwrap())
        .collect();
    Races { times, distances }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Races) -> u32 {
    let mut product = 1;
    for (time, distance) in input.times.iter().zip(input.distances.iter()) {
        let mut factor = 0u32;
        for i in 0..=*time {
            if i * (*time - i) >= *distance {
                factor = i;
                break;
            }
        }
        product *= *time - factor * 2 + 1;
    }
    product
}

#[aoc_generator(day6, part2)]
pub fn input_generator2(input: &str) -> (u64, u64) {
    let parts = input.split(':').collect::<Vec<_>>();
    let re = Regex::new(r"\d+").unwrap();
    let time = re
        .find_iter(parts[1])
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = re
        .find_iter(parts[2])
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    (time, distance)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &(u64, u64)) -> u64 {
    let mut factor = 0;
    for i in 0..=input.0 {
        if i * (input.0 - i) >= input.1 {
            factor = i;
            break;
        }
    }
    input.0 - factor * 2 + 1
}
