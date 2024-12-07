mod update;
mod order;

use std::rc::Rc;
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

    let validator = ManualUpdates::make_validator(&rules);
    let score = manual_updates
        .iter()
        .filter(|&u| validator(u))
        .map(|updates| updates.middle())
        .sum::<usize>();
    println!("Part 1: valid updates score: {score}");
    assert_eq!(6949,score);

    let score = manual_updates
        .iter()
        .filter(|u| !validator(u))
        .map(ManualUpdates::sort_update(&rules))
        .map(|u| u.middle())
        .sum::<usize>();
    println!("Part 2: Score for fixed updates : {score}");
    assert_eq!(4145,score);
}
