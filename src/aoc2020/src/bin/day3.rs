use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day3.txt");

#[aoc_main(year = 2020, day = 3, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    traverse(INPUT.lines(), 3, 1)
}

pub fn part2() -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (right, down)| {
            acc * traverse(INPUT.lines(), *right, *down)
        })
}

fn traverse<'a>(input: impl Iterator<Item = &'a str>, right: usize, down: usize) -> usize {
    input
        .skip(down)
        .step_by(down)
        .fold((0, right), |acc, item| {
            match item.chars().cycle().nth(acc.1) {
                Some('#') => (acc.0 + 1, acc.1 + right),
                _ => (acc.0, acc.1 + right),
            }
        })
        .0
}
