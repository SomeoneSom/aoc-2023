use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let numbers = input.split(':').collect::<Vec<_>>()[1];
        let parts = numbers.split('|').collect::<Vec<_>>();
        let winning = re.find_iter(parts[0]).map(|c| c.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();
        let numbers = re.find_iter(parts[1]).map(|c| c.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();
        cards.push(Card {
            winning,
            numbers,
        });
    }
    cards
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Card>) -> u32 {
    let mut sum = 0u32;
    todo!();
    sum
}
