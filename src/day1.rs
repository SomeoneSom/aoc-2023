use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0u32;
    for line in input.split('\n') {
        let chars = line.chars();
        let digits = chars.filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
        sum += digits[0] * 10;
        sum += digits.last().unwrap();
    }
    sum
}
