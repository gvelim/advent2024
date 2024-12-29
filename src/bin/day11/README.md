# Day 11 Challenge - README

## Overview

The approach to solving the Day 11 challenge involves defining a `Blinker` struct with a cache for memoization, implementing the `count` method to recursively calculate the number of stones after a specified number of blinks, and defining the `Blink` trait for the `Stone` type. The provided code snippets illustrate the key steps in this process.

## Problem Statement

The problem for Day 11 involves processing a collection of stones, represented as integers, and performing a series of "blink" operations on them. The goal is to determine the number of stones after a specified number of blinks.

## Approach
The intuition for solving the problem is to leverage memoization to efficiently handle the exponential growth of possible stone states after each blink. Recursion is used to break down the problem into smaller subproblems, allowing the solution to build up from the simplest cases and reuse previously computed results.

### Step 1: Implementing the `Blink` Trait

The `Blink` trait is implemented for the `Stone` type to define the `blink` and `has_even_digits` methods. These methods determine how a stone splits or transforms during a blink.

```rust
pub type Stone = u64;

trait Blink {
    fn blink(self) -> BlinkResult;
    fn has_even_digits(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
enum BlinkResult {
    One(Stone),
    Two(Stone,Stone)
}

impl Blink for Stone {
    fn blink(self) -> BlinkResult {
        if self == 0 {
            BlinkResult::One(1)
        } else if self.has_even_digits() {
            let m = (10 as Stone).pow((self.ilog10() + 1) / 2);
            BlinkResult::Two(self / m, self % m)
        } else {
            BlinkResult::One(self * 2024)
        }
    }
    fn has_even_digits(&self) -> bool {
        self.ilog10() % 2 == 1
    }
}
```

### Step 2: Defining the `Blinker` Struct

The `Blinker` struct is defined with a cache to store intermediate results for optimization purposes.

```rust
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Blinker {
    cache: HashMap<(usize, Stone), usize>
}
```

### Step 3: Implementing the `count` Method

The `count` method recursively calculates the number of stones after a specified number of blinks. It uses memoization to store and reuse previously computed results.

```rust
impl Blinker {
    pub(crate) fn count(&mut self, blink: usize, stone: Stone) -> usize {
        if blink == 0 { return 1 }
        if let Some(&ret) =  self.cache.get(&(blink,stone)) { return ret }
        let ret = match stone.blink() {
            BlinkResult::One(a) => self.count(blink-1, a),
            BlinkResult::Two(a,b) =>
                self.count(blink-1, a)
                + self.count(blink-1, b),
        };
        self.cache.insert((blink,stone), ret);
        ret
    }
}
```

### Step 4: Main Function

The main function initializes the stones and the `Blinker` struct, then calculates the number of stones after a specified number of blinks.

```rust
mod blinker;

use std::time::Instant;
use blinker::{Blinker, Stone};

fn main() {
    let stones = vec![1 as Stone, 24596, 0, 740994, 60, 803, 8918, 9405859];

    let blink_counter = |stones: &[Stone], blinks: usize| {
        let mut blinker = Blinker::default();
        stones
            .iter()
            .map(|&stone| blinker.count(blinks, stone))
            .sum::<usize>()
    };

    let t = Instant::now();
    let count = blink_counter(&stones, 25);
    println!("Part 1: {count} stones after blinking 25 times - {:?}",t.elapsed() );
    assert_eq!(203457, count);

    let t = Instant::now();
    let count = blink_counter(&stones, 75);
    println!("Part 2: {count} stones after blinking 75 times - {:?}",t.elapsed() );
    assert_eq!(241394363462435, count);
}
```
