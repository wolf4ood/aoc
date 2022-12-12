use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use aoc_attributes::aoc_main;

static INPUT: &str = include_str!("../../input/day12.txt");

#[aoc_main(year = 2022, day = 12, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    solve1(INPUT)
}

pub fn solve1(input: &str) -> usize {
    let maze = maze(input);
    let (point, _) = maze.square(Square::S).next().unwrap();
    maze.run(point).unwrap().distance
}
pub fn part2() -> usize {
    solve2(INPUT)
}
pub fn solve2(input: &str) -> usize {
    let maze = maze(input);
    let squares = vec![Square::S, Square::X('a')];

    squares
        .into_iter()
        .flat_map(|square| maze.square(square))
        .filter_map(|(start, _)| maze.run(start))
        .map(|runner| runner.distance)
        .min()
        .unwrap_or_default()
}

#[derive(Debug)]
pub enum Direction {
    H,
    J,
    K,
    L,
}

pub struct Maze(Vec<Vec<Square>>);

impl Maze {
    fn square(&self, square: Square) -> impl Iterator<Item = (Point, &Square)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_idx, square)| (Point::new(row_idx, col_idx), square))
            })
            .filter(move |(_, s)| *s == &square)
    }

    fn at(&self, point: &Point) -> &Square {
        &self.0[point.row][point.col]
    }

    fn run(&self, start: Point) -> Option<Runner> {
        let mut stack = VecDeque::new();
        let directions = vec![Direction::L, Direction::K, Direction::J, Direction::H];

        let mut visited = HashSet::new();
        visited.insert(start.clone());
        stack.push_back(Runner::with_point(start, 0));

        while let Some(current) = stack.pop_front() {
            let current_square = self.at(&current.point);

            if current_square == &Square::E {
                return Some(current);
            }

            for d in &directions {
                if let Some(next) = current.next(d, self.0.len(), self.0[0].len()) {
                    let to_square = self.at(&next.point);
                    if visited.contains(next.point()) {
                        continue;
                    }
                    if to_square.gap(&current_square) <= 1 {
                        visited.insert(next.point().clone());
                        stack.push_back(next);
                    }
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Runner {
    point: Point,
    distance: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl Runner {
    pub fn new(row: usize, col: usize, distance: usize) -> Self {
        Self::with_point(Point::new(row, col), distance)
    }
    pub fn with_point(point: Point, distance: usize) -> Self {
        Self { point, distance }
    }

    fn next(&self, direction: &Direction, height: usize, width: usize) -> Option<Runner> {
        let row_range = 0..height;
        let col_range = 0..width;
        let distance = self.distance + 1;
        let runner = match direction {
            Direction::H => Runner::new(self.point.row, self.point.col.checked_sub(1)?, distance),
            Direction::J => Runner::new(self.point.row + 1, self.point.col, distance),
            Direction::K => Runner::new(self.point.row.checked_sub(1)?, self.point.col, distance),
            Direction::L => Runner::new(self.point.row, self.point.col + 1, distance),
        };

        if row_range.contains(&runner.point.row) && col_range.contains(&runner.point.col) {
            Some(runner)
        } else {
            None
        }
    }

    pub fn col(&self) -> usize {
        self.point.col()
    }
    pub fn row(&self) -> usize {
        self.point.row()
    }

    pub fn point(&self) -> &Point {
        &self.point
    }
}

#[derive(Debug, PartialEq)]
pub enum Square {
    S,
    E,
    X(char),
}

impl Square {
    fn gap(&self, other: &Square) -> i32 {
        self.elevation() - other.elevation()
    }

    fn elevation(&self) -> i32 {
        let c = match self {
            Square::S => 'a',
            Square::E => 'z',
            Square::X(x) => *x,
        };

        c as i32
    }
}

fn maze(input: &str) -> Maze {
    Maze(
        input
            .lines()
            .map(|line| line.chars().map(Square::from).collect())
            .collect(),
    )
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            'S' => Square::S,
            'E' => Square::E,
            _ => Square::X(c),
        }
    }
}

#[cfg(test)]
mod day12 {
    use crate::{solve1, solve2};

    static INPUT: &str = include_str!("../../input/sample12.txt");
    #[test]
    fn test_part1() {
        assert_eq!(31, solve1(INPUT));
    }
    #[test]
    fn test_part2() {
        assert_eq!(29, solve2(INPUT));
    }
}
