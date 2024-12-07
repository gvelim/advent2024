use std::{rc::Rc, collections::HashMap, num::ParseIntError, str::FromStr};
use nom::{self, branch::alt, bytes::{complete::{tag, take_till}, streaming::take_while}, character::{complete::digit1, streaming::{anychar, line_ending}}, combinator::{eof, map, map_res}, multi::{many1, many_till, separated_list1}, IResult, Map, Parser};


fn parse_update(s: &str) -> IResult<&str, Rc<[usize]>> {
    map(
        separated_list1(tag(","),
            map_res(digit1, |s: &str| s.parse::<usize>())
        ), |v| v.into()
    )(s)
}

fn parse_updates(s: &str) -> IResult<&str, Rc<[Rc<[usize]>]>> {
    map(
        separated_list1(
            line_ending,
            parse_update
        ), |v| v.into()
    )(s)
}

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/sample.txt").expect("msg");
    let lists = input.split("\n\n").skip(1).next().unwrap();

    println!("{:?}",
        parse_updates(lists)
    )
}

struct OrderingRule {
    x: usize,
    y: usize
}

struct Update {
    list: HashMap<usize,usize>
}

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Update {
            list: s
                .split(',')
                .enumerate()
                .map(|(i,numeric)|
                    numeric.parse::<usize>().map(|num| (i,num))
                )
                .collect::<Result<HashMap<usize,usize>,ParseIntError>>()?
        })
    }
}

#[test]
fn test_parse_update() {
    let input = std::fs::read_to_string("str/bin/day5/sample.txt").expect("msg");
    let lists = input.split("\n\n").next().unwrap();
}
