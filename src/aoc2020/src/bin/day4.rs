use aoc2020::utils::num_parser;
use aoc_attributes::aoc_main;
use itertools::Itertools;
use std::{collections::HashMap, ops::RangeBounds};

static INPUT: &str = include_str!("../../input/day4.txt");

type Validator = dyn Fn(&str) -> bool;

const VALIDATORS: &[(&str, &Validator)] = &[
    ("byr", &|input| validate_range(input, 1920..=2002)),
    ("iyr", &|input| validate_range(input, 2010..=2020)),
    ("eyr", &|input| validate_range(input, 2020..=2030)),
    ("hgt", &|input| match num_parser::<i32>()(input) {
        Ok(("cm", 150..=193)) | Ok(("in", 59..=76)) => true,
        _ => false,
    }),
    ("hcl", &|input| {
        input
            .strip_prefix("#")
            .map(|hcl| {
                hcl.chars()
                    .all(|char| matches!(char, '0'..='9' | 'a'..='f'))
            })
            .unwrap_or_default()
    }),
    ("ecl", &|input| {
        matches!(input, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }),
    ("pid", &|input| {
        input.chars().filter(|char| char.is_ascii_digit()).count() == 9
    }),
];

#[aoc_main(year = 2020, day = 4, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn validate_range(input: &str, range: impl RangeBounds<i32>) -> bool {
    input
        .parse::<i32>()
        .map(|num| range.contains(&num))
        .unwrap_or_default()
}

pub fn input_generator<'a>(input: &'a str) -> impl Iterator<Item = HashMap<&'a str, &'a str>> {
    input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .flat_map(|p| p.split(':'))
                .tuples()
                .collect::<HashMap<_, _>>()
        })
}

pub fn part_one() -> usize {
    input_generator(INPUT)
        .filter(|passport| {
            VALIDATORS
                .iter()
                .all(|(name, _)| passport.contains_key(*name))
        })
        .count()
}

pub fn part_two() -> usize {
    input_generator(INPUT)
        .filter(|passport| {
            VALIDATORS.iter().all(|(name, validator)| {
                passport
                    .get(*name)
                    .map(|field| validator(field))
                    .unwrap_or_default()
            })
        })
        .count()
}
