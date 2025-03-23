# Day 1 Challenge - README

## Overview

In this challenge, we are given a file containing pairs of numbers on each line. The challenge is divided into two parts: calculating the sum of absolute differences between corresponding pairs and computing a weighted sum based on the frequency of each number in the second list.

## Problem Statement

### Part 1

For Part 1, we need to calculate the sum of absolute differences between corresponding pairs of numbers from two lists derived from the input file.

### Part 2

For Part 2, we need to compute a weighted sum where each number in the first list is multiplied by its frequency in the second list.

## Approach

The solution involves reading the input file, parsing the numbers into two separate lists, and then performing the required calculations for each part. The steps are logically sequenced to ensure clarity and efficiency.

### Step 1: Reading and Parsing the Input

The input is read from a file and parsed into two separate lists of numbers. Each line in the file contains a pair of numbers, which are split and parsed into two vectors, `a` and `b`.

```rust
let input = fs::read_to_string("./src/bin/day1/input.txt").expect("File not found");

let (mut a, mut b): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|line| {
        let mut values = line.split_whitespace();
        (
            values
                .next()
                .expect("no values found")
                .parse::<usize>()
                .expect("invalid numeric"),
            values
                .next()
                .expect("Only one value found")
                .parse::<usize>()
                .expect("invalid numeric"),
        )
    })
    .unzip();
```

### Step 2: Sorting the Lists

Both lists are sorted to facilitate the calculations. Sorting ensures that the corresponding elements in both lists are in a consistent order, which is crucial for the subsequent steps.

```rust
a.sort();
b.sort();
```

### Step 3: Creating a Frequency Map

A frequency map of the second list is created to be used in Part 2. This map will store the count of each number in the list `b`, allowing for efficient lookups when calculating the weighted sum.

```rust
let hash_b = b.iter().fold(HashMap::new(), |mut map, key| {
    map.entry(key).and_modify(|val| *val += 1).or_insert(1);
    map
});
```

### Step 4: Implementing Part 1

For Part 1, we calculate the sum of absolute differences between corresponding pairs of numbers from the two lists. The `abs_diff` method is used to compute the absolute difference between each pair of numbers.

```rust
let t = Instant::now();
println!(
    "Part 1: {} - ({:?})",
    a.iter()
        .zip(b.iter())
        .map(|(x, &y)| x.abs_diff(y))
        .sum::<usize>(),
    t.elapsed()
);
```

### Step 5: Implementing Part 2

For Part 2, we compute a weighted sum where each number in the first list is multiplied by its frequency in the second list. The frequency map created in Step 3 is used to look up the frequency of each number in `a`.

```rust
let t = Instant::now();
println!(
    "Part 2: {} - ({:?})",
    a.iter()
        .map(|key| key * hash_b.get(key).unwrap_or(&0))
        .sum::<usize>(),
    t.elapsed()
);
```
