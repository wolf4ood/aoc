use std::{fmt::Debug, str::FromStr};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day8.txt");

#[aoc_main(year = 2022, day = 8, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    Grid::from_str(INPUT).unwrap().visible_trees()
}
pub fn part2() -> usize {
    Grid::from_str(INPUT).unwrap().max_scenic_score()
}

pub struct Grid {
    size: usize,
    cells: Vec<Vec<Tree>>,
}

pub struct Tree {
    height: usize,
    pos: Position,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    H = 0,
    J = 1,
    K = 2,
    L = 3,
}

impl Direction {
    pub fn next_pos(&self, pos: Position) -> Position {
        match self {
            Direction::H => Position::new(pos.row, pos.col - 1),
            Direction::J => Position::new(pos.row + 1, pos.col),
            Direction::K => Position::new(pos.row - 1, pos.col),
            Direction::L => Position::new(pos.row, pos.col + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Visibility {
    Visible,
    NotVisible,
    NotComputed,
}

impl Tree {
    pub fn new(height: usize, pos: Position) -> Self {
        Self { height, pos }
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.cells {
            let row = v
                .iter()
                .map(|s| format!("{} - ({},{})", s.height(), s.pos.row, s.pos.col))
                .join(" ");
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

pub type NeighboordStat = (Visibility, usize);

#[derive(Debug, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn is_edge(&self, size: usize) -> bool {
        self.row == 0 || self.row == size - 1 || self.col == 0 || self.col == size - 1
    }
}

impl Grid {
    fn visible_trees(&self) -> usize {
        let mut visibles = 0;
        let dirs = [Direction::H, Direction::J, Direction::K, Direction::L];

        for row in &self.cells {
            for cell in row {
                if cell.pos.is_edge(self.size) {
                    visibles += 1;
                } else {
                    let height = cell.height;
                    let pos = cell.pos.clone();

                    visibles += dirs
                        .iter()
                        .map(|dir| self.max_by_direction(dir.next_pos(pos), dir))
                        .all(|max| max >= height)
                        .then_some(0)
                        .unwrap_or(1);
                }
            }
        }

        visibles
    }

    fn max_scenic_score(&self) -> usize {
        let dirs = [Direction::H, Direction::J, Direction::K, Direction::L];

        let mut max_score = 0;
        for row in &self.cells {
            for cell in row {
                if !cell.pos.is_edge(self.size) {
                    let height = cell.height;
                    let pos = cell.pos.clone();
                    let score = dirs
                        .iter()
                        .map(|dir| self.scenic_score(height, dir.next_pos(pos), dir))
                        .fold(1, |acc, item| acc * item);

                    max_score = usize::max(max_score, score);
                }
            }
        }

        max_score
    }

    fn scenic_score(&self, height: usize, mut start: Position, dir: &Direction) -> usize {
        let mut score = 0;
        loop {
            score += 1;
            let h = self.cells[start.row][start.col].height;
            if start.is_edge(self.size) || h >= height {
                break score;
            }
            start = dir.next_pos(start);
        }
    }
    fn max_by_direction(&self, mut start: Position, dir: &Direction) -> usize {
        let mut max = 0;

        loop {
            let h = self.cells[start.row][start.col].height;
            max = usize::max(max, h);
            if start.is_edge(self.size) {
                break;
            }
            start = dir.next_pos(start);
        }

        max
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cells = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        Tree::new(format!("{}", c).parse().unwrap(), Position::new(row, col))
                    })
                    .collect::<Vec<_>>()
            })
            .fold(Vec::new(), |mut acc, item| {
                acc.push(item);
                acc
            });

        let size = cells.len();

        Ok(Grid { cells, size })
    }
}

#[cfg(test)]
mod day8 {
    use std::str::FromStr;

    use crate::Grid;

    static SAMPLE: &str = include_str!("../../input/sample8.txt");

    #[test]
    fn test_part1() {
        let grid = Grid::from_str(SAMPLE).unwrap();

        assert_eq!(grid.visible_trees(), 21);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::from_str(SAMPLE).unwrap();

        assert_eq!(grid.max_scenic_score(), 8);
    }
}
