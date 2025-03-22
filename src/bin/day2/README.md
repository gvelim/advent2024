# Understanding a Validation Algorithm: Documentation

## Introduction

This document explains a Rust program that validates sequences of numbers and counts valid patterns according to specific criteria. The program examines "reports" containing numeric sequences, validates them against rules, and then explores how removing elements affects validity.

## Solution Intuition

The core problem involves validating sequences of numbers according to specific rules:
1. Each sequence must move consistently in one direction (increasing or decreasing)
2. Consecutive numbers must differ by 1, 2, or 3 units
3. Part 2 explores whether removing any single element can make an invalid sequence valid

This is essentially a pattern recognition problem where we need to:
- Identify valid sequences according to strict rules
- Count sequences that are already valid
- Count sequences that can become valid by removing one element

## Fundamental Building Blocks

### 1. Data Representation

We represent each numeric sequence as a `Report` struct with the following structure:

```rust
#[derive(Debug)]
struct Report {
    levels: rc::Rc<[usize]>,
}
```

The `Rc<[usize]>` (reference-counted array) is chosen to efficiently handle the sequences with shared ownership. This avoids unnecessary copying of data when manipulating sequences.

### 2. Parsing Input

The program parses the input file line by line, converting each line into a `Report`:

```rust
fn main() {
    let input = fs::read_to_string("src/bin/day2/input.txt").expect("File not found");
    let lists = input
        .lines()
        .map(|line| line.parse::<Report>().expect("Invalid list"))
        .collect::<Vec<Report>>();
    // ...
}
```

We implement the `FromStr` trait to allow parsing strings directly into our `Report` struct:

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

This approach leverages Rust's trait system to make the code more readable and maintainable, separating the parsing logic from the main program flow.

### 3. Sequence Validation Algorithm

The core algorithm checks two conditions for each sequence:

```rust
fn validate(r: &[usize]) -> bool {
    let dir = r[0] < r[1];
    r.windows(2)
        .all(|a| {
            (1..=3).contains(&(a[0].abs_diff(a[1])))
                && match dir {
                    true => a[0] < a[1],
                    false => a[0] > a[1],
                }
    })
}
```

This function:
1. Determines the direction (increasing or decreasing) based on the first two numbers
2. Uses the `windows(2)` method to check consecutive pairs of elements
3. Verifies two conditions for each pair:
   - The absolute difference is between 1 and 3 (inclusive)
   - The direction remains consistent throughout the sequence

The `all()` combinator ensures that every pair satisfies both conditions.

### 4. Part 1: Counting Valid Sequences

For part 1, we simply count the sequences that are already valid:

```rust
let count = lists.iter().filter(|r| r.is_safe()).count();
```

The `is_safe()` method is a simple wrapper around our validation function:

```rust
fn is_safe(&self) -> bool {
    Report::validate(&self.levels)
}
```

### 5. Part 2: Counting Sequences That Can Become Valid

In part 2, we need to determine if removing any single element makes an invalid sequence valid:

```rust
fn is_safe_dumpen(&self) -> bool {
    (0..self.levels.len()).any(|p| {
        let mut levels = self.levels.to_vec();
        levels.remove(p);
        Report::validate(&levels)
    })
}
```

This function:
1. Iterates through each position in the sequence
2. Creates a modified copy with that element removed
3. Checks if the modified sequence is valid
4. Returns true if any modified sequence is valid

We then count the sequences that satisfy this condition:

```rust
let count = lists.iter().filter(|r| r.is_safe_dumpen()).count();
```

## Performance Considerations

The program includes timing code to measure performance:

```rust
let t = time::Instant::now();
// code to time
println!("Part 1: {} = {:?}", count, t.elapsed());
```

Several design choices improve efficiency:
1. Using `Rc<[usize]>` allows efficient cloning of sequences
2. The `windows(2)` approach avoids manual indexing and bounds checking
3. The `any()` combinator in `is_safe_dumpen()` short-circuits once a valid modification is found

## Conclusion

This program demonstrates effective use of Rust's features:
1. Strong typing and error handling with the `Result` type
2. Trait implementations for clean parsing
3. Functional programming with iterators and combinators
4. Efficient ownership management with `Rc`

The algorithm itself highlights the importance of breaking down complex validation into simpler rules and leveraging Rust's standard library to express those rules concisely and efficiently.
