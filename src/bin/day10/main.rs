use std::{rc::Rc, str::FromStr};

use advent2024::{field::Field, location::*};

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/sample.txt").unwrap();
    let map = input.parse::<TopographicalMap>().unwrap();

    println!("{:?}", map);
}

#[derive(Debug)]
struct TopographicalMap(Field<u8>);

impl TopographicalMap {
    fn lowests(&self) -> impl Iterator<Item = Location> {
        self.0.iter()
            .enumerate()
            .filter(|&(_,s)| *s == 0)
            .map(|(idx,_)| self.0.index_to_cartesian(idx))
    }
}

impl FromStr for TopographicalMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TopographicalMap(
            s.parse::<Field<u8>>()?
        ))
    }
}
