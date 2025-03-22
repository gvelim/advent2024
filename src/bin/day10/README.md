# Understanding the Trailhead Counting Program

This document explains a solution for counting trails on a topographical map. The program analyzes input data representing height levels to find paths that ascend strictly by one unit at a time until reaching a specific target height.

## Solution Intuition

The core challenge involves finding paths through a height map where:
1. We always start at the lowest points (height 0)
2. We can only step to adjacent positions (up, down, left, right)
3. We must increase height by exactly 1 with each step
4. We want to count paths that successfully reach a target height (9)

This is a classic graph traversal problem that can be solved using recursive depth-first search with some constraints on valid moves.

## Program Structure

The solution is organized into three main modules:
- `main.rs`: Entry point with problem definition and solution execution
- `topographical_map.rs`: Map representation and utility functions
- `trailhead.rs`: Path finding and path counting logic

Let's explore each component in detail.

## 1. The Map Representation

The foundation of our solution is a proper representation of the topographical data.

```rust
// topographical_map.rs
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

### Design Insights:
- The map is wrapped around the generic `Field<u8>` type which likely handles the grid structure
- The `get` method provides safe access to height values at a specific location
- The `lowests` method finds all starting points (locations with height 0) by:
  - Iterating through all cells
  - Filtering for cells with value 0
  - Converting raw indices to coordinate locations

This design creates a clean abstraction that hides the complexity of the underlying data structure.

## 2. The TrailHead Mechanism

The core algorithm for path finding is implemented in the `TrailHead` struct:

```rust
// trailhead.rs
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

    // Path finding and counting method...
}
```

### Key Design Choice:
The `history` field is an `Option<HashSet<Location>>`, allowing two different trail counting strategies:
- When `Some`: Tracks visited locations to avoid counting them multiple times
- When `None`: Allows revisiting locations for finding unique paths

This flexible design enables solving both part 1 and part 2 with minimal code duplication.

## 3. The Path Finding Algorithm

The core algorithm uses recursive depth-first search with constraints:

```rust
pub(crate) fn count_trails(
    &mut self,
    map: &TopographicalMap,
    loc: Location,
    is_found: impl Fn(u8) -> bool + Copy
) -> Option<usize>
{
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
```

### Algorithm Breakdown:
1. First check if we've reached a target height (`is_found(val)`)
2. If found, return 1 (we've found a valid path)
3. Otherwise, explore all four adjacent directions:
   - Filter out invalid moves (stepping outside map boundaries)
   - Filter out neighbors we've already visited (if tracking history)
   - Filter out neighbors that don't have exactly one height higher
4. For valid neighbors, recursively count paths and sum the results

### Elegant Functional Style:
- Uses iterator chaining for clean, declarative code
- Uses `filter_map` to both filter invalid moves and transform results
- Higher-order function `is_found` enables flexible termination criteria

## 4. Executing the Solution

The `main.rs` file brings everything together:

```rust
// main.rs
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

### Solution Process:
1. Parse the input file into our `TopographicalMap` structure
2. For part 1:
   - Find all the lowest points (height 0)
   - For each start location, create a `TrailHead` that tracks history
   - Count paths that reach height 9
   - Sum all path counts
3. For part 2:
   - Similar approach but with unique trails (no history tracking)

## Key Programming Principles Demonstrated

1. **Separation of Concerns**:
   - Map representation is separate from path finding logic
   - Main program coordinates high-level workflow

2. **Functional Programming Style**:
   - Iterator chaining for clean data processing
   - Higher-order functions for customizable behavior

3. **Efficient Data Structures**:
   - HashSet for O(1) lookups of visited locations
   - Field-based grid representation for map data

4. **Smart Algorithm Design**:
   - Recursive DFS with early termination
   - Code reuse between similar problems through parameterization

5. **Performance Measurement**:
   - Timing each solution part for performance analysis

## Conclusion

This program demonstrates an elegant recursive approach to path finding on a grid. The use of Rust's type system and functional programming features creates a solution that is both readable and efficient. The two different modes (with and without history tracking) showcase how a small design difference can change the behavior of the algorithm while maintaining the same core logic.
