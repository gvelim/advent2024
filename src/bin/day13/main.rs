use std::str::FromStr;
use nom::{
    bytes::complete::{tag, take_till},
    character::{complete::alpha1, is_digit},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Button {
    dir: (u32,u32),
    cost: u8
}

impl FromStr for Button {
    type Err = nom::Err<()>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parse_button_cost = map(
            preceded(tag("Button "), alpha1),
            |id| if id == "A" { 3 } else { 1 }
        );

        match map(
            separated_pair(parse_button_cost, tag(":"), parse_numbers_pair),
            |(cost, dir)| Button { dir, cost}
        )(input) {
            Ok((_, button)) => Ok(button),
            Err(err) => Err(err)
        }
    }
}

#[derive(Debug)]
struct Prize(u32,u32);

impl FromStr for Prize {
    type Err = nom::Err<()>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match preceded(tag("Prize:"), parse_numbers_pair)(input) {
            Ok((_, (x,y))) => Ok(Prize(x, y)),
            Err(err) => Err(err)
        }
    }
}

fn parse_numbers_pair(input: &str) -> IResult<&str, (u32,u32), ()> {
    separated_pair(
        preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32),
        tag(","),
        preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32)
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let button = "Button A: X+10, Y+10";
        let prize = "Prize: X=8400, Y=5400";

        println!("Parsed button: {:?}", button.parse::<Button>());
        println!("Parsed button: {:?}", prize.parse::<Prize>());
    }
}
