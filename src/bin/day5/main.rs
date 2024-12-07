mod update;
mod order;

use std::{rc::Rc, time::Instant};
use order::OrderRules;
use update::ManualUpdates;

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("msg");
    let mut s = input.split("\n\n");

    let rules = s.next().unwrap()
        .parse::<OrderRules>()
        .unwrap();
    let manual_updates = s.next().unwrap()
        .lines()
        .map(|line| line.parse::<ManualUpdates>().unwrap())
        .collect::<Rc<[_]>>();

    let is_valid_order = ManualUpdates::make_validator(&rules);
    let t = Instant::now();
    let score = manual_updates
        .iter()
        .filter(|&update| is_valid_order(update))
        .map(|update| update.middle())
        .sum::<usize>();
    println!("Part 1: valid updates score: {score} - {:?}",t.elapsed());
    assert_eq!(6949,score);

    let t = Instant::now();
    let score = manual_updates
        .iter()
        .filter(|update| !is_valid_order(update))
        .map(ManualUpdates::sort_update(&rules))
        .map(|update| update.middle())
        .sum::<usize>();
    println!("Part 2: Score for fixed updates : {score} - {:?}",t.elapsed());
    assert_eq!(4145,score);
}
