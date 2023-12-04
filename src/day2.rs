use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Game {
    id: u32,
    cubes: Vec<(u32, char)>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let mut id = 1u32;
    let mut out = Vec::new();
    let re = Regex::new(r"\d+ (blue|red|green)").unwrap();
    for line in input.split('\n') {
        let mut cubes = Vec::new();
        for capture in re.find_iter(line).map(|c| c.as_str()) {
            let parts = capture.split(' ').collect::<Vec<_>>();
            cubes.push((
                parts[0].parse::<u32>().unwrap(),
                parts[1].chars().next().unwrap(),
            ));
        }
        out.push(Game { id, cubes });
        id += 1;
    }
    out
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> u32 {
    let mut sum = 0u32;
    for game in input {
        let red: u32 = game.cubes.iter().filter(|c| c.1 == 'r').map(|c| c.0).max().unwrap();
        let green: u32 = game.cubes.iter().filter(|c| c.1 == 'g').map(|c| c.0).max().unwrap();
        let blue: u32 = game.cubes.iter().filter(|c| c.1 == 'b').map(|c| c.0).max().unwrap();
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += game.id;
        }
    }
    sum
}
