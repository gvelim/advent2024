# Word Search Puzzle Solution Documentation

## Overview
This documentation explores a Rust solution for a word search puzzle, demonstrating several programming principles including functional programming, iterator chaining, and spatial algorithms. The solution effectively searches a 2D grid for specific patterns ("XMAS" and "MAS") in various directions.

## Solution Intuition

The core challenge is to scan a 2D grid of characters to find specific words in multiple directions. The solution utilizes:

1. A spatial representation model to manage the 2D grid
2. Direction-based scanning algorithms to find patterns
3. Functional programming principles to create reusable, composable search functions
4. Efficient algorithms that avoid unnecessary work

## Step 1: Data Representation

The first step is to parse and represent the 2D character grid from the input file.

### Insight
A 2D grid requires a data structure that supports efficient positional access and boundary checks. The solution uses a `Field<char>` type, which wraps a vector of characters and provides methods for accessing elements by their 2D coordinates.

### Implementation
```rust
let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("File not found");
let field = input.parse::<Field<char>>().expect("Doesn't error");
let (height, width) = (field.height(), field.width());
```

This code reads the input file and parses it into a `Field<char>` representation, then extracts the grid dimensions for later use.

## Step 2: Direction-Based Word Matching

Next, we need a function to check if a word exists in a specific direction starting from a particular position.

### Insight
Word matching needs to:
1. Start from a specific position
2. Follow a specific direction vector
3. Check each character along the path
4. Handle grid boundaries gracefully

### Implementation
```rust
fn is_word_matched(field: &Field<char>, word: &str, start: Location, dir: DirVector) -> bool {
    word.char_indices()
        .all(|(i,c)| start
            // calculate new location based on (a) current index (b) starting position & (c) direction
            .move_relative((dir.0 * i as isize, dir.1 * i as isize))
            .map(|p| field
                // match the value in position with input's character
                .get(p)
                .map(|&val| val == c)
                .unwrap_or(false)
            ).unwrap_or(false)
        )
}
```

This function:
1. Iterates through each character in the target word with its index
2. For each character, calculates its expected position using the starting point and direction
3. Checks if the character at that position matches the expected character
4. Returns true only if all characters match

The use of `map` and `unwrap_or` ensures graceful handling of out-of-bounds coordinates.

## Step 3: Building a Reusable Scanner Factory

To avoid code duplication, the solution creates a function that generates specialized word scanners.

### Insight
Creating a higher-order function allows:
1. Preloading common parameters (field, directions)
2. Returning a specialized function that only needs the word and starting location
3. Abstracting the complexity of the scanning process

### Implementation
```rust
fn search_directions<'a>(
    field: &'a Field<char>,
    dirs: &'a [DirVector]
) -> impl Fn(&'a str, Location) -> Box<dyn Iterator<Item=(Location,DirVector)> + 'a>
{
    // return a function that takes a world and location
    // and performs a scan on field and set of directions that has be constructed with
    move |word: &'a str, pos: Location| {
        let ret = dirs.iter()
            .copied()
            .filter(move |&dir| is_word_matched(field, word, pos, dir))
            .map(move |dir| (pos,dir));
        // iterator must be boxed as it doesn't compile with "-> impl Iterator"
        Box::new(ret)
    }
}
```

This function:
1. Takes a field and a set of directions to search
2. Returns a function that takes a word and starting position
3. The returned function checks if the word exists in any of the specified directions
4. Returns an iterator of matching positions and directions

The use of Rust's lifetime annotations and boxed iterators ensures the returned function can be used flexibly.

## Step 4: Part 1 - Finding "XMAS" Words

The first part of the puzzle requires counting occurrences of "XMAS" and "SAMX" in the grid.

### Insight
For efficiency, we can:
1. Search for "XMAS" and its reverse "SAMX" simultaneously
2. Only scan in half the possible directions (the other half is covered by the reverse word)
3. Use nested iteration over the grid to check all starting positions

### Implementation
```rust
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
```

This code:
1. Creates a scanner for the horizontal, vertical, and diagonal directions
2. Iterates over each position in the grid
3. Counts occurrences of both "XMAS" and "SAMX" starting at each position
4. Sums the total occurrences

## Step 5: Part 2 - Finding "MAS" Crosses

The second part looks for a specific pattern: "MAS" (or "SAM") in two diagonal directions forming a cross.

### Insight
To find this pattern:
1. We need specialized scanners for the two diagonal directions
2. We need to check if both diagonals have matches at the same position
3. The pattern forms a cross with a specific geometry

### Implementation
```rust
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
```

This code:
1. Creates two specialized scanners: one for diagonal down-right and one for diagonal up-right
2. Checks each position to see if it has a "MAS" or "SAM" in the down-right direction
3. Also checks if there's a "MAS" or "SAM" in the up-right direction starting 2 positions down
4. These two conditions together form the cross pattern
5. Counts the total number of positions where both conditions are true

## Testing and Validation

The solution includes a test function to verify the word matching logic:

```rust
#[test]
fn test_scan_for_xmas() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    assert!(is_word_matched(&field, "XMAS", Location(9, 9), (-1, -1)));
    assert!(!is_word_matched(&field, "XMAS", Location(8, 9), (-1, -1)));
    // Additional assertions...
}
```

This test ensures the `is_word_matched` function correctly identifies words in the expected positions.

## Key Design Principles Demonstrated

1. **Function Composition**: The solution builds complex behavior from simple, composable functions
2. **Higher-Order Functions**: Using functions that produce specialized functions
3. **Iterators and Functional Chains**: Leveraging Rust's iterators for clean, expressive code
4. **Spatial Algorithms**: Efficiently handling 2D grid traversal and pattern matching
5. **Error Handling**: Gracefully handling boundary conditions and potential errors
6. **Performance Optimization**: Minimizing unnecessary work by searching in half the directions

## Conclusion

This solution demonstrates how to approach a 2D grid search problem using functional programming principles in Rust. By breaking down the problem into smaller, reusable pieces and leveraging Rust's type system and iterator patterns, the solution achieves both clarity and efficiency.

The approach is also extensible - new patterns or directions could be easily added by modifying the direction vectors and pattern recognition logic without changing the core algorithm structure.
