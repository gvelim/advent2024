# Simulating a Robot in a Lab: A Step-by-Step Guide

## Overview

This document explains a program that simulates a robot guard navigating through a lab environment. The guard follows a set of rules to move through the lab, and the program tracks its path and analyzes specific aspects of its movement.

## Solution Intuition

The core concept is to simulate a robot guard that follows a simple set of rules:
1. Turn clockwise until finding a clear path
2. Move forward one step
3. Repeat

This creates a deterministic path through the lab. The challenge is divided into two parts:
1. Count unique locations visited by the guard
2. Identify obstacles that would cause the guard to get stuck in a loop if placed in the path

## 1. Fundamental Data Structures

### Lab Representation

First, we need to represent the lab environment. The program uses a `Field` type to represent a 2D grid where:
- `'.'` represents an empty space
- `'#'` represents a wall/obstacle
- `^`, `>`, `v`, `<` represent the guard's initial position and orientation

```rust
use advent2024::field::Field;
use advent2024::location::*;

pub type Lab = Field<char>;
```

### Location and Direction

The guard's position and movement are represented using:

```rust
pub(crate) struct Guard<'a> {
    pub lab: &'a Lab,      // Reference to the lab
    pub dir: DirVector,    // Direction vector (e.g., (0,-1) for up)
    pub pos: Location      // Current position (x,y coordinates)
}
```

**Insight**: Separating the position from direction allows us to track the guard's state completely, enabling simulation of its movement according to the rules.

## 2. Initializing the Guard

Before simulating movement, we need to find the guard's starting position and direction:

```rust
pub fn find_guard(lab: &Lab, token: &[char]) -> Option<(Location, DirVector)> {
    lab
        .iter()
        .position(|c| token.contains(c))
        .map(|idx| {
            let loc = lab.index_to_cartesian(idx);
            (
                loc,
                lab.get(loc)
                    .map(|val|
                        match &val {'^' => (0,-1),'>' => (1,0),'v' => (0,1),'<' => (-1,0), _ => unreachable!()}
                    )
                    .unwrap()
            )
        })
}
```

**Insight**: This function searches for one of the directional characters (`^`, `>`, `v`, `<`) and converts it to a location and corresponding direction vector. The use of `Option` elegantly handles the possibility that no guard is found.

## 3. Implementing Guard Movement

The guard movement follows the right-hand rule - it keeps turning until it finds a clear path:

```rust
impl Iterator for Guard<'_> {
    type Item = (Location, DirVector);

    fn next(&mut self) -> Option<Self::Item> {
        // turn until you find a way fwd
        while let Some(&'#') = self.lab.peek(self.pos, self.dir) {
            self.dir = turn_cw(self.dir);
        }
        // move next position as long as it is within bounds
        self.pos.move_relative(self.dir)
            .filter(|&p| self.lab.within_bounds(p))
            .map(|pos| {
                self.pos = pos;
                (pos, self.dir)
            })
    }
}
```

**Insight**:
- Implementing `Iterator` for `Guard` makes the movement logic reusable and allows using all of Rust's iterator methods.
- The guard turns clockwise until finding a clear path, then moves forward one step.
- Each iteration returns both the new position and direction, which is crucial for tracking the guard's path.

## 4. Solving Part 1: Counting Unique Locations

For the first challenge, we need to count how many unique locations the guard visits:

```rust
let mut unique_locations = Guard{lab:&lab,pos,dir}.collect::<HashMap<_,_>>();
unique_locations.insert(pos,dir);
println!("Part 1: Guard visited {:?} unique locations - {:?}", unique_locations.len(), t.elapsed());
```

**Insight**:
- Using a `HashMap` to collect location → direction pairs automatically handles duplicate locations.
- We explicitly add the starting position which might not be included in the iterator.
- This approach efficiently counts unique locations without needing to track the full path sequence.

## 5. Solving Part 2: Finding Loop-Causing Obstacles

For the second challenge, we need to identify obstacles that would cause the guard to get stuck in a loop:

```rust
let obstacles = unique_locations
    .iter()
    .filter(|&(l, _)| {
        path.clear();
        *lab.get_mut(*l).unwrap() = '#';  // Place test obstacle
        // Simulate guard movement with the obstacle
        let in_loop = Guard{lab:&lab,pos,dir}
            .any(|(nl,nd)| {
                let in_loop = path.get(&nl).is_some_and(|&pd| nd == pd);
                path.entry(nl).or_insert(nd);
                in_loop
            });
        *lab.get_mut(*l).unwrap() = '.';  // Remove test obstacle
        in_loop
    })
    .count();
```

**Insight**:
- For each unique location visited, we temporarily place an obstacle there and rerun the simulation.
- We detect a loop by checking if the guard revisits any location from the same direction.
- This is an important distinction: revisiting a location from a different direction doesn't constitute a loop.
- The approach modifies the lab temporarily for each test, then restores it.

## 6. Performance Considerations

The solution uses several techniques to optimize performance:

1. **Reusing the Iterator**: The `Guard` iterator encapsulates the movement logic, making it reusable.
2. **Efficient Loop Detection**: Using a `HashMap` to track positions and directions allows O(1) lookups.
3. **Space Efficiency**: Only storing unique locations rather than the full path sequence.
4. **Temporary Modifications**: Modifying the lab in-place for obstacle testing rather than creating copies.

## 7. Main Program Flow

The main function orchestrates the entire process:

```rust
fn main() {
    // Load and parse the input
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let mut lab = input.parse::<Lab>().expect("Field parse err");

    // Find the guard's starting position and direction
    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");

    // Part 1: Count unique locations
    let t = Instant::now();
    let mut unique_locations = Guard{lab:&lab,pos,dir}.collect::<HashMap<_,_>>();
    unique_locations.insert(pos,dir);
    println!("Part 1: Guard visited {:?} unique locations - {:?}", unique_locations.len(), t.elapsed());

    // Part 2: Find loop-causing obstacles
    let t = Instant::now();
    let mut path = HashMap::new();
    let obstacles = unique_locations
        .iter()
        .filter(|&(l, _)| {
            // Test logic for each potential obstacle
            // [Implementation details omitted for brevity]
        })
        .count();

    println!("Part 2: There are {:?} loop obstacles - {:?}", obstacles, t.elapsed());
}
```

## Conclusion

This program demonstrates several important programming principles:

1. **Separation of concerns**: The guard logic is separate from the main program flow
2. **Iterators for complex sequences**: Using Rust's iterator pattern to model the guard's movement
3. **Efficient data structures**: Using HashMaps for quick lookups and deduplication
4. **Temporary state modification**: Modifying and restoring state for testing
5. **Error handling**: Using Option and Result types to handle potential failures gracefully
6. **Performance measurement**: Including timing to measure solution efficiency

The solution elegantly tackles a complex simulation problem through clear abstractions and efficient algorithms, resulting in a program that effectively models a robot's movement through a constrained environment.
