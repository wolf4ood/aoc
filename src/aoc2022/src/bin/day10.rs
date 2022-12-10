use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    marker::PhantomData,
    str::FromStr,
};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day10.txt");

#[aoc_main(year = 2022, day = 10, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub struct Cpu<T> {
    register: Register,
    ops: VecDeque<Instruction>,
    cycle: i32,
    data: PhantomData<T>,
}
impl<T> Debug for Cpu<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("register", &self.register)
            .field("ops", &self.ops)
            .field("cycle", &self.cycle)
            .finish()
    }
}

pub struct Crt(Vec<char>);

impl Debug for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CRT")?;
        for mut chunk in &self.0.iter().chunks(40) {
            writeln!(f, "{}", chunk.join(""))?;
        }
        Ok(())
    }
}

impl Crt {
    pub fn from_instructions(map: BTreeMap<i32, i32>) -> Self {
        let mut vec = vec!['.'; map.len() - 1];
        for rows in &map.iter().filter(|(k, _)| *k < &240).chunks(40) {
            let mut col = 0;
            for (k, v) in rows {
                let idx = k - 1;
                let range = v - 1..=v + 1;
                vec[idx as usize] = range.contains(&col).then_some('#').unwrap_or('.');
                col += 1;
            }
        }

        Crt(vec)
    }
}

impl<T: Scheduler> Cpu<T> {
    pub fn new() -> Self {
        Self {
            register: Register(1),
            ops: VecDeque::new(),
            cycle: 1,
            data: PhantomData,
        }
    }

    pub fn load(&mut self, op: Instruction) {
        self.schedule(op);
    }
    pub fn run(&mut self) -> BTreeMap<i32, i32> {
        let mut map = BTreeMap::new();
        map.insert(1, 1);
        while let Some(i) = self.ops.pop_front() {
            i.apply(&mut self.register);
            self.cycle += 1;
            map.insert(self.cycle, self.register.0);
        }
        map
    }

    fn schedule(&mut self, op: Instruction) {
        for op in T::schedule(op) {
            self.ops.push_back(op);
        }
    }
}

pub trait Scheduler {
    fn schedule(op: Instruction) -> Vec<Instruction>;
}

pub struct Part1Scheduler {}

impl Scheduler for Part1Scheduler {
    fn schedule(op: Instruction) -> Vec<Instruction> {
        match op {
            Instruction::Noop => vec![op],
            Instruction::Add(_) => vec![Instruction::Noop, op],
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [_] => Ok(Instruction::Noop),
            [_, val] => Ok(Instruction::Add(val.parse()?)),
            _ => anyhow::bail!("Failed to parse"),
        }
    }
}

fn solve1(input: &str) -> i32 {
    let mut cpu: Cpu<Part1Scheduler> = Cpu::new();

    let filter = vec![20, 60, 100, 140, 180, 220];
    for i in instructions(input) {
        cpu.load(i);
    }

    let map = cpu.run();

    map.iter()
        .filter(|(k, _)| filter.contains(k))
        .map(|(k, v)| (*k as i32) * v)
        .sum()
}

pub fn part1() -> i32 {
    solve1(INPUT)
}
fn solve2(input: &str) -> Crt {
    let mut cpu: Cpu<Part1Scheduler> = Cpu::new();
    for i in instructions(input) {
        cpu.load(i);
    }
    let map = cpu.run();

    Crt::from_instructions(map)
}
pub fn part2() -> Crt {
    solve2(INPUT)
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .map(Instruction::from_str)
        .filter_map(Result::ok)
}

#[derive(Debug)]
pub struct Register(i32);

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn apply(self, register: &mut Register) {
        match self {
            Instruction::Noop => {}
            Instruction::Add(v) => register.0 += v,
        }
    }
}

#[cfg(test)]
mod day10 {
    use crate::{solve1, solve2};

    static INPUT: &str = include_str!("../../input/sample10.txt");
    #[test]
    fn test_part1() {
        assert_eq!(solve1(INPUT), 13140);
    }
    #[test]
    fn test_part2() {
        println!("{:?}", solve2(INPUT));
    }
}
