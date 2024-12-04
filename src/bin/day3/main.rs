use std::str::FromStr;
use nom::{bytes::complete::tag, character::complete::{anychar, char}, IResult, sequence::{delimited, separated_pair}, multi::many_till, combinator::value, Parser};
use nom::branch::alt;

fn main() {
    let i = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
    let pgm = i.parse::<Program>().unwrap();

    let sum = pgm.instructions
        .iter()
        .map(|i|
            match i {
                &Instruction::Mul(x, y) => x*y,
                _ => 0
            }
        )
        .sum::<u32>();
    println!("part1: {}",sum);
    assert_eq!(185797128,sum);

    let mut run = true;
    let sum = pgm.instructions
        .iter()
        .filter(|i| {
            match i {
                Instruction::DONT => run = false,
                Instruction::DO => run = true,
                _ => ()
            }
            run
        })
        .map(|i|
            match i {
                &Instruction::Mul(x, y) => x*y,
                _ => 0
            }
        )
        .sum::<u32>();
    println!("part1: {}",sum);

}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Mul(u32,u32),
    DONT,
    DO
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>
}

impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        while let Ok((remaining, (_,instruction)))
            = many_till(
                anychar,
                alt((
                    parse_instruction,
                    parse_do,
                    parse_dont
                ))
            )(s)
        {
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