use std::{ops::RangeInclusive, str::FromStr};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day4.txt");

#[aoc_main(year = 2022, day = 4, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub struct ElfSections(RangeInclusive<u32>);

impl ElfSections {
    pub fn contains(&self, other: &ElfSections) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }
    pub fn overlaps(&self, other: &ElfSections) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

impl FromStr for ElfSections {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split('-').take(2).tuples().next() {
            Ok(ElfSections(a.parse()?..=b.parse()?))
        } else {
            anyhow::bail!("Failed to parse")
        }
    }
}

pub struct ElfPair(ElfSections, ElfSections);

impl ElfPair {
    pub fn overlap(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn partial_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }
}

impl FromStr for ElfPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split(',').take(2).tuples().next() {
            Ok(ElfPair(first.parse()?, second.parse()?))
        } else {
            anyhow::bail!("Failed to parse")
        }
    }
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(ElfPair::from_str)
        .filter_map(Result::ok)
        .filter(ElfPair::overlap)
        .count()
}
pub fn part2() -> usize {
    INPUT
        .lines()
        .map(ElfPair::from_str)
        .filter_map(Result::ok)
        .filter(ElfPair::partial_overlap)
        .count()
}
