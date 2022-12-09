use std::{collections::HashSet, fmt::Debug, str::FromStr};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day9.txt");

#[aoc_main(year = 2022, day = 9, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    count_positions(INPUT, 1)
}
pub fn part2() -> usize {
    count_positions(INPUT, 9)
}

pub fn count_positions(input: &str, knots: usize) -> usize {
    moves(input)
        .fold(Bridge::new(knots), |mut acc, item| {
            acc.next(item);
            acc
        })
        .visited()
        .len()
}

fn moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().map(Move::from_str).filter_map(Result::ok)
}

pub struct Bridge {
    head: Position,
    tail: Vec<Position>,
    visited: HashSet<Position>,
}

impl Bridge {
    pub fn new(knots: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert(Position::default());
        Self {
            visited,
            head: Position::default(),
            tail: vec![Position::default(); knots],
        }
    }

    pub fn next(&mut self, movements: Move) {
        for _ in 0..movements.times {
            self.head = self.head.appy(&movements.kind);
            self.tail = self
                .tail
                .iter()
                .fold((Vec::new(), self.head.clone()), |(mut tail, prev), item| {
                    let next = item.follow(&prev);
                    tail.push(next);
                    (tail, next)
                })
                .0;

            if let Some(last) = self.tail.last() {
                self.visited.insert(last.clone());
            }
        }
    }
    pub fn visited(&self) -> &HashSet<Position> {
        &self.visited
    }
}
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:>2},{:>2})", self.row, self.col)
    }
}

struct Snake<'a>(&'a Vec<Position>);

impl<'a> Debug for Snake<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.0.iter().map(|f| format!("{:?}", f)).join(" <-");
        write!(f, "{}", line)
    }
}

impl Position {
    fn appy(&self, kind: &MoveKind) -> Self {
        match kind {
            MoveKind::H => Position {
                row: self.row,
                col: self.col - 1,
            },
            MoveKind::J => Position {
                row: self.row + 1,
                col: self.col,
            },
            MoveKind::K => Position {
                row: self.row - 1,
                col: self.col,
            },
            MoveKind::L => Position {
                row: self.row,
                col: self.col + 1,
            },
        }
    }

    fn follow(&self, other: &Position) -> Position {
        let diff_row = other.row - self.row;
        let diff_col = other.col - self.col;
        let sign = |num| match num {
            0 => 0,
            n if n > 0 => 1,
            _ => -1,
        };
        if diff_row.abs() <= 1 && diff_col.abs() <= 1 {
            return self.clone();
        } else {
            Position {
                row: self.row + sign(diff_row),
                col: self.col + sign(diff_col),
            }
        }
    }
}

pub struct Move {
    times: usize,
    kind: MoveKind,
}

#[derive(Debug)]
pub enum MoveKind {
    H,
    J,
    K,
    L,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().tuple_windows().next() {
            Some((first, second)) => Ok(Move {
                times: second.parse()?,
                kind: first.parse()?,
            }),
            _ => anyhow::bail!("Failed to parse move"),
        }
    }
}
impl FromStr for MoveKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(MoveKind::L),
            "L" => Ok(MoveKind::H),
            "U" => Ok(MoveKind::K),
            "D" => Ok(MoveKind::J),
            _ => anyhow::bail!("Failed to parse move"),
        }
    }
}

#[cfg(test)]
mod day9 {
    use crate::count_positions;

    static SAMPLE: &str = include_str!("../../input/sample9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(count_positions(SAMPLE, 1), 13);
    }

    #[test]
    fn test_part2() {
        let input = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;
        assert_eq!(count_positions(input, 9), 36);
    }
}
