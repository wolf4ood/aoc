use std::{cmp::Ordering, fmt::Debug};

use aoc_attributes::aoc_main;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list0, sequence::delimited, IResult,
};

static INPUT: &str = include_str!("../../input/day13.txt");

#[aoc_main(year = 2022, day = 13, part1 = "part1", part2 = "part2")]
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
    elements_bis(input)
        .iter()
        .tuples()
        .map(|(a, b)| a.cmp(b))
        .enumerate()
        .filter(|pair| pair.1 == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn solve2(input: &str) -> usize {
    let elements = elements_bis(input);
    let f = Element::List(vec![Element::Integer(2)]);
    let s = Element::List(vec![Element::Integer(6)]);

    elements
        .iter()
        .chain(&vec![f.clone()])
        .chain(&vec![s.clone()])
        .sorted_by(|a, b| a.cmp(b))
        .enumerate()
        .filter(|e| e.1 == &f || e.1 == &s)
        .map(|e| e.0 + 1)
        .product::<usize>()
}

impl Element {
    fn cmp(&self, other: &Element) -> Ordering {
        let result = match (self, other) {
            (Element::Integer(left), Element::Integer(right)) => left.cmp(right),
            (Element::Integer(left), Element::List(right)) => {
                self.cmq_list(&vec![Element::Integer(*left)], right)
            }
            (Element::List(left), Element::Integer(right)) => {
                self.cmq_list(&left, &vec![Element::Integer(*right)])
            }
            (Element::List(left), Element::List(right)) => self.cmq_list(left, right),
        };

        result
    }

    fn cmq_list(&self, left: &Vec<Element>, right: &Vec<Element>) -> Ordering {
        let s = left.len().min(right.len());
        for i in 0..s {
            match left[i].cmp(&right[i]) {
                Ordering::Equal => {}
                n => return n,
            }
        }
        left.len().cmp(&right.len())
    }
}

fn elements_bis(input: &str) -> Vec<Element> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(idx, line)| {
            parse_list(line)
                .map(|(_, el)| Element::List(el))
                .map_err(|err| anyhow::anyhow!("{}  pair {} on line {}", err, idx, line))
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap()
}

#[derive(PartialEq, Clone)]
enum Element {
    Integer(i32),
    List(Vec<Element>),
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(arg0) => write!(f, "{}", arg0),
            Self::List(arg0) => write!(
                f,
                "[{}]",
                arg0.iter().map(|elem| format!("{:?}", elem)).join(",")
            ),
        }
    }
}

fn parse_list(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(tag("["), separated_list0(tag(","), parse_element), tag("]"))(input)
}
fn parse_element(input: &str) -> IResult<&str, Element> {
    let number = map(nom::character::complete::i32, Element::Integer);
    let list = map(parse_list, Element::List);
    nom::branch::alt((number, list))(input)
}

#[cfg(test)]
mod day13 {
    use crate::{solve1, solve2};

    static INPUT: &str = include_str!("../../input/sample13.txt");
    #[test]
    fn test_part1() {
        assert_eq!(solve1(INPUT), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve2(INPUT), 140);
    }
}
