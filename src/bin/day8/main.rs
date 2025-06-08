mod antenna;
mod city;

use crate::city::City;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("src/bin/day8/input.txt").unwrap();
    let city = input.parse::<City>().expect("Failed to parse City");

    let t = Instant::now();
    let count = city.antinodes(1..=1).unique().count();
    println!(
        "Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}",
        count,
        t.elapsed()
    );
    assert_eq!(247, count);

    let t = Instant::now();
    let count = city.antinodes(0..=100).unique().count();
    println!(
        "Part 2: {:?} unique locations contain an antinode given the effects of resonant harmonics - {:?}",
        count,
        t.elapsed()
    );
    assert_eq!(861, count);
}
