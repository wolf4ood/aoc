use std::collections::{HashMap, HashSet};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day3.txt");

#[aoc_main(year = 2022, day = 1, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            (
                left.chars().collect::<HashSet<char>>(),
                right.chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(left, right)| left.into_iter().filter(move |char| right.contains(char)))
        .map(|chars| chars.map(point).sum::<u32>())
        .sum()
}

pub fn point(c: char) -> u32 {
    let diff = if c.is_uppercase() { 38 } else { 96 };
    let point = c as u32 - diff;
    point
}

pub fn part2() -> u32 {
    INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .fold(HashMap::new(), |mut acc, item| {
                    item.chars().unique().for_each(|c| {
                        let entry = acc.entry(c).or_insert(0);
                        *entry = *entry + 1;
                    });
                    acc
                })
                .iter()
                .filter_map(|(k, v)| if *v == 3 { Some(point(*k)) } else { None })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {}
