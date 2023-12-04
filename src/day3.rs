use aoc_runner_derive::aoc;
use regex::Regex;

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let schematic = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    todo!();
}
