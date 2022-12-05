use std::{collections::VecDeque, str::FromStr};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day5.txt");

#[aoc_main(year = 2022, day = 5, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[derive(Debug, Default)]
pub struct Ship(Vec<Stack>);

pub trait Mover {
    fn apply(ship: &mut Ship, movement: Move);

    fn to_move(ship: &mut Ship, movement: &Move) -> Vec<char> {
        ship.0
            .get_mut(movement.from)
            .map(|from| {
                (0..movement.quantity)
                    .filter_map(|_| from.0.pop_back())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

pub struct CrateMover9000;

impl Mover for CrateMover9000 {
    fn apply(ship: &mut Ship, movement: Move) {
        let to_move = Self::to_move(ship, &movement);
        if let Some(to) = ship.0.get_mut(movement.to) {
            for m in to_move {
                to.0.push_back(m);
            }
        }
    }
}

pub struct CrateMover9001;

impl Mover for CrateMover9001 {
    fn apply(ship: &mut Ship, movement: Move) {
        let to_move = Self::to_move(ship, &movement);
        if let Some(to) = ship.0.get_mut(movement.to) {
            for m in to_move.into_iter().rev() {
                to.0.push_back(m);
            }
        }
    }
}

impl Ship {
    pub fn add_stack(&mut self) {
        self.0.push(Stack::default())
    }

    pub fn push(&mut self, stack: usize, cargo: char) {
        self.0.get_mut(stack).unwrap().0.push_back(cargo);
    }

    pub fn apply<M: Mover>(&mut self, movement: Move) {
        M::apply(self, movement)
    }

    pub fn tops(&self) -> Vec<Option<char>> {
        self.0.iter().map(|first| first.0.back().cloned()).collect()
    }
}

#[derive(Debug, Default)]
pub struct Stack(VecDeque<char>);

impl FromStr for Ship {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines().rev().fold(Ship::default(), |mut acc, item| {
            let mut idx = 0;
            for chunk in &item.chars().chunks(4) {
                let position = chunk.filter(|c| c.is_numeric() || c.is_alphabetic()).next();
                match position {
                    Some(numeric) if numeric.is_numeric() => {
                        acc.add_stack();
                    }
                    Some(alpha) if alpha.is_alphabetic() => {
                        acc.push(idx, alpha);
                        idx += 1;
                    }
                    _ => {
                        idx += 1;
                    }
                }
            }
            acc
        }))
    }
}

#[derive(Default, Debug)]
pub struct Move {
    from: usize,
    to: usize,
    quantity: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = s
            .replace("move", "")
            .replace("from", "")
            .replace("to", "")
            .split_whitespace()
            .map(|pos| pos.parse::<usize>().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<usize>>>()?;

        match m.as_slice() {
            [quantity, from, to] => Ok(Move {
                from: *from - 1,
                to: *to - 1,
                quantity: *quantity,
            }),
            _ => anyhow::bail!("Failed to parse movement"),
        }
    }
}

pub fn part1() -> String {
    solve_part::<CrateMover9000>(INPUT)
}

fn solve_part<M: Mover>(input: &str) -> String {
    input
        .split("\n\n")
        .tuples()
        .map(|(initial, moves)| {
            Ship::from_str(initial).map(|ship| {
                moves
                    .lines()
                    .map(Move::from_str)
                    .filter_map(Result::ok)
                    .fold(ship, |mut acc, movement| {
                        acc.apply::<M>(movement);
                        acc
                    })
            })
        })
        .map(|ship| ship.map(|s| s.tops().into_iter().flatten().join("")))
        .filter_map(Result::ok)
        .join(",")
}
pub fn part2() -> String {
    solve_part::<CrateMover9001>(INPUT)
}

#[cfg(test)]
mod tests {
    use crate::{solve_part, CrateMover9000, CrateMover9001};

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/sample5.txt");

        let result = solve_part::<CrateMover9000>(input);

        assert_eq!("CMZ", result);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/sample5.txt");

        let result = solve_part::<CrateMover9001>(input);

        assert_eq!("MCD", result);
    }
}
