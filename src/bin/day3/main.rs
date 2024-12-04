use std::str::FromStr;
use std::time::Instant;
use nom::{bytes::complete::tag, character::complete::{anychar, char}, IResult, sequence::{delimited, separated_pair}, multi::many_till, combinator::value, Parser};
use nom::branch::alt;

fn main() {
    let input = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
    let pgm = input.parse::<Program>().map_err(|e| panic!("{e:?}")).unwrap();
    let mut cpu = CPU::default();

    let t = Instant::now();
    let sum = pgm.instructions
        .iter()
        .filter_map(|&i| cpu.run_instruction(i))
        .sum::<u32>();
    println!("part1: {} - {:?}",sum, t.elapsed());
    assert_eq!(185797128,sum);

    let t = Instant::now();
    cpu.set_enhanced_instructions(true);
    let sum = pgm.instructions
        .iter()
        .filter_map(|&i| cpu.run_instruction(i))
        .sum::<u32>();
    println!("part1: {} - {:?}",sum, t.elapsed());
    assert_eq!(89798695,sum)
}

#[derive(Debug)]
struct CPU {
    run_state: bool,
    enhanced: bool
}

impl CPU {
    fn run_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        match instruction {
            Instruction::Mul(x, y) if self.run_state => Some(x*y),
            Instruction::DONT if self.enhanced => {self.run_state = false; None}
            Instruction::DO if self.enhanced => {self.run_state = true; None}
            _ => None
        }
    }
    fn set_enhanced_instructions(&mut self, state:bool) { self.enhanced = state }
}

impl Default for CPU {
    fn default() -> Self {
        CPU { run_state: true, enhanced: false }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Mul(u32,u32),
    DONT,
    DO
}

struct Program {
    instructions: Vec<Instruction>
}

impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        while let Ok((remaining, (_,instruction))) = many_till(anychar, alt((parse_instruction, parse_do, parse_dont)))(s) {
            instructions.push(instruction);
            s = remaining;
        }

        Ok(Self { instructions })
    }
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    delimited(
        tag("mul("),
        separated_pair(
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
        ).map(|(x,y)| Instruction::Mul(x,y)),
        tag(")"),
    )(i)
}

fn parse_do(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DO,tag("do()"))(i)
}

fn parse_dont(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DONT,tag("don't()"))(i)
}