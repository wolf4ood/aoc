use std::{convert::TryFrom, str::FromStr};

use aoc_attributes::aoc_main;
use itertools::Itertools;

use anyhow::Result;
static INPUT: &str = include_str!("../../input/2020/day8.txt");

#[aoc_main(year = 2020, day = 7, part1 = "part_one", part2 = "part_two")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

type Accumulator = i32;
type InstructionPointer = usize;

#[derive(Debug)]
enum Instruction {
    Nop(usize),
    Acc(i32),
    Jump(usize),
}

type Patch = (usize, Instruction);
#[derive(Debug)]
struct ProgramState {
    pointer: InstructionPointer,
    acc: Accumulator,
    status: ProgramStatus,
}

#[derive(Debug, Clone)]
pub enum ProgramStatus {
    Running,
    Exit,
    Loop,
}

struct Program {
    instructions: Vec<Instruction>,
    state: ProgramState,
    executed_instructions: Vec<bool>,
    patch: Option<Patch>,
}

impl Program {
    fn new(ops: Vec<Instruction>) -> Self {
        Program {
            executed_instructions: vec![false; ops.len()],
            instructions: ops,
            ..Default::default()
        }
    }

    pub fn apply_patch(&mut self, patch: Patch) {
        self.patch = Some(patch);
        self.reset();
    }

    fn reset(&mut self) {
        self.state = ProgramState {
            pointer: 0,
            acc: 0,
            status: ProgramStatus::Running,
        };
        self.executed_instructions = vec![false; self.instructions.len()];
    }

    fn get_op(&self) -> Result<&Instruction, String> {
        self.patch
            .as_ref()
            .filter(|(instruction, _)| self.state.pointer == *instruction)
            .map(|(_, op)| op)
            .or_else(|| self.instructions.get(self.state.pointer))
            .ok_or_else(|| {
                String::from(format!(
                    "Instruction not found with state {:?} and patch {:?}",
                    self.state, self.patch,
                ))
            })
    }

    pub fn run(&mut self) -> Result<&ProgramState, String> {
        loop {
            let pointer = self.state.pointer;
            self.state = self.apply(self.get_op()?);
            match self.state.status {
                ProgramStatus::Exit => break,
                ProgramStatus::Loop => break,
                ProgramStatus::Running => {
                    self.executed_instructions[pointer] = true;
                }
            }
        }

        Ok(&self.state)
    }

    pub fn calculate_patches(&self) -> Vec<Patch> {
        self.instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, op)| match op {
                Instruction::Nop(nop) => Some((idx, Instruction::Jump(*nop))),
                Instruction::Acc(_) => None,
                Instruction::Jump(jump) => Some((idx, Instruction::Nop(*jump))),
            })
            .collect()
    }

    fn apply(&self, op: &Instruction) -> ProgramState {
        match op {
            Instruction::Nop(_) => ProgramState {
                acc: self.state.acc,
                pointer: self.state.pointer + 1,
                status: self.state.status.clone(),
            },
            Instruction::Acc(acc) => ProgramState {
                pointer: self.state.pointer + 1,
                acc: self.state.acc + acc,
                status: self.state.status.clone(),
            },
            Instruction::Jump(jump) => {
                let next_pos = self.state.pointer + jump;

                let status = if next_pos < self.executed_instructions.len() {
                    if self.executed_instructions[next_pos] {
                        ProgramStatus::Loop
                    } else {
                        ProgramStatus::Running
                    }
                } else {
                    ProgramStatus::Exit
                };

                ProgramState {
                    pointer: next_pos,
                    acc: self.state.acc,
                    status,
                }
            }
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Program {
            executed_instructions: vec![],
            instructions: vec![],
            state: ProgramState {
                acc: 0,
                pointer: 0,
                status: ProgramStatus::Running,
            },
            patch: None,
        }
    }
}

impl TryFrom<(&str, &str)> for Instruction {
    type Error = String;

    fn try_from((op, number): (&str, &str)) -> Result<Self, Self::Error> {
        match (op, number.parse::<i32>().map_err(|err| err.to_string())?) {
            ("acc", acc) => Ok(Instruction::Acc(acc)),
            ("jmp", jump) => Ok(Instruction::Jump(jump as usize)),
            ("nop", nop) => Ok(Instruction::Nop(nop as usize)),
            _ => Err(String::from(format!(
                "Instruction not valid {} -{}",
                op, number,
            ))),
        }
    }
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops = s
            .lines()
            .filter_map(|s| s.split_whitespace().tuples().next())
            .map(|(op, number)| Instruction::try_from((op, number)))
            .collect::<Result<Vec<Instruction>, _>>()
            .map_err(|err| err.to_string())?;

        Ok(Program::new(ops))
    }
}

pub fn part_one() -> i32 {
    Program::from_str(INPUT).unwrap().run().unwrap().acc
}

pub fn part_two() -> i32 {
    let mut program = Program::from_str(INPUT).unwrap();

    let mut acc = 0;
    for patch in program.calculate_patches() {
        program.apply_patch(patch);

        let state = program.run().unwrap();

        if let ProgramStatus::Exit = state.status {
            acc = state.acc
        }
    }

    acc
}
