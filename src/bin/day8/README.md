# Understanding Antenna Antinode Simulation in Rust

This documentation provides an educational guide to a Rust program that simulates antennas and their antinode patterns. The program explores spatial computations and uses various Rust features including iterators, collections, and custom data structures.

## Table of Contents
1. [Problem Intuition](#problem-intuition)
2. [Core Data Structures](#core-data-structures)
3. [Antenna Functionality](#antenna-functionality)
4. [City Representation](#city-representation)
5. [Antinode Calculation](#antinode-calculation)
6. [Program Execution](#program-execution)
7. [Design Patterns and Rust Features](#design-patterns-and-rust-features)

## Problem Intuition

The program simulates radio antennas distributed across a city grid. When pairs of antennas interact, they create "antinodes" - specific locations in the city where radio waves interfere. The goal is to identify these antinodes based on harmonics (multiplication factors that affect the distance and placement of antinodes).

The key insight is that antinodes are positioned at specific relative distances from antenna pairs, and these distances depend on the harmonics being considered.

## Core Data Structures

### Location

The program builds on a `Location` type that represents a 2D coordinate with methods for calculating distances and relative movements:

```rust
// From the Location module (not shown in the code snippets)
pub struct Location(pub usize, pub usize);

impl Location {
    // Calculates the distance between two locations
    pub fn distance(&self, other: &Location) -> (usize, usize) { ... }

    // Moves a location by a relative offset
    pub fn move_relative(&self, (dx, dy): (isize, isize)) -> Option<Location> { ... }
}
```

This fundamental building block provides the spatial reasoning capabilities needed for the antenna simulation.

## Antenna Functionality

### Antenna Structure

The `Antenna` struct encapsulates a location and provides methods to calculate antinodes:

```rust
#[derive(Debug, Clone, Copy)]
pub(crate) struct Antenna(pub Location);
```

The simple wrapper design pattern allows us to extend location with antenna-specific behaviors without modifying the original type.

### Calculating Antinode Pairs

The `antinode_pair` method computes the two antinodes created by a pair of antennas at a specific harmonic:

```rust
pub fn antinode_pair(&self, rhs: Antenna, harmonics: usize) -> [Option<Location>; 2] {
    let (dxu, dyu) = self.0.distance(&rhs.0);
    let (dx, dy) = ((harmonics * dxu) as isize, (harmonics * dyu) as isize);
    match (self.0.0 >= rhs.0.0, self.0.1 >= rhs.0.1) {
        (true, true) => [rhs.0.move_relative((-dx, -dy)), self.0.move_relative((dx, dy))],
        (true, false) => [rhs.0.move_relative((-dx, dy)), self.0.move_relative((dx, -dy))],
        (false, true) => [rhs.0.move_relative((dx, -dy)), self.0.move_relative((-dx, dy))],
        (false, false) => [rhs.0.move_relative((dx, dy)), self.0.move_relative((-dx, -dy))],
    }
}
```

The key insights here:
1. We calculate the base distance between the two antennas
2. We scale this distance by the harmonic factor
3. The direction of the antinode depends on the relative positions of the antennas
4. We return optional locations because antinodes might be outside valid bounds

### Multiple Harmonics

To compute antinodes across multiple harmonics, we create an iterator:

```rust
pub fn antinodes(&self, rhs: Antenna, harmonics: RangeInclusive<usize>)
    -> impl Iterator<Item = [Option<Location>; 2]> {
    harmonics
        .map(move |harmonics| self.antinode_pair(rhs, harmonics))
}
```

This demonstrates a functional programming approach by:
1. Taking a range of harmonics
2. Mapping each harmonic to its corresponding antinode pair
3. Returning an iterator instead of collecting results into a collection

## City Representation

### City Structure

The `City` struct models the entire grid with antennas:

```rust
pub(crate) struct City {
    city: Field<char>,
    antennas: HashMap<char, Vec<Antenna>>
}
```

This design shows:
1. Separation of the physical grid (`city`) from the logical components (`antennas`)
2. Grouping of antennas by type/character using a HashMap
3. Efficient lookup capability by antenna type

### Parsing a City

The `FromStr` implementation shows how to construct a `City` from a text representation:

```rust
impl FromStr for City {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let city = s.parse::<Field<char>>()?;
        let antennas: HashMap<char, Vec<Antenna>> = city.iter()
            .enumerate()
            .filter(|&(_, c)| c.ne(&'.'))
            .fold(HashMap::new(), |mut map, (i, &c)| {
                let loc = city.index_to_cartesian(i);
                map.entry(c)
                    .and_modify(|antennas| antennas.push(Antenna(loc)))
                    .or_insert(vec![Antenna(loc)]);
                map
            });
        Ok(City { city, antennas })
    }
}
```

This demonstrates:
1. Converting a string into a grid representation
2. Using iterators to process each character in the grid
3. Building a collection using folding operations
4. Using the entry API to efficiently update the HashMap

## Antinode Calculation

The `antinodes` method in the `City` structure is the core algorithm:

```rust
pub fn antinodes(&self, harmonics: RangeInclusive<usize>) -> impl Iterator<Item = Location> {
    self.antennas
        .values()
        .flat_map(move |antennas| antennas
            .iter()
            .tuple_combinations()
            .flat_map({
                let h = harmonics.clone();
                move |(a, b)| a
                    .antinodes(*b, h.clone())
                    .take_while(|&antinodes| {
                        match (antinodes[0], antinodes[1]) {
                            (_, Some(l)) if self.city.get(l).is_some() => true,
                            (Some(l), _) if self.city.get(l).is_some() => true,
                            _ => false
                        }
                    })
            })
        )
        .flat_map(|antinodes| antinodes.into_iter())
        .filter_map(|location|
            location.filter(|&location| self.city.get(location).is_some())
        )
}
```

This complex iterator pipeline demonstrates:

1. **Iterator Chaining**: Multiple transformations applied in sequence
2. **Combinations**: Using `tuple_combinations()` to generate all pairs of antennas
3. **Early Termination**: Using `take_while` to stop computing when antinodes fall outside the map
4. **Flattening**: Converting nested structures to flat iterators
5. **Filtering**: Keeping only valid locations
6. **Lazy Evaluation**: Computing antinodes on-demand

## Program Execution

The `main` function demonstrates how to use the city and measure performance:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day8/input.txt").unwrap();
    let city = input.parse::<City>().expect("Failed to parse City");

    let t = Instant::now();
    let count = city.antinodes(1..=1).unique().count();
    println!("Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}", count, t.elapsed());
    assert_eq!(247, count);

    let t = Instant::now();
    let count = city.antinodes(0..=100).unique().count();
    println!("Part 2: {:?} unique locations contain an antinode given the effects of resonant harmonics - {:?}", count, t.elapsed());
    assert_eq!(861, count);
}
```

Key aspects:
1. File I/O for input reading
2. Error handling for parsing
3. Performance measurement with `Instant`
4. Deduplication with `unique()`
5. Validation with `assert_eq`

## Design Patterns and Rust Features

Throughout this program, we see several advanced Rust programming concepts:

1. **Type-driven Design**: Using types like `Location` and `Antenna` to model domain concepts
2. **Functional Programming**: Heavy use of iterators and transformations
3. **Ownership and Borrowing**: Careful use of references and cloning when needed
4. **Trait Implementation**: Implementing `FromStr` for custom parsing
5. **Lazy Evaluation**: Using iterators for on-demand computation
6. **Testing**: Unit tests to verify correctness of individual functions
7. **Error Handling**: Using `Result` and `Option` types to handle potential failures

The program effectively uses Rust's zero-cost abstractions to create a solution that is both expressive and efficient, with a clear separation of concerns between data structures and algorithms.
