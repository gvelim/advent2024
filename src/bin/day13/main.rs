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
struct CostCalculator;

impl CostCalculator {
    fn cost(&self, target: Prize, buttons: &[Button]) -> Option<u32> {
        // substract target per button
        // if new target is (0,0) then return Some(Button.cost)
        // if new target is less than 0 retun None; path has no solution
        // recurse Min( passing (a) new target, (b) button )
        None
    }
}


#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
        assert_eq!("Button A: X+10, Y+10".parse::<Button>(), Ok(Button { dir: (10, 10), cost: 3 }));
        assert_eq!("Button A:X+10,Y+10".parse::<Button>(), Ok(Button { dir: (10, 10), cost: 3 }));
        assert!("ButtonA:X+10,Y+10".parse::<Button>().is_err());
        assert_eq!("Prize: X=8400, Y=5400".parse::<Prize>(),Ok(Prize(8400, 5400)));
        assert_eq!("Prize:X=8400,Y=5400".parse::<Prize>(),Ok(Prize(8400, 5400)));
        assert!("X=8400, Y=5400".parse::<Prize>().is_err());
    }
}
