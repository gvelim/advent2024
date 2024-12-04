use std::str::FromStr;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    IResult,
    sequence::{delimited, separated_pair},
    multi::many_till
};

fn main() {
    let i = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
    let pgm = i.parse::<Program>().unwrap();

    println!("part1: {}",
        pgm.instructions
            .iter()
            .map(|i| match i { &Instruction::Mul(x, y) => x*y })
            .sum::<u32>()
    );

}

enum Instruction {
    Mul(u32,u32)
}

struct Program {
    instructions: Vec<Instruction>
}
impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        while let Ok((remaining, (_,(x,y)))) = many_till(anychar,parse_mul)(s) {
            instructions.push(Instruction::Mul(x,y));
            s = remaining;
        }

        Ok(Self { instructions })
    }
}

fn parse_mul(i: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
        ),
        tag(")"),
    )(i)
}