use std::collections::HashSet;

use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day6.txt");

#[aoc_main(year = 2020, day = 6, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn input_generator<'a>(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

pub fn part_one() -> usize {
    INPUT
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part_two() -> usize {
    INPUT
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(None, |acc, item| {
                    let current = item
                        .chars()
                        .filter(|char| !char.is_whitespace())
                        .collect::<HashSet<char>>();
                    match acc {
                        None => Some(current),
                        Some(acc) => Some(&acc & &current),
                    }
                })
                .unwrap_or_default()
                .len()
        })
        .sum()
}
