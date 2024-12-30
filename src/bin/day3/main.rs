use nom::branch::alt;
use nom::combinator::map;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::value,
    multi::many_till,
    sequence::{delimited, separated_pair},
};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
    let pgm = input
        .parse::<Program>()
        .map_err(|e| panic!("{e:?}"))
        .unwrap();

    let t = Instant::now();
    let sum = Cpu::use_simple_instructions().run(&pgm);
    println!("part1: {} - {:?}", sum, t.elapsed());
    assert_eq!(185797128, sum);

    let t = Instant::now();
    let sum = Cpu::use_enhanced_instructions().run(&pgm);
    println!("part1: {} - {:?}", sum, t.elapsed());
    assert_eq!(89798695, sum)
}

#[derive(Debug)]
struct Cpu {
    run_state: bool,
    use_enhanced: bool,
}

impl Cpu {
    fn use_simple_instructions() -> Cpu {
        Cpu {
            run_state: true,
            use_enhanced: false,
        }
    }
    fn use_enhanced_instructions() -> Cpu {
        Cpu {
            run_state: true,
            use_enhanced: true,
        }
    }
    fn run(&mut self, pgm: &Program) -> u32 {
        pgm.instructions
            .iter()
            .filter_map(|&i| self.run_instruction(i))
            .sum::<u32>()
    }
    fn run_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        match instruction {
            Instruction::MUL(x, y) if self.run_state => Some(x * y),
            Instruction::DONT if self.use_enhanced => {
                self.run_state = false;
                None
            }
            Instruction::DO if self.use_enhanced => {
                self.run_state = true;
                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    MUL(u32, u32),
    DONT,
    DO,
}

struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        while let Ok((remaining, (_, instruction))) =
            many_till(anychar, alt((parse_mul, parse_do, parse_dont)))(s)
        {
            instructions.push(instruction);
            s = remaining;
        }

        Ok(Self { instructions })
    }
}

fn parse_mul(i: &str) -> IResult<&str, Instruction> {
    delimited(
        tag("mul("),
        map(
            separated_pair(
                nom::character::complete::u32,
                char(','),
                nom::character::complete::u32,
            ),
            |(x, y)| Instruction::MUL(x, y),
        ),
        tag(")"),
    )(i)
}

fn parse_do(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DO, tag("do()"))(i)
}

fn parse_dont(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DONT, tag("don't()"))(i)
}
