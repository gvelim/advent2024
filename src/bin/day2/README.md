# Day 2 Challenge - README

## Overview

In this challenge, we are given a list of reports, each containing a sequence of levels. Our task is to determine how many of these reports are "safe" based on specific criteria.

## Problem Statement

### Part 1

For Part 1, we need to count the number of reports that are considered "safe". A report is deemed safe if the sequence of levels alternates in a strictly increasing or decreasing manner, with each step differing by at most 3.

### Part 2

For Part 2, we need to count the number of reports that can be made "safe" by removing exactly one level from the sequence.

## Approach

The solution involves reading the input file, parsing it into a list of `Report` structs, and then applying the safety criteria to count the valid reports.

### Step 1: Implementing the `Report` Struct

The `Report` struct contains a sequence of levels represented as a reference-counted slice (`Rc<[usize]>`). The struct provides methods to validate the sequence and check if it is safe.

```rust
#[derive(Debug)]
struct Report {
    levels: rc::Rc<[usize]>,
}
```

The `validate` method checks if the sequence alternates in a strictly increasing or decreasing manner, with each step differing by at most 3.

```rust
impl Report {
    fn validate(r: &[usize]) -> bool {
        let dir = r[0] < r[1];
        r.windows(2).all(|a| {
            (1..=3).contains(&(a[0].abs_diff(a[1])))
                && match dir {
                    true => a[0] < a[1],
                    false => a[0] > a[1],
                }
        })
    }
```

The `is_safe` method uses `validate` to check if the report is safe.

```rust
    fn is_safe(&self) -> bool {
        Report::validate(&self.levels)
    }
```

The `is_safe_dumpen` method checks if the report can be made safe by removing exactly one level.

```rust
    fn is_safe_dumpen(&self) -> bool {
        (0..self.levels.len()).any(|p| {
            let mut levels = self.levels.to_vec();
            levels.remove(p);
            Report::validate(&levels)
        })
    }
}
```

### Step 2: Parsing the Input

The input is read from a file and parsed into a list of `Report` structs. Each line in the input file represents a report, and the levels are space-separated integers.

```rust
impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            levels: s
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>())
                .collect::<Result<rc::Rc<[usize]>, ParseIntError>>()?
        })
    }
}
```

### Step 3: Main Function

The main function reads the input file, parses it into a list of `Report` structs, and then counts the number of safe reports for both parts of the challenge.

```rust
use std::{fs, num::ParseIntError, rc, str::FromStr, time};

fn main() {
    let input = fs::read_to_string("src/bin/day2/input.txt").expect("File not found");
    let lists = input
        .lines()
        .map(|line| line.parse::<Report>().expect("Invalid list"))
        .collect::<Vec<Report>>();
```

For Part 1, we count the number of reports that are safe.

```rust
    let t = time::Instant::now();
    let count = lists.iter().filter(|r| r.is_safe()).count();
    println!("Part 1: {} = {:?}", count, t.elapsed());
    assert_eq!(count, 407);
```

For Part 2, we count the number of reports that can be made safe by removing one level.

```rust
    let t = time::Instant::now();
    let count = lists.iter().filter(|r| r.is_safe_dumpen()).count();
    println!("Part 2: {} - {:?}", count, t.elapsed());
    assert_eq!(count, 459);
}
```
