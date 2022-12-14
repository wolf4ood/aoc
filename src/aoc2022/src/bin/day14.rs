use std::fmt::Debug;

use aoc_attributes::aoc_main;
use itertools::Itertools;
use once_cell::sync::Lazy;

static INPUT: &str = include_str!("../../input/day14.txt");

static DIRECTIONS: Lazy<Vec<Direction>> =
    Lazy::new(|| vec![Direction::Down, Direction::DownLeft, Direction::DownRight]);

#[aoc_main(year = 2022, day = 1, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    solve1(INPUT)
}
pub fn part2() -> usize {
    solve2(INPUT)
}

fn solve1(input: &str) -> usize {
    let mut grid: Grid = parse_lines(input).into();
    let mut units = 0;

    'main: loop {
        let mut walker = GridWalker::new(&mut grid);
        'walker: loop {
            match walker.next() {
                WalkerStatus::Abiss => break 'main,
                WalkerStatus::Stucked => panic!("It should not stuck"),
                WalkerStatus::Running => {}
                WalkerStatus::Completed => break 'walker,
            }
        }
        units += 1;
    }

    units
}

fn solve2(input: &str) -> usize {
    let mut grid: Grid = parse_lines(input).into();
    grid.add_row(Cell::Air);
    grid.add_row(Cell::Rock);

    let mut units = 0;
    'main: loop {
        let mut walker = GridWalker::new(&mut grid);
        'walker: loop {
            match walker.next() {
                WalkerStatus::Abiss => panic!("It should not abiss"),
                WalkerStatus::Stucked => {
                    units += 1;
                    break 'main;
                }
                WalkerStatus::Running => {}
                WalkerStatus::Completed => {
                    break 'walker;
                }
            }
        }
        units += 1;
    }

    units
}
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub struct Line(Vec<Point>);

impl Line {
    fn max_row(&self) -> usize {
        self.0.iter().map(|p| p.row).max().unwrap_or_default()
    }
    fn max_col(&self) -> usize {
        self.0.iter().map(|p| p.col).max().unwrap_or_default()
    }
    fn min_col(&self) -> usize {
        self.0.iter().map(|p| p.col).min().unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Sand,
    Start,
    Rock,
    Air,
    Abiss,
}

pub struct Grid {
    height: usize,
    width: usize,
    start: Point,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn at(&self, point: &Point) -> &Cell {
        &self.cells[point.row][point.col]
    }
    fn set(&mut self, point: &Point, cell: Cell) {
        self.cells[point.row][point.col] = cell
    }

    fn add_row(&mut self, cell: Cell) {
        self.height += 1;
        self.cells.push(vec![cell; self.width + 1]);
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.cells {
            let line = r
                .iter()
                .map(|c| match c {
                    Cell::Sand => 'o',
                    Cell::Rock => '#',
                    Cell::Air => '.',
                    Cell::Abiss => '~',
                    Cell::Start => '+',
                })
                .join(" ");

            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl From<Vec<Line>> for Grid {
    fn from(lines: Vec<Line>) -> Self {
        let (min_col, max_row) = lines.iter().fold((usize::MAX, 0), |acc, item| {
            (acc.0.min(item.min_col()), acc.1.max(item.max_row()))
        });

        let padding = max_row * 3;

        let lines = lines
            .iter()
            .map(|line| {
                Line(
                    line.0
                        .iter()
                        .map(|point| Point {
                            row: point.row,
                            col: point.col - min_col + padding,
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        let max_col = lines.iter().map(|l| l.max_col()).max().unwrap();
        let cells = vec![vec![Cell::Air; max_col + 1 + padding]; max_row + 1];

        let start = Point {
            row: 0,
            col: 500 - min_col + padding,
        };
        let mut grid = Grid {
            height: max_row,
            width: max_col + padding,
            cells,
            start: start.clone(),
        };

        grid.set(&start, Cell::Start);

        for l in lines {
            for (p1, p2) in l.0.iter().tuple_windows() {
                grid.set(p1, Cell::Rock);
                grid.set(p2, Cell::Rock);
                if p1.col == p2.col {
                    let range = if p1.row > p2.row {
                        p2.row..p1.row
                    } else {
                        p1.row..p2.row
                    };
                    for i in range {
                        grid.set(&Point::new(i, p1.col), Cell::Rock);
                    }
                } else if p1.row == p2.row {
                    let range = if p1.col > p2.col {
                        p2.col..p1.col
                    } else {
                        p1.col..p2.col
                    };
                    for i in range {
                        grid.set(&Point::new(p1.row, i), Cell::Rock);
                    }
                }
            }
        }
        grid
    }
}

enum Direction {
    Down,
    DownLeft,
    DownRight,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn step(&self, direction: &Direction, width: usize, height: usize) -> Option<Point> {
        if self.row == height || self.col == width || self.col == 0 {
            return None;
        }
        let Point { row, col } = self;
        let next = match direction {
            Direction::Down => Point {
                row: row + 1,
                col: *col,
            },
            Direction::DownLeft => Point {
                row: row + 1,
                col: col - 1,
            },
            Direction::DownRight => Point {
                row: row + 1,
                col: col + 1,
            },
        };
        Some(next)
    }
}

#[derive(Debug)]
pub struct GridWalker<'a> {
    grid: &'a mut Grid,
    current: Point,
}

#[derive(PartialEq)]
pub enum WalkerStatus {
    Abiss,
    Stucked,
    Running,
    Completed,
}
impl<'a> GridWalker<'a> {
    pub fn new(grid: &'a mut Grid) -> Self {
        let current = grid.start.clone();
        Self { grid, current }
    }

    pub fn starting_point(&self) -> bool {
        self.current == self.grid.start
    }

    pub fn next(&mut self) -> WalkerStatus {
        for dir in DIRECTIONS.iter() {
            if let Some(next) = self.current.step(dir, self.grid.width, self.grid.height) {
                if self.grid.at(&next) == &Cell::Air {
                    if !self.starting_point() {
                        self.grid.cells[self.current.row][self.current.col] = Cell::Air;
                    }
                    self.grid.cells[next.row][next.col] = Cell::Sand;
                    self.current = next;
                    return WalkerStatus::Running;
                }
            } else {
                return WalkerStatus::Abiss;
            }
        }
        if self.starting_point() {
            self.grid.cells[self.current.row][self.current.col] = Cell::Sand;
            WalkerStatus::Stucked
        } else {
            WalkerStatus::Completed
        }
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    let lines = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();
    lines
}

fn parse_line(input: &str) -> Line {
    Line(input.split(" -> ").map(parse_point).collect())
}
fn parse_point(input: &str) -> Point {
    let (x, y) = input.split(",").tuples().next().unwrap();
    Point {
        row: y.parse().unwrap(),
        col: x.parse().unwrap(),
    }
}

#[cfg(test)]
mod day14 {
    use crate::{solve1, solve2};

    static INPUT: &str = include_str!("../../input/sample14.txt");
    #[test]
    fn test_part1() {
        assert_eq!(solve1(INPUT), 24);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve2(INPUT), 93);
    }
}
