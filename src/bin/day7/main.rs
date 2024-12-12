use std::str::FromStr;
use nom::{
    bytes::complete::tag, character::complete::{space0, space1, u64}, combinator::map, multi::separated_list1, sequence::{separated_pair, tuple}, IResult, Parser
};

fn main() {
    let input = std::fs::read_to_string("src/bin/day7/input.txt").expect("msg");

    let sum = input.lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .inspect(|eq| print!("Eq:{:?} => ", eq))
        .filter(|eq| if eq.solver() != 0 { true } else { println!("Invalid"); false })
        .inspect(|eq| println!("Valid"))
        .map(|eq| eq.result)
        .sum::<u64>();

    println!("Part 1: total calibration result is {sum}");
}

#[derive(Debug)]
struct Equation {
    result: u64,
    coeff: Vec<u64>
}

impl Equation {
    fn solver(&self) -> u64 {
        let mut tmp = self.coeff.clone();
        tmp.reverse();
        Self::solve(self.result, &tmp)
    }
    fn solve(total: u64, coeff: &[u64]) -> u64 {
        // println!("{:?}",(total,&coeff));
        if coeff.len() ==1 {
            return coeff[0]
        }
        let res_1 = if total >= coeff[0] { coeff[0] + Self::solve(total - coeff[0], &coeff[1..]) } else { 0 };
        let res_2 = coeff[0] * Self::solve(total / coeff[0], &coeff[1..]);
        match (res_1 == total, res_2 == total) {
            (true, true) => res_1,
            (true, false) => res_1,
            (false, true) => res_2,
            (false, false) => 0,
        }
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_equation(s) {
            Ok(e) => Ok(e.1),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn parse_equation(s: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
        u64,
        tuple(( space0, tag(":") )),
        tuple(( space0, separated_list1(space1,u64) ))
        ),
        |(result, (_, coeff))| Equation { result, coeff }
    )(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        assert!("190: 10 19".parse::<Equation>().is_ok());
        assert!("3267: 81 40 27".parse::<Equation>().is_ok());
        assert!("83:17 5".parse::<Equation>().is_ok());
        assert!("83 :17 5".parse::<Equation>().is_ok());
        assert!("83   :    17     5".parse::<Equation>().is_ok());
        assert!("83 : ".parse::<Equation>().is_err());
        assert!("363816188802: 5 601 3 603 2 2 93 6 3 5".parse::<Equation>().is_err());
    }
}
