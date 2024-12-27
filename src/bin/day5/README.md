# Day 5 Challenge - README

## Overview

The approach to solving the Day 5 challenge involves defining `OrderRules` to represent the rules for page ordering, and `ManualUpdates` to represent the updates to be validated and sorted. The provided code snippets illustrate the key steps in this process.

## Problem Statement

The problem for Day 5 involves processing a collection of manual updates, represented as sequences of pages, and validating them against a set of ordering rules. The goal is to determine the score of valid updates and the score of updates after reordering invalid ones.

## Approach

The intuition for solving the problem is to define a set of rules that specify which pages must follow which other pages, and then validate and reorder updates based on these rules. The solution involves parsing the rules and updates, validating the updates, and calculating scores based on the middle page of each update.

### Step 1: Defining the `OrderRules` Struct

The `OrderRules` struct is defined to represent the rules for page ordering. It uses a `HashMap` to map each page to a set of pages that must follow it.

```rust
pub type PageSet = HashSet<usize>;
pub type Page = usize;

pub struct OrderRules {
    rules: HashMap<Page, PageSet>
}

impl OrderRules {
    pub fn followed_by(&self, p: Page) -> Option<&PageSet> {
        self.rules.get(&p)
    }
}
```

### Step 2: Defining the `ManualUpdates` Struct

The `ManualUpdates` struct is defined to represent the updates to be validated and sorted. It includes methods for validating and sorting updates based on the rules provided.

```rust
pub struct ManualUpdates {
    list: Vec<Page>,
}

impl ManualUpdates {
    pub fn make_validator(rules: &OrderRules) -> impl Fn(&ManualUpdates) -> bool {
        |updates: &ManualUpdates| {
            updates
                .entries()
                .is_sorted_by(|&a, b| {
                    rules.followed_by(*a).map(|set| set.contains(b)).unwrap_or(false)
                })
        }
    }

    pub fn sort_update(rules: &OrderRules) -> impl Fn(&ManualUpdates) -> ManualUpdates {
        |updates: &ManualUpdates| {
            let mut list = updates.entries().cloned().collect::<Vec<_>>();
            list.sort_by(|&a, b| {
                rules.
                    followed_by(a)
                    .map(|set| {
                        if set.contains(b) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    })
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            ManualUpdates { list }
        }
    }
}
```

### Step 3: Main Function

The main function reads the input, parses the rules and updates, validates the updates, and calculates the scores for valid and reordered updates.

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("msg");
    let mut s = input.split("\n\n");

    let rules = s.next().unwrap().parse::<OrderRules>().unwrap();
    let manual_updates = s.next().unwrap()
        .lines()
        .map(|line| line.parse::<ManualUpdates>().unwrap())
        .collect::<Rc<[_]>>();

    let is_valid_order = ManualUpdates::make_validator(&rules);
    let t = Instant::now();
    let score = manual_updates.iter()
        .filter(|&update| is_valid_order(update))
        .map(|update| update.middle())
        .sum::<usize>();
    println!("Part 1: valid updates score: {score} - {:?}", t.elapsed());
    assert_eq!(6949, score);

    let t = Instant::now();
    let reorder_update = ManualUpdates::sort_update(&rules);
    let score = manual_updates.iter()
        .filter(|update| !is_valid_order(update))
        .map(reorder_update)
        .map(|update| update.middle())
        .sum::<usize>();
    println!("Part 2: Score for fixed updates : {score} - {:?}", t.elapsed());
    assert_eq!(4145, score);
}
```

### Summary

The solution involves defining the `OrderRules` and `ManualUpdates` structs, implementing methods for validating and sorting updates, and calculating scores based on the middle page of each update. The main function orchestrates the process by reading input, parsing the rules and updates, and calculating the required scores.
