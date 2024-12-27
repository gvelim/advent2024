
# Day 10 Challenge - README

## Overview

In this challenge, we are given a topographical map represented as a grid of numbers. Each number represents the elevation at that point on the map. Our task is to identify specific points on the map called "trailheads" and calculate certain metrics based on trails starting from these trailheads.

## Problem Statement

### Part 1

For Part 1, we need to find all the trailheads and calculate the sum of their scores. A trailhead is defined as a point on the map where the elevation is `0` and lead to a `9`. From each trailhead, we can move to adjacent points (up, down, left, right) if the elevation increases by exactly `1`. The score of a trailhead is the number of valid trails that can be formed starting from it and ending at a point where the elevation is `9`.

### Part 2

For Part 2, we need to find all unique trails starting from the trailheads and calculate the sum of their ratings. The rating of a trailhead is the number of unique trails that can be formed starting from it and ending at a point where the elevation is `9`.

## Approach
The intuition behind the solution is to use a `depth-first search (DFS)` approach to explore all possible trails from each trailhead, ensuring that we only move to adjacent points with increasing elevation.

### Step 1: Implementing the `TopographicalMap` Struct

The topographical map is represented using a custom `TopographicalMap` struct that wraps around a `Field<u8>`. This struct provides methods to access the elevation at a specific location and to find all trailheads (points with elevation `0`).

```rust
#[derive(Debug)]
pub(crate) struct TopographicalMap(Field<u8>);

impl TopographicalMap {

    #[inline]
    pub(crate) fn get(&self, loc: Location) -> Option<&u8> {
        self.0.get(loc)
    }

    pub(crate) fn lowests(&self) -> impl Iterator<Item = Location> {
        self.0.iter()
            .enumerate()
            .filter(|&(_,s)| *s == 0)
            .map(|(idx,_)| self.0.index_to_cartesian(idx))
    }
}
```

### Step 2: Defining the `TrailHead` Struct

The `TrailHead` struct is used to explore trails starting from a given trailhead. It maintains a history of visited locations to ensure that trails do not revisit any point (for Part 1). The `count_trails` method recursively explores all valid trails from a given starting point. For part 2, the `unique_trails` method is used to create a `TrailHead` instance that tracks unique trails by disabling the history tracking.

```rust
#[derive(Debug, Default)]
pub(crate) struct TrailHead {
    history: Option<HashSet<Location>>
}

impl TrailHead {
    pub(crate) fn unique_trails() -> TrailHead {
        TrailHead { history: None }
    }
    pub(crate) fn trail_heads() -> TrailHead {
        TrailHead { history: Some(HashSet::new()) }
    }
    pub(crate) fn count_trails(&mut self, map: &TopographicalMap, loc: Location, is_found: fn(u8)->bool) -> Option<usize> {
        let &val = map.get(loc)?;
        if is_found(val) { return Some(1) }
        Some(
            [(0,1),(0,-1),(1,0),(-1,0)]
                .into_iter()
                .filter_map(|dir| loc.move_relative(dir))
                .filter_map(|neighbor| {
                    if self.history.as_ref().is_some_and(|h| h.contains(&neighbor)) { return None };
                    if map.get(neighbor).is_some_and(|&nv| nv != val+1) { return None }

                    self.history.as_mut().map(|h| h.insert(neighbor));
                    self.count_trails(map, neighbor, is_found)
                })
                .sum::<usize>()
        )
    }
}
```

### Step 3: Main Function

The main program reads the input file, parses it into a `TopographicalMap`, and then calculates the required sums for Part 1 and Part 2 using the methods provided by `TopographicalMap` and `TrailHead`.

```rust
mod topographical_map;
mod trailhead;

use std::time::Instant;
use topographical_map::TopographicalMap;
use trailhead::TrailHead;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").unwrap();
    let map = input.parse::<TopographicalMap>().unwrap();

    let t = Instant::now();
    let sum = map.lowests()
        .filter_map(|start|
            TrailHead::trail_heads().count_trails(&map, start, |d| d == 9)
        )
        .sum::<usize>();
    println!("Part 1: Sum of the scores of all trailheads = {sum} - {:?}", t.elapsed());
    assert_eq!(786,sum);

    let t = Instant::now();
    let sum = map.lowests()
        .filter_map(|start|
            TrailHead::unique_trails().count_trails(&map, start, |d| d == 9)
        )
        .sum::<usize>();
    println!("Part 2: Sum of the ratings of all unique trailheads = {sum} - {:?}", t.elapsed());
    assert_eq!(1722,sum);
}
```
