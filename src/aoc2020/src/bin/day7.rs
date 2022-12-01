use std::collections::{HashMap, HashSet};

use aoc2020::utils::num_parser;
use aoc_attributes::aoc_main;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space1,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

static INPUT: &str = include_str!("../../input/day7.txt");

#[aoc_main(year = 2020, day = 7, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[derive(Debug)]
pub struct Bag<'a> {
    name: &'a str,
    bags: HashMap<&'a str, i32>,
}

pub fn input_generator<'a>(input: &'a str) -> impl Iterator<Item = Bag<'a>> {
    input
        .lines()
        .map(bag_parser())
        .filter_map(Result::ok)
        .map(|(_, bag)| bag)
}

pub fn part_one() -> usize {
    let parents = input_generator(INPUT).fold(HashMap::new(), |mut acc, item| {
        for (name, _) in &item.bags {
            acc.entry(name.clone())
                .or_insert_with(|| HashSet::new())
                .insert(item.name);
        }

        acc
    });

    count_parents(&parents, "shiny gold").len()
}

fn count_parents<'a>(
    parents: &'a HashMap<&'a str, HashSet<&'a str>>,
    current: &'a str,
) -> HashSet<&'a str> {
    match parents.get(current) {
        Some(parents_for) => parents_for.iter().fold(HashSet::new(), |mut acc, item| {
            acc.extend(count_parents(parents, item));
            acc.insert(item);
            acc
        }),
        None => HashSet::new(),
    }
}

fn count_bags<'a>(bags: &'a HashMap<&'a str, Bag<'a>>, current: &'a str) -> i32 {
    match bags.get(current) {
        Some(bag) => bag.bags.iter().fold(0, |acc, (b, count)| {
            acc + count + (count * count_bags(bags, b))
        }),

        None => 0,
    }
}

pub fn part_two() -> i32 {
    let parents = input_generator(INPUT).fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.name, item);

        acc
    });

    count_bags(&parents, "shiny gold")
}

fn single_bag_parser<'a>() -> impl FnMut(&'a str) -> IResult<&str, (&str, i32)> {
    move |input| {
        let (remaining, (_, count, _, color)) =
            tuple((space1, num_parser::<i32>(), space1, take_until("bag")))(input)?;

        let bag_tag = if count > 1 { "bags" } else { "bag" };

        let (remaining, _) = tag(bag_tag)(remaining)?;

        Ok((remaining, (color.trim(), count)))
    }
}

fn bag_parser<'a>() -> impl FnMut(&'a str) -> IResult<&str, Bag<'a>> {
    move |input| {
        let (remaining, (bag, _, colors)) = tuple((
            take_until("bags"),
            tag("bags contain"),
            separated_list0(tag(","), single_bag_parser()),
        ))(input)?;

        Ok((
            remaining,
            Bag {
                name: bag.trim(),
                bags: colors.into_iter().collect(),
            },
        ))
    }
}
