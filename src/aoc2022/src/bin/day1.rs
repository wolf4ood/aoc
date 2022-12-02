use std::collections::BTreeMap;

use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day1.txt");

#[aoc_main(year = 2022, day = 1, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub type ElfScore = BTreeMap<u32, Vec<u32>>;
pub type ElfAccumulator = (ElfScore, u32, u32);

pub fn part1() -> u32 {
    groups().iter().rev().map(|item| item.0).take(1).sum()
}
pub fn part2() -> u32 {
    groups()
        .iter()
        .rev()
        .flat_map(|item| item.1.iter().map(move |_| item.0))
        .take(3)
        .sum()
}

fn groups() -> ElfScore {
    INPUT
        .lines()
        .fold(ElfAccumulator::default(), |mut acc, item| {
            if item.is_empty() {
                let items = acc.0.entry(acc.1).or_insert_with(Vec::new);
                items.push(acc.2);
                acc.1 = 0;
                acc.2 += 1;
            } else {
                let n: u32 = item.parse().unwrap();
                acc.1 += n;
            }
            acc
        })
        .0
}
