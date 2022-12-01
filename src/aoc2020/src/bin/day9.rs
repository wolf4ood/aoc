use aoc_attributes::aoc_main;
use itertools::Itertools;

use anyhow::Result;
static INPUT: &str = include_str!("../../input/day9.txt");

#[aoc_main(year = 2020, day = 9, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn input_generator() -> Vec<usize> {
    INPUT
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
pub fn part_one() -> usize {
    find_invalid_number(&input_generator())
}

fn find_invalid_number(numbers: &[usize]) -> usize {
    numbers
        .windows(26)
        .find(|wnd| {
            wnd[0..25]
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != wnd[25])
        })
        .unwrap()[25]
}

pub fn part_two() -> usize {
    let input = input_generator();

    let invalid = find_invalid_number(&input);

    let (mut min, mut max) = (0, 0);
    'outer: for (i, a) in input.iter().enumerate() {
        min = *a;
        max = *a;
        let mut sum = *a;
        for (_, b) in input[i + 1..].iter().enumerate() {
            min = *b.min(&min);
            max = *b.max(&max);
            sum += b;
            if sum == invalid {
                break 'outer;
            } else if sum > invalid {
                break;
            }
        }
    }

    min + max
}
