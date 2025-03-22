# Building a Claw Machine Solver: Path Finding & Dynamic Programming

## Introduction

This program solves a problem involving finding the optimal sequence of button presses to move a claw to a prize in a claw machine game. Each button moves the claw in a specific direction and has an associated cost. The goal is to find the minimum cost to reach the prize.

This solution demonstrates several important programming principles:
- Dynamic programming with memoization
- Recursive problem solving
- Path finding algorithms
- Data structure design

## Solution Intuition

The problem can be viewed as finding the shortest path from the prize to the origin (0,0), where the "edges" are the reverse of the button movements. By working backward from the prize, we can use dynamic programming to find the minimum cost path.

The key insight is that we need to:
1. Track visited locations to avoid cycles
2. Cache results to avoid recomputing the same subproblems
3. Use recursion with memoization to efficiently explore all possible paths

## Building Blocks of the Solution

### 1. Representing Locations and Directions

First, we need a way to represent locations and movement directions:

```rust
// From advent2024/location.rs (not shown in the code snippets)
pub type DirVector = (isize, isize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Location(pub usize, pub usize);

impl Location {
    pub fn is_origin(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    pub fn move_relative(&self, dir: DirVector) -> Option<Self> {
        // Move a location by a direction vector
        // Returns None if the move would result in negative coordinates
    }
}
```

This design separates the concepts of position and movement, making the code more expressive and easier to reason about.

### 2. Button Data Structure

Each button has a direction vector and a cost:

```rust
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub(crate) struct Button {
    dir: DirVector,
    cost: u32
}

impl Button {
    pub(crate) fn new(dir: DirVector, cost: u32) -> Self {
        Button { dir, cost }
    }
}
```

The `Button` structure encapsulates both the direction and cost, following good object-oriented design principles.

### 3. The Claw Machine

The `ClawMachine` structure is the core of the solution:

```rust
pub(crate) struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
    trail: RefCell<HashMap<u32, u32>>,
    paths: RefCell<Vec<PushCombinations>>,
}
```

Let's break down the fields:
- `buttons`: The available buttons (using `Rc<[Button]>` for shared ownership)
- `cache`: Memoization cache to store computed costs for visited locations
- `trail`: Tracks the current path being explored
- `paths`: Stores all found optimal paths

Using `RefCell` allows for interior mutability, which is necessary for the recursive algorithm to maintain state without requiring mutable method signatures.

### 4. Recursive Path Finding with Memoization

The core algorithm is implemented in the `_optimal_cost` method:

```rust
fn _optimal_cost(&self, prize: Location) -> Option<u32> {
    if let Some(val) = self.cache.borrow().get(&prize) {
        return *val;
    }
    if prize.is_origin() {
        self.paths.borrow_mut().push(
            self.trail.borrow()
                .iter()
                .map(|(x,y)| (*x,*y))
                .collect::<Vec<_>>()
        );
        return Some(0)
    }

    self.buttons
        .iter()
        .filter_map(|button| {
            let cost = prize
                .move_relative( reverse_dirvector(button.dir) )
                .and_then(|new_prize| {
                    self.trail.borrow_mut().entry(button.cost).and_modify(|c| *c += 1).or_insert(1);
                    let cost = self._optimal_cost(new_prize).map(|c| c + button.cost);
                    self.trail.borrow_mut().entry(button.cost).and_modify(|c| *c -= 1);
                    cost
                });
            self.cache.borrow_mut().insert(prize,cost);
            cost
        })
        .min()
}
```

This method demonstrates several important principles:

1. **Base Cases**:
   - If the location is already in the cache, return the cached value
   - If we've reached the origin, we've found a valid path

2. **Recursive Exploration**:
   - For each button, try moving from the current prize location in the reverse direction
   - Recursively find the optimal cost from the new location
   - Add the button's cost to the result

3. **Memoization**:
   - Cache results to avoid recomputing subproblems

4. **Path Tracking**:
   - Maintain a trail of button presses during exploration
   - When a solution is found, save the path

### 5. Parsing Input

The parser uses the Nom library to handle the structured input:

```rust
pub(super) fn parse_prize_clawmachine(input: &str) -> IResult<&str, (Location, ClawMachine)> {
    let (input, button_a) = terminated(parse_button, tag("\n"))(input)?;
    let (input, button_b) = terminated(parse_button, tag("\n"))(input)?;
    let (input, prize) = parse_prize(input)?;

    Ok((input, (prize, ClawMachine::new(&[button_a, button_b]))))
}
```

The parser creates a structured representation of the problem from the text input, demonstrating good separation of concerns between parsing and problem-solving logic.

### 6. Main Program Flow

The main function ties everything together:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Failed to read input file");

    let runs = input.split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res| res.map(|(_,res)| res))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| panic!("{:?}", e) )
        .unwrap();

    let sum = runs.iter()
        .filter_map(|run| {
            let (prize, machine) = run;
            let res = machine.optimal_cost(*prize);
            if let Some((cost,paths)) = res.clone() {
                println!("Optimal Cost: {:?}", cost);
                println!("Optimal Path: {:?}", paths);
            }
            res
        })
        .map(|(cost,_)| cost)
        .sum::<u32>();

    println!("Total Sum: {}", sum);
}
```

The main function follows a clean, functional approach:
1. Read the input file
2. Parse each run into a prize location and claw machine
3. Find the optimal cost for each run
4. Sum the costs

## Conclusion

This solution demonstrates several key programming principles:

1. **Dynamic Programming**: Using memoization to avoid redundant calculations
2. **Recursive Problem Solving**: Breaking down a complex problem into simpler subproblems
3. **Data Encapsulation**: Using appropriate data structures to model the problem
4. **Functional Programming**: Using iterator chains for clean, expressive code
5. **Memory Management**: Using Rust's ownership system with Rc and RefCell for shared, mutable state

The solution is efficient because it avoids recalculating paths to previously visited locations, and it's expressive because the code clearly maps to the problem domain.
