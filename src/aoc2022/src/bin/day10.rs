use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    str::FromStr,
};

use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day10.txt");

#[aoc_main(year = 2022, day = 10, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub struct Cpu {
    register: Register,
    ops: VecDeque<Instruction>,
    cycle: i32,
    scheduler: Part1Scheduler,
}
impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("register", &self.register)
            .field("ops", &self.ops)
            .field("cycle", &self.cycle)
            .finish()
    }
}

pub struct Crt([[char; 40]; 6]);

impl Crt {
    fn new() -> Self {
        Crt([['.'; 40]; 6])
    }

    pub fn visible(&mut self, x: usize, y: usize) {
        self.0[x][y] = '#';
    }
}

impl Debug for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CRT")?;
        for chunk in self.0.iter() {
            writeln!(f, "{}", chunk.iter().join(""))?;
        }
        Ok(())
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register(1),
            ops: VecDeque::new(),
            cycle: 1,
            scheduler: Part1Scheduler {},
        }
    }

    pub fn load(&mut self, op: Instruction) {
        self.schedule(op);
    }

    pub fn exec(
        &mut self,
        mut instructions: impl Iterator<Item = Instruction>,
        mut cycle_hook: impl FnMut(i32, &Register),
    ) {
        {
            while let Some(i) = instructions.next() {
                self.schedule(i);

                self.run_instructions(&mut cycle_hook);
            }
        }
    }
    fn run_instructions(&mut self, cycle_hook: &mut impl FnMut(i32, &Register)) {
        while let Some(i) = self.ops.pop_front() {
            cycle_hook(self.cycle, &self.register);
            i.apply(&mut self.register);
            self.cycle += 1;
        }
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
        for op in self.scheduler.schedule(op) {
            self.ops.push_back(op);
        }
    }
}

pub struct Part1Scheduler {}

impl Part1Scheduler {
    fn schedule(&self, op: Instruction) -> Vec<Instruction> {
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

pub fn part1() -> i32 {
    solve1(INPUT)
}

fn solve1(input: &str) -> i32 {
    let mut cpu = Cpu::new();

    let mut signal = 0;
    cpu.exec(instructions(input), |cycle, register| {
        if cycle % 40 == 20 && cycle <= 220 {
            signal += cycle * register.0;
        }
    });

    signal
}

fn solve2(input: &str) -> Crt {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    cpu.exec(instructions(input), |cycle, register| {
        let range = register.0 - 1..=register.0 + 1;
        let r = (cycle - 1) / 40;
        let c = (cycle - 1) % 40;
        if range.contains(&c) {
            crt.visible(r as usize, c as usize);
        }
    });

    crt
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
