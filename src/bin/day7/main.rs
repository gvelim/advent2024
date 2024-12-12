#![feature(iter_map_windows)]

use std::str::FromStr;
use nom::{
    bytes::complete::tag, character::complete::{space0, space1, u64}, combinator::map, multi::separated_list1, sequence::{separated_pair, tuple}, IResult
};

fn main() {
    let input = std::fs::read_to_string("src/bin/day7/input.txt").expect("msg");
    let equations = input.lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .collect::<Vec<_>>();

    let sum = equations.iter()//.skip(4).take(1)
        .inspect(|eq| print!("{:15} = {:?} ",eq.result,eq.coeff))
        .filter_map(|eq| {
            let res = eq.solver();
            if res.is_none() { println!("✕"); None } else { println!("✓"); res }
        })
        .sum::<u64>();
    println!("Part 1: total calibration result is {sum}");
    // assert_eq!(12553187650171, sum);
}

#[derive(Debug)]
struct Equation {
    result: u64,
    coeff: Vec<u64>
}

impl Equation {
    fn solver(&self) -> Option<u64> {
        let mut tmp = self.coeff.clone();
        tmp.reverse();
        Self::solve(self.result, &tmp)
    }
    fn solve(total: u64, coeff: &[u64]) -> Option<u64> {
        fn ct(a:u64, b:u64) -> u64 { format!("{}{}",a,b).parse::<u64>().unwrap() }

        if coeff.len() == 1 { return Some(coeff[0]) }

        let res_1 = Self::solve(total / coeff[0], &coeff[1..]).map(|s| s * coeff[0]);
        let res_2 = if total >= coeff[0] {
            Self::solve(total - coeff[0], &coeff[1..]).map(|s| s + coeff[0])
        } else { None };
        let res_3 = if total >= coeff[0] {
            Self::solve((total - coeff[0])/10, &coeff[1..]).map(|s| ct(s,coeff[0]))
        } else { None };

        match (res_1 == Some(total), res_2 == Some(total), res_3 == Some(total)) {
            (true, _, _) => res_1,
            (_, true, _) => res_2,
            (_, _, true) => res_3,
            _ => None,
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
