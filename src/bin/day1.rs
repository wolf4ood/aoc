use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/2020/day1.txt");

#[aoc_main(year = 2020, day = 1, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1() -> i32 {
    combinator_sum_k(&input_generator(INPUT), 2, 2020)
        .iter()
        .fold(1, |acc, item| acc * item)
}

pub fn part2() -> i32 {
    combinator_sum_k(&input_generator(INPUT), 3, 2020)
        .iter()
        .fold(1, |acc, item| acc * item)
}
fn combinator_sum_k(input: &[i32], combinations: usize, n: i32) -> Vec<i32> {
    input
        .iter()
        .copied()
        .combinations(combinations)
        .find(|v| v.iter().sum::<i32>() == n)
        .unwrap_or_else(Vec::new)
}
