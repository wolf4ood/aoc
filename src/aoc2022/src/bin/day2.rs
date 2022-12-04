use std::str::FromStr;

use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day2.txt");

#[aoc_main(year = 2022, day = 2, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Strategy {
    Lose,
    Win,
    Draw,
}

impl Strategy {
    fn my_move(&self, other: &Shape) -> Shape {
        match (self, other) {
            (Strategy::Lose, Shape::Rock) => Shape::Scissors,
            (Strategy::Lose, Shape::Paper) => Shape::Rock,
            (Strategy::Lose, Shape::Scissors) => Shape::Paper,
            (Strategy::Win, Shape::Rock) => Shape::Paper,
            (Strategy::Win, Shape::Paper) => Shape::Scissors,
            (Strategy::Win, Shape::Scissors) => Shape::Rock,
            (Strategy::Draw, _) => other.clone(),
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_with(&self, other: &Shape) -> u32 {
        match (self, other) {
            (me, opponent) if me == opponent => 3,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
            _ => 0,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => anyhow::bail!("Failed to parse input"),
        }
    }
}

impl FromStr for Strategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::Lose),
            "Y" => Ok(Strategy::Draw),
            "Z" => Ok(Strategy::Win),
            _ => anyhow::bail!("Failed to parse input"),
        }
    }
}
pub fn parse_input<L, R>() -> anyhow::Result<Vec<(L, R)>>
where
    L: FromStr<Err = anyhow::Error>,
    R: FromStr<Err = anyhow::Error>,
{
    INPUT.lines().map(parse_line).collect()
}
pub fn parse_line<L, R>(line: &str) -> anyhow::Result<(L, R)>
where
    L: FromStr<Err = anyhow::Error>,
    R: FromStr<Err = anyhow::Error>,
{
    let moves: Vec<&str> = line.split_whitespace().collect();

    match moves.as_slice() {
        [opponent, me] => Ok((opponent.parse()?, me.parse()?)),
        _ => anyhow::bail!("Failed to parse input"),
    }
}
pub fn part1() -> u32 {
    parse_input()
        .unwrap()
        .into_iter()
        .map(calculate_line_score)
        .sum()
}

pub fn calculate_line_score(round: (Shape, Shape)) -> u32 {
    round.1.score() + round.1.play_with(&round.0)
}
pub fn part2() -> u32 {
    parse_input::<Shape, Strategy>()
        .unwrap()
        .into_iter()
        .map(|(opponent, strategy)| {
            let my_move = strategy.my_move(&opponent);
            (opponent, my_move)
        })
        .map(calculate_line_score)
        .sum()
}

#[cfg(test)]
mod tests {

    use crate::{calculate_line_score, Shape, Strategy};

    #[test]
    fn calculate_line_test_same() {
        let m = (Shape::Paper, Shape::Paper);

        assert_eq!(calculate_line_score(m), 5);
    }
    #[test]
    fn calculate_line_test_win() {
        let m = (Shape::Paper, Shape::Scissors);

        assert_eq!(calculate_line_score(m), 9);
    }
    #[test]
    fn calculate_line_test_with_strategy_win() {
        test_strategy(Shape::Rock, Strategy::Win, Shape::Paper, 8);
        test_strategy(Shape::Paper, Strategy::Win, Shape::Scissors, 9);
        test_strategy(Shape::Scissors, Strategy::Win, Shape::Rock, 7);
    }
    #[test]
    fn calculate_line_test_with_strategy_lose() {
        test_strategy(Shape::Rock, Strategy::Lose, Shape::Scissors, 3);
        test_strategy(Shape::Paper, Strategy::Lose, Shape::Rock, 1);
        test_strategy(Shape::Scissors, Strategy::Lose, Shape::Paper, 2);
    }

    fn test_strategy(opponent: Shape, strategy: Strategy, expected: Shape, points: u32) {
        let m = strategy.my_move(&opponent);

        assert_eq!(expected, m);
        assert_eq!(calculate_line_score((opponent, m)), points);
    }
}
