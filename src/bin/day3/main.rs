use nom::bytes::complete::{is_a, tag, take_until, take_while};
use nom::character::complete::{anychar, char, space0};
use nom::{IResult};
use nom::branch::alt;
use nom::bytes::streaming::take_till;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::error::Error;
use nom::multi::{fold_many0, many0, many_till};

fn parse_mul(i: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(
            delimited(space0,nom::character::complete::u32,space0),
            delimited(space0,char(','),space0),
            delimited(space0,nom::character::complete::u32,space0)
        ),
        tag(")"),
    )(i)
}

fn main() {
    let i = std::fs::read_to_string("src/bin/day3/sample.txt").unwrap();
    let mut input = i.as_str();

    while let Ok((remaining, result)) = many_till(anychar,parse_mul)(input) {
        println!("input: {:?}", result.1);
        input = remaining;
    }

}
