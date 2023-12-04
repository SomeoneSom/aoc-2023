use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0u32;
    for line in input.split('\n') {
        let chars = line.chars();
        let mut digits = chars.filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        sum += first * 10;
        sum += digits.last().unwrap_or(first);
    }
    sum
}

pub fn to_digit(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse::<u32>().unwrap(),
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0u32;
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in input.split('\n') {
        let mut captures = re.find_iter(line).map(|c| (c.as_str(), c.end() - 1));
        let first = captures.next().unwrap();
        let mut last = captures.last().unwrap_or(first);
        // overlap check
        let after_last = &line[last.1..];
        let captures = re.find_iter(after_last).map(|c| (c.as_str(), 0));
        last = captures.last().unwrap_or(last);
        sum += to_digit(first.0) * 10;
        sum += to_digit(last.0);
    }
    sum
}
