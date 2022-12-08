use std::{cell::RefCell, fmt::Debug, rc::Rc};

use aoc_attributes::aoc_main;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, space1},
    sequence::tuple,
    IResult,
};

static INPUT: &str = include_str!("../../input/day7.txt");

pub type MutableDir = Rc<RefCell<Directory>>;

#[aoc_main(year = 2022, day = 1, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> u64 {
    let root = build_fs(INPUT);
    root.sum_folder(100000)
}

pub fn part2() -> u64 {
    let root = build_fs(INPUT);
    root.min_folder_to_delete()
}

// TYPES

#[derive(Debug, PartialEq, Eq)]
pub enum TerminalLine {
    Command(Command),
    Output(Output),
}

#[derive(Debug)]
pub enum Node {
    File(File),
    Directory(Directory),
}

impl Node {
    fn size(&self) -> u64 {
        match self {
            Node::File(f) => f.size,
            Node::Directory(d) => d.size(),
        }
    }
}

pub struct Directory {
    name: String,
    childs: Vec<Node>,
}

impl Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("childs", &self.childs)
            .finish()
    }
}

impl Directory {
    pub fn root() -> Directory {
        Directory {
            name: "/".to_string(),
            childs: vec![],
        }
    }

    pub fn new(name: String) -> Directory {
        Directory {
            name,
            childs: vec![],
        }
    }

    pub fn size(&self) -> u64 {
        self.childs.iter().map(Node::size).sum()
    }
    pub fn sum_folder(&self, cap: u64) -> u64 {
        self.browse_dirs()
            .filter_map(|f| {
                let size = f.size();
                if size <= cap {
                    Some(size)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn min_folder_to_delete(&self) -> u64 {
        let space = 70000000;
        let used = self.size();
        let free = space - used;
        let min = 30000000 - free;
        self.browse_dirs()
            .filter_map(|f| {
                let size = f.size();
                if size >= min {
                    Some(size)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    pub fn create_file(&mut self, file: File) {
        self.childs.push(Node::File(file))
    }
    pub fn create_dir(&mut self, directory: Directory) {
        self.childs.push(Node::Directory(directory));
    }

    pub fn browse_dirs(&self) -> Box<dyn Iterator<Item = &Directory> + '_> {
        Box::new(
            self.childs.iter().filter_map(Self::only_folder).chain(
                self.childs
                    .iter()
                    .filter_map(Self::only_folder)
                    .flat_map(|c| c.browse_dirs()),
            ),
        )
    }

    fn only_folder(node: &Node) -> Option<&Directory> {
        match node {
            Node::File(_) => None,
            Node::Directory(dir) => Some(dir),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CD(Cd),
    LS,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cd {
    Back,
    Forward(String),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    File(File),
    Directory(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct File {
    name: String,
    size: u64,
}

pub struct FileSystemBuilder {
    stack: Vec<Directory>,
}

impl FileSystemBuilder {
    pub fn new() -> Self {
        Self {
            stack: vec![Directory::root()],
        }
    }

    pub fn root(mut self) -> Directory {
        let mut r = self.stack.pop().unwrap();

        while let Some(mut n) = self.stack.pop() {
            n.create_dir(r);
            r = n;
        }
        r
    }
    pub fn apply(&mut self, line: TerminalLine) {
        match line {
            TerminalLine::Command(Command::LS) => {}
            TerminalLine::Command(Command::CD(Cd::Back)) => {
                let elem = self.stack.pop().unwrap();
                self.stack.last_mut().unwrap().create_dir(elem);
            }
            TerminalLine::Command(Command::CD(Cd::Forward(dir))) => match dir.as_str() {
                "/" => {}
                _ => {
                    self.stack.push(Directory::new(dir));
                }
            },
            TerminalLine::Output(Output::File(f)) => {
                self.stack.last_mut().unwrap().create_file(f);
            }
            TerminalLine::Output(Output::Directory(_)) => {}
        };
    }
}

fn build_fs(input: &str) -> Directory {
    input
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .fold(FileSystemBuilder::new(), |mut acc, item| {
            acc.apply(item);
            acc
        })
        .root()
}

// PARSING
//
pub fn parse_line(input: &str) -> anyhow::Result<TerminalLine> {
    let (_, cmd) = alt((parse_command, parse_output))(input)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;

    Ok(cmd)
}

fn parse_command(input: &str) -> IResult<&str, TerminalLine> {
    tuple((tag("$"), space1, alt((parse_cd, parse_ls))))(input)
        .map(|(r, (_, _, cmd))| (r, TerminalLine::Command(cmd)))
}
fn parse_cd(input: &str) -> IResult<&str, Command> {
    tag("cd")(input).map(|(r, _)| match r.trim() {
        ".." => ("", Command::CD(Cd::Back)),
        dir => ("", Command::CD(Cd::Forward(dir.to_string()))),
    })
}
fn parse_ls(input: &str) -> IResult<&str, Command> {
    tag("ls")(input).map(|(r, _)| (r, Command::LS))
}
fn parse_output(input: &str) -> IResult<&str, TerminalLine> {
    alt((file_parser, parse_dir))(input).map(|(r, output)| (r, TerminalLine::Output(output)))
}

fn file_parser(input: &str) -> IResult<&str, Output> {
    nom::character::complete::u64(input).map(|(r, size)| {
        (
            "",
            Output::File(File {
                name: r.trim().to_string(),
                size,
            }),
        )
    })
}

fn parse_dir(input: &str) -> IResult<&str, Output> {
    tuple((tag("dir"), space1, alphanumeric1))(input)
        .map(|(r, (_, _, dir))| (r, Output::Directory(dir.to_string())))
}
#[cfg(test)]
mod day7 {
    use crate::{build_fs, parse_line, TerminalLine};

    #[test]
    fn test_command_parser() {
        let cmd = parse_line("$ cd /").unwrap();

        assert_eq!(
            TerminalLine::Command(crate::Command::CD(crate::Cd::Forward("/".to_string())),),
            cmd
        );
    }
    #[test]
    fn test_output_parser() {
        let cmd = parse_line("14848514 b.txt").unwrap();

        assert_eq!(
            TerminalLine::Output(crate::Output::File(crate::File {
                name: "b.txt".to_string(),
                size: 14848514
            }),),
            cmd
        );
    }
    #[test]
    fn test_part1() {
        let input = include_str!("../../input/sample7.txt");

        let fs = build_fs(input);

        assert_eq!(fs.sum_folder(100000), 95437);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("../../input/sample7.txt");

        let fs = build_fs(input);

        assert_eq!(fs.min_folder_to_delete(), 24933642);
    }
}
