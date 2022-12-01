use aoc_attributes::aoc_main;
use nom::{
    character::complete::{alpha1, anychar, char, space1},
    sequence::tuple,
    IResult,
};

use aoc2020::utils::num_parser;

static INPUT: &str = include_str!("../../input/day2.txt");

#[aoc_main(year = 2020, day = 2, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn input_generator(input: &str) -> impl Iterator<Item = PwdInput> + '_ {
    input
        .lines()
        .map(parser())
        .map(|parsed| parsed.map(|(_, pwd)| pwd))
        .filter_map(Result::ok)
}

#[derive(Debug)]
pub struct PwdInput {
    low: usize,
    high: usize,
    letter: char,
    pwd: String,
}

pub fn part1() -> usize {
    input_generator(INPUT)
        .filter(|pwd| is_valid_one(pwd))
        .count()
}

pub fn is_valid_one(input: &PwdInput) -> bool {
    let count = input
        .pwd
        .chars()
        .filter(|letter| *letter == input.letter)
        .count();

    (input.low..=input.high).contains(&count)
}

pub fn part2() -> usize {
    input_generator(INPUT)
        .filter(|pwd| is_valid_two(pwd))
        .count()
}

fn is_valid_two(input: &PwdInput) -> bool {
    input
        .pwd
        .char_indices()
        .filter(|(idx, letter)| {
            (*idx == input.low - 1 || *idx == input.high - 1) && *letter == input.letter
        })
        .count()
        == 1
}

fn parser() -> impl FnMut(&str) -> IResult<&str, PwdInput> {
    move |input| {
        let (remaining, (low, _, high, _, letter, _, _, pwd)) = tuple((
            num_parser::<usize>(),
            char('-'),
            num_parser::<usize>(),
            space1,
            anychar,
            char(':'),
            space1,
            alpha1,
        ))(input)?;

        Ok((
            remaining,
            PwdInput {
                low,
                high,
                letter,
                pwd: pwd.to_string(),
            },
        ))
    }
}
