# Day 4 Challenge - README

## Overview

In this challenge, we are given a grid of characters and tasked with finding specific words within this grid. The challenge is divided into two parts: finding occurrences of the word "XMAS" and its reverse "SAMX", and identifying specific patterns that form a cross with the word "MAS".

## Problem Statement

### Part 1

For Part 1, we need to find all occurrences of the word "XMAS" and its reverse "SAMX" in the grid. The words can appear in four directions: horizontally, vertically, and diagonally (both left-to-right and right-to-left).

### Part 2

For Part 2, we need to identify patterns that form a cross with the word "MAS". Specifically, we need to find locations where "MAS" or "SAM" appears diagonally in one direction and "MAS" or "SAM" appears diagonally in the opposite direction, forming a cross.

## Approach

The solution involves scanning the grid in multiple directions and checking for the presence of the specified words. We use helper functions to perform these scans and count the occurrences.

### Step 1: Reading and Parsing the Input

The input is read from a file and parsed into a `Field<char>` structure, which represents the grid of characters.

```rust
let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("File not found");
let field = input.parse::<Field<char>>().expect("Doesn't error");
let (height, width) = (field.height(), field.width());
```

### Step 2: Defining the Search Function

We define a function `search_directions` that takes a reference to the field and a set of directions. This function returns a closure that can be used to scan directinally for a specific word starting from a given location.

```rust
fn search_directions<'a>(field: &'a Field<char>, dirs: &'a [DirVector]) -> impl Fn(&'a str, Location) -> Box<dyn Iterator<Item=(Location,DirVector)> + 'a> {
    move |word: &'a str, pos: Location| {
        let ret = dirs.iter()
            .copied()
            .filter(move |&dir| is_word_matched(field, word, pos, dir))
            .map(move |dir| (pos,dir));
        Box::new(ret)
    }
}
```

### Step 3: Checking for Word Matches

The `is_word_matched` function checks if a given word matches the characters in the field starting from a specific location and moving in a specified direction.

```rust
fn is_word_matched(field: &Field<char>, word: &str, start: Location, dir: DirVector) -> bool {
    word.char_indices()
        .all(|(i,c)| start
            .move_relative((dir.0 * i as isize, dir.1 * i as isize))
            .map(|p| field
                .get(p)
                .map(|&val| val == c)
                .unwrap_or(false)
            ).unwrap_or(false)
        )
}
```

### Step 4: Implementing Part 1

For Part 1, we scan the grid for the words "XMAS" and "SAMX" in four directions: horizontally, vertically, and diagonally.

```rust
let t = Instant::now();
let xmas_scanner = search_directions(&field, &[(1,0),(0,1),(1,1),(1,-1)]);
let sum = (0..width)
    .map(|x| (0..height)
        .map(|y|
            xmas_scanner("XMAS", Location(x,y)).count()
            + xmas_scanner("SAMX", Location(x,y)).count()
        )
        .sum::<usize>()
    )
    .sum::<usize>();
println!("Part 1: Found ({sum}) XMAS words - {:?}",t.elapsed());
assert_eq!(2603,sum);
```

### Step 5: Implementing Part 2

For Part 2, we scan the grid for the word "MAS" or "SAM" forming a cross pattern in two diagonal directions.

```rust
let t = Instant::now();
let mas_leg1_scanner = search_directions(&field, &[(1,1)]);
let mas_leg2_scanner = search_directions(&field, &[(1,-1)]);
let sum = (0..height)
    .map(|y| (0..width)
        .filter(|&x|
            (mas_leg1_scanner("MAS",Location(x,y)).count() == 1 ||
                mas_leg1_scanner("SAM",Location(x,y)).count() == 1) &&
                (mas_leg2_scanner("MAS",Location(x,y+2)).count() == 1 ||
                    mas_leg2_scanner("SAM",Location(x,y+2)).count() == 1)
        )
        .count()
    )
    .sum::<usize>();
println!("Part 2: Found ({sum}) MAS crosses - {:?}",t.elapsed());
assert_eq!(1965,sum);
```
