use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Part {
    value: u32,
    coords: (i32, i32),
    length: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Part>, Vec<Vec<char>>) {
    let mut parts = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for (y, line) in input.lines().enumerate() {
        for capture in re.find_iter(line) {
            parts.push(Part {
                value: capture.as_str().parse::<u32>().unwrap(),
                coords: (capture.start() as i32, y as i32),
                length: capture.len() as u32,
            });
        }
    }
    let schematic = input.lines().map(|c| c.chars().collect()).collect();
    (parts, schematic)
}

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
pub fn solve_part1(input: &(Vec<Part>, Vec<Vec<char>>)) -> u32 {
    let (parts, schematic) = input;
    let mut sum = 0u32;
    for part in parts {
        let mut coords = part.coords;
        'partloop: for _ in 0..part.length {
            for neighbor in NEIGHBORS {
                let copy = (coords.0 + neighbor.0, coords.1 + neighbor.1);
                let line = schematic.get(copy.1 as usize);
                if line.is_none() {
                    continue;
                }
                let symbol = line.unwrap().get(copy.0 as usize);
                if symbol.is_none() {
                    continue;
                }
                match symbol.unwrap() {
                    '.' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => (),
                    _ => {
                        sum += part.value;
                        break 'partloop;
                    }
                }
            }
            coords.0 += 1;
        }
    }
    sum
}
