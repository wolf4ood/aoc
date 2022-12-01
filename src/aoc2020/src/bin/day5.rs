use std::collections::HashSet;

use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day5.txt");

#[aoc_main(year = 2020, day = 5, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

type SeatsBound = (i32, i32);

type Direction = (char, char);

pub fn input_generator<'a>(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}

fn find(code: &str, (lower, upper): Direction, (low, high): SeatsBound) -> i32 {
    let middle = (high + low) / 2;

    match code.chars().next() {
        Some(letter) if letter == lower => find(&code[1..], (lower, upper), (low, middle - 1)),
        Some(letter) if letter == upper => find(&code[1..], (lower, upper), (middle + 1, high)),
        Some(_) => -1,
        None => low,
    }
}

pub fn part_one() -> i32 {
    input_generator(INPUT)
        .map(|boarding| calculate_boarding_id(boarding))
        .fold(0, |acc, item| acc.max(item))
}

pub fn part_two() -> i32 {
    let (min, max, seats) = input_generator(INPUT)
        .map(|boarding| calculate_boarding_id(boarding))
        .fold(
            (i32::MAX, 0, HashSet::new()),
            |(min, max, mut seats), item| {
                seats.insert(item);
                (min.min(item), max.max(item), seats)
            },
        );

    (min..=max)
        .find(|id| !seats.contains(id))
        .unwrap_or_default()
}

fn calculate_boarding_id(boarding: &str) -> i32 {
    (find(&boarding[0..7], ('F', 'B'), (0, 127)) * 8) + find(&boarding[7..], ('L', 'R'), (0, 7))
}
