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

    let validator = make_validator(&rules);
    let score = manual_updates
        .iter()
        // .inspect(|d| print!("{:?}",d))
        .filter(|&u| validator(u))
        .map(|updates| updates.middle())
        // .inspect(|s| println!("{s}"))
        .sum::<usize>();
    println!("Part 1: valid updates score: {score}");
    assert_eq!(6949,score);

    let score = manual_updates
        .iter()
        .filter(|u| !validator(u))
        .map(sort_update(&rules))
        .map(|u| u.middle())
        .sum::<usize>();
    println!("Part 2: Score for fixed updates : {score}");
    assert_eq!(4145,score);


}

fn make_validator(rules: &OrderRules) ->  impl Fn(&ManualUpdates) -> bool {
    |updates: &ManualUpdates| {
        let tmp = sort_update(rules)(updates);

        tmp.entries()
            .zip(updates.entries())
            .all(|(a,b)| a == b)
    }
}

fn sort_update(rules: &OrderRules) ->  impl Fn(&ManualUpdates) -> ManualUpdates {
    use std::cmp;

    |updates: &ManualUpdates| {

        let mut list = updates.entries().cloned().collect::<Vec<_>>();

        list.sort_by(|&a,b| {
            rules
                .pages_to_follow(a)
                .map(|set|
                    if set.contains(b) { cmp::Ordering::Less } else { cmp::Ordering::Greater }
                )
                .unwrap_or(cmp::Ordering::Equal)
        });

        ManualUpdates { list }
    }
}
