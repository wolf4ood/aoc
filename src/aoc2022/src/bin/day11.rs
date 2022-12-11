use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use anyhow::Context;
use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day11.txt");

#[aoc_main(year = 2022, day = 11, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

type Cooler = Box<dyn Fn(Item) -> Item>;
type CoolerFactory = Box<dyn Fn(&Vec<Rc<RefCell<Monkey>>>) -> Cooler>;

pub type Item = i64;
pub type MonkeyN = usize;

pub struct Game {
    rounds: usize,
    monkeys: Vec<Rc<RefCell<Monkey>>>,
    cooler: Cooler,
}

impl Game {
    pub fn play(&mut self) {
        for _ in 0..self.rounds {
            self.round();
        }
    }

    fn round(&mut self) {
        for monkey in &self.monkeys {
            while let Some((idx, item)) = monkey.borrow_mut().inspect(&self.cooler) {
                self.monkeys[idx].borrow_mut().add_item(item);
            }
        }
    }

    fn monkeys(&self) -> impl Iterator<Item = Ref<'_, Monkey>> {
        self.monkeys.iter().map(|m| m.borrow())
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<Item>,
    op: Op,
    cond: Cond,
    inspections: Item,
}

impl Monkey {
    pub fn new(op: Op, cond: Cond) -> Self {
        Monkey {
            items: VecDeque::new(),
            op,
            cond,
            inspections: 0,
        }
    }
    fn inspect(&mut self, cooler: &dyn Fn(Item) -> Item) -> Option<(MonkeyN, Item)> {
        if let Some(next) = self.items.pop_front() {
            self.inspections += 1;
            let item = cooler(self.op.apply(next));
            Some((self.cond.apply(item), item))
        } else {
            None
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn inspections(&self) -> Item {
        self.inspections
    }
}

#[derive(Debug)]
pub struct Cond {
    test: Test,
    t: MonkeyN,
    f: MonkeyN,
}

impl Cond {
    pub fn new(test: Test, t: MonkeyN, f: MonkeyN) -> Self {
        Self { test, t, f }
    }

    fn apply(&self, item: Item) -> MonkeyN {
        if self.test.test(item) {
            self.t
        } else {
            self.f
        }
    }
}
#[derive(Debug)]
pub enum Op {
    Plus(Item),
    Mul(Item),
    MulSelf,
    AddSelf,
}

impl Op {
    fn apply(&self, item: Item) -> Item {
        match self {
            Op::Plus(val) => val + item,
            Op::Mul(val) => val * item,
            Op::MulSelf => item * item,
            Op::AddSelf => item + item,
        }
    }
}
#[derive(Debug)]
pub enum Test {
    Divisible(Item),
}

impl Test {
    fn test(&self, item: Item) -> bool {
        match self {
            Test::Divisible(val) => item % val == 0,
        }
    }

    fn item(&self) -> Item {
        match self {
            Test::Divisible(i) => *i,
        }
    }
}

pub fn part1() -> Item {
    solve1(INPUT)
}

pub fn solve1(input: &str) -> Item {
    solve(input, 20, Box::new(|_| Box::new(|val| val / 3)))
}

pub fn solve(input: &str, rounds: usize, cooler: CoolerFactory) -> Item {
    let mut game = Game::from_str(input, rounds, cooler).unwrap();

    game.play();

    game.monkeys()
        .map(|m| m.inspections())
        .sorted_by(|first, second| second.cmp(first))
        .take(2)
        .fold(1, |acc, item| acc * item)
}

pub fn solve2(input: &str) -> Item {
    solve(
        input,
        10000,
        Box::new(|g| {
            let factors = g
                .iter()
                .map(|m| m.borrow().cond.test.item())
                .product::<Item>();
            Box::new(move |val| val % factors)
        }),
    )
}
pub fn part2() -> Item {
    solve2(INPUT)
}

impl Monkey {
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut tuples = s.split("\n  ").tuples();

        let mut m = Monkey::new(Op::Mul(0), Cond::new(Test::Divisible(0), 0, 0));

        if let Some((_, items, op)) = tuples.next() {
            let parsed = parse_items(items).context(format!("Parsing item {}", items))?;
            let op = parse_op(op).context(format!("Parsing op {}", op))?;
            m.op = op;
            for p in parsed {
                m.add_item(p);
            }
        }
        if let Some((test, if_true, if_false)) = tuples.next() {
            let (test, t, f) = parse_cond(test, if_true, if_false).context(format!(
                "Parsing cond ({}) ({}) ({})",
                test, if_true, if_false
            ))?;

            m.cond = Cond::new(test, t, f);
        }

        Ok(m)
    }
}

fn parse_cond(test: &str, if_true: &str, if_false: &str) -> anyhow::Result<(Test, usize, usize)> {
    let n = test
        .to_string()
        .replace("Test: divisible by", "")
        .trim()
        .parse()?;
    let test = Test::Divisible(n);
    let t = if_true
        .to_string()
        .replace("If true: throw to monkey ", "")
        .trim()
        .parse()?;
    let f = if_false
        .to_string()
        .trim()
        .replace("If false: throw to monkey ", "")
        .parse()?;
    Ok((test, t, f))
}

fn parse_op(op: &str) -> anyhow::Result<Op> {
    let op = match op
        .to_string()
        .replace("Operation: new = ", "")
        .split_whitespace()
        .tuples()
        .next()
    {
        Some((_, "+", "old")) => Op::AddSelf,
        Some((_, "*", "old")) => Op::MulSelf,
        Some((_, "+", right)) => Op::Plus(right.parse()?),
        Some((_, "*", right)) => Op::Mul(right.parse()?),
        _ => anyhow::bail!("Failed to parse"),
    };
    Ok(op)
}

fn parse_items(items: &str) -> anyhow::Result<Vec<Item>> {
    let parsed = items
        .to_string()
        .replace("Starting items:", "")
        .trim()
        .split(",")
        .map(|s| s.trim())
        .map(|n| n.parse::<Item>().map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(parsed)
}

impl Game {
    fn from_str(s: &str, rounds: usize, cooler: CoolerFactory) -> anyhow::Result<Self> {
        let monkeys = s
            .split("\n\n")
            .map(|s| Monkey::from_str(s))
            .map(|m| m.map(|e| Rc::new(RefCell::new(e))))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let cooler = cooler(&monkeys);
        Ok(Game {
            rounds,
            monkeys,
            cooler,
        })
    }
}

#[cfg(test)]
mod day11 {
    use crate::{solve1, solve2};

    static INPUT: &str = include_str!("../../input/sample11.txt");
    #[test]
    fn test_part1() {
        assert_eq!(10605, solve1(INPUT));
    }
    #[test]
    fn test_part2() {
        assert_eq!(2713310158_i64, solve2(INPUT));
    }
}
