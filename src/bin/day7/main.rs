#![feature(iter_map_windows)]
mod equation;

use std::time::Instant;
use crate::equation::Equation;

fn main() {
    let input = std::fs::read_to_string("src/bin/day7/input.txt").expect("msg");
    let equations = input.lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .collect::<Vec<_>>();

    let t = Instant::now();
    let sum = equations.iter()
        .filter_map(|eq| eq.solver(false))
        .sum::<u64>();
    println!("Part 1: total calibration result is {sum} - {:?}", t.elapsed());
    assert_eq!(12553187650171, sum);

    let t = Instant::now();
    let sum = equations.iter()
        .filter_map(|eq| eq.solver(true))
        .sum::<u64>();
    println!("Part 2: total calibration result with CompOp is {sum} - {:?}", t.elapsed());
    assert_eq!(96779702119491, sum);
}
