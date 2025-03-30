# Claw Machine Path Optimization: A Dynamic Programming Approach

This document explains the design and implementation of a program that solves the "claw machine" optimization problem using dynamic programming. The challenge involves finding the minimum-cost sequence of button presses to reach a target prize location.

*Note*: We ignore the mathematical approach to solve this puzzle, by expressing it in the form of
- `Ax * Pa + Bx * Pb = PRx`
- `Ay * Pa + By * Pb = PRy`

and given
- Button A = `(Ax, Ay)`
- Button B = `(Bx, By)`
- Prize = `(PRx, PRy)`
- `Pa` = number of clicks, button A
- `Pb` = number of clicks, button B

## Table of Contents
1. [Problem Understanding](#problem-understanding)
2. [Solution Intuition](#solution-intuition)
3. [Core Data Structures](#core-data-structures)
4. [The Dynamic Programming Algorithm](#the-dynamic-programming-algorithm)
5. [Input Parsing](#input-parsing)
6. [Program Execution Flow](#program-execution-flow)
7. [Optimization Techniques](#optimization-techniques)
8. [Code Walkthrough](#code-walkthrough)

## Problem Understanding

We're given a claw machine with two buttons (A and B), each moving the claw in a specific direction with a specific cost:
- Button A costs 3 units and moves the claw by (Xa, Ya)
- Button B costs 1 unit and moves the claw by (Xb, Yb)

The goal is to find the minimum cost (sum of button press costs) to move the claw from the origin (0,0) to a target prize location (Xp, Yp).

## Solution Intuition

This problem is perfectly suited for dynamic programming due to its optimal substructure and overlapping subproblems:

1. **Optimal Substructure**: The minimum cost to reach a target can be composed of the minimum cost to reach intermediate positions plus the cost of additional button presses.

2. **Overlapping Subproblems**: As we work backward from the prize location, we'll encounter the same intermediate positions multiple times.

The key insight is to work backward from the prize location toward the origin, rather than forward from the origin to the prize. This allows us to build a recursive solution with memoization.

## Core Data Structures

Let's examine the fundamental building blocks:

### 1. Location and Direction Vectors

```rust
// From advent2024::location
struct Location(usize, usize);
type DirVector = (isize, isize);

// Function to reverse a direction vector
fn reverse_dirvector(dir: DirVector) -> DirVector {
    (-dir.0, -dir.1)
}
```

The `Location` represents a position in 2D space, while `DirVector` represents movement direction and magnitude. We need the `reverse_dirvector` function because we're working backward from the prize to the origin.

### 2. Button Representation

```rust
struct Button {
    dir: DirVector,
    cost: u32
}
```

The `Button` struct encapsulates the direction of movement and the cost of pressing that button.

### 3. ClawMachine and Its State

```rust
struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
    click_trail: RefCell<HashMap<u32, u32>>,
    combos: RefCell<Vec<ButtonCombinations>>,
}
```

The `ClawMachine` holds:
- A list of available buttons
- A cache for memoization
- A trail of button clicks for path reconstruction
- A collection of button combinations for solution tracking

## The Dynamic Programming Algorithm

The core algorithm uses a recursive approach with memoization:

```rust
fn _optimal_cost(&self, prize: Location) -> Option<u32> {
    // Check cache first
    if let Some(val) = self.cache.borrow().get(&prize) {
        return *val;
    }

    // Base case: we've reached the origin
    if prize.is_origin() {
        // Store button combinations and return cost 0
        self.combos.borrow_mut().push(
            self.click_trail.borrow().iter().map(|(x,y)| (*x,*y)).collect()
        );
        return Some(0);
    }

    // Try each button and find minimum cost
    let min_cost = self.buttons.iter().filter_map(|button| {
        // Calculate origin of current prize given button press
        prize.move_relative(reverse_dirvector(button.dir))
            .and_then(|origin_prize| {
                // Track button press
                self.click_trail.borrow_mut()
                    .entry(button.cost)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                // Recursively find cost for previous position
                let cost = self._optimal_cost(origin_prize)
                    .map(|c| c + button.cost);

                // Undo button press for backtracking
                self.click_trail.borrow_mut()
                    .entry(button.cost)
                    .and_modify(|c| *c -= 1);

                cost
            })
    }).min();

    // Cache result and return
    self.cache.borrow_mut().insert(prize, min_cost);
    min_cost
}
```

This approach:
1. Checks if we've already computed the cost for this position
2. If we've reached the origin, records the solution and returns cost 0
3. Otherwise, tries each button, computes costs recursively, and selects the minimum
4. Caches and returns the result

## Input Parsing

The program uses the Nom parsing library to handle the structured input:

```rust
fn parse_prize_clawmachine(input: &str) -> IResult<&str, (Location, ClawMachine)> {
    let (input, button_a) = terminated(parse_button, tag("\n"))(input)?;
    let (input, button_b) = terminated(parse_button, tag("\n"))(input)?;
    let (input, prize) = parse_prize(input)?;

    Ok((input, (prize, ClawMachine::new(&[button_a, button_b]))))
}
```

This creates a structured approach to parsing the input format:
- Button A description
- Button B description
- Prize location

## Program Execution Flow

The main program:
1. Reads the input file
2. Splits it into separate problem sets
3. Parses each problem
4. Calculates the optimal cost for each prize
5. Sums and displays the results

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Failed to read input file");

    let runs = input.split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res| res.map(|(_,res)| res))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let sum = runs.iter()
        .filter_map(|(prize, machine)| {
            if let Some((cost, paths)) = machine.optimal_cost(*prize) {
                println!("{cost}");
                Some(cost)
            } else {
                println!("No Solution");
                None
            }
        })
        .sum::<u32>();

    println!("Total Sum: {}", sum);
}
```

## Optimization Techniques

Several key optimizations make this solution efficient:

1. **Memoization**: We cache results to avoid recalculating costs for positions we've already visited.

2. **Working Backward**: By starting at the prize and working backward, we can efficiently prune the search space.

3. **Minimal State Tracking**: We only track the essential information needed to reconstruct the solution path.

4. **Reference Counting**: Using `Rc` for the buttons array prevents unnecessary cloning.

5. **Interior Mutability**: `RefCell` allows us to modify internal state during the recursive traversal.

## Code Walkthrough

### 1. The Button Type

```rust
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

Each button has a movement direction and cost. The implementation includes parsing from a string representation.

### 2. The ClawMachine Implementation

```rust
pub(crate) struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
    click_trail: RefCell<HashMap<u32, u32>>,
    combos: RefCell<Vec<ButtonCombinations>>,
}

impl ClawMachine {
    pub(crate) fn new(buttons: &[Button]) -> Self {
        ClawMachine {
            buttons: buttons.into(),
            cache: RefCell::new(HashMap::new()),
            click_trail: RefCell::new(HashMap::default()),
            combos: RefCell::new(Vec::new()),
        }
    }

    // Public API that returns cost and button combinations
    pub(crate) fn optimal_cost(&self, prize: Location) -> Option<(u32, Vec<ButtonCombinations>)> {
        self._optimal_cost(prize)
            .map(|c| {
                let paths = self.combos.borrow().clone();
                (c, paths)
            })
    }

    // Internal implementation with the dynamic programming logic
    fn _optimal_cost(&self, prize: Location) -> Option<u32> {
        // Implementation as described in the algorithm section
    }
}
```

The public API allows callers to get both the optimal cost and the button combinations to achieve it, while the internal implementation handles the recursive dynamic programming logic.

### 3. Parsing Infrastructure

The program uses Nom to create a declarative parsing approach:

```rust
pub(super) fn parse_prize_clawmachine(input: &str) -> IResult<&str, (Location, ClawMachine)> {
    // Sequentially parse button A, button B, and prize location
}

pub(super) fn parse_prize(input: &str) -> IResult<&str, Location> {
    // Parse "Prize: X=8400, Y=5400" format
}

pub(super) fn parse_button(input: &str) -> IResult<&str, Button> {
    // Parse "Button A: X+94, Y+34" format
}

pub(super) fn parse_numbers_pair(input: &str) -> IResult<&str, (u32,u32)> {
    // Parse X+94, Y+34 coordinate pairs
}
```

This parser framework provides robust error handling and clear separation of parsing concerns.

## Conclusion

This program demonstrates several important programming principles:

1. **Dynamic Programming**: Using recursion with memoization to solve complex optimization problems

2. **Functional Programming**: Using map/filter/reduce operations for data transformation

3. **Immutable Data**: Using interior mutability patterns to maintain clean function signatures

4. **Declarative Parsing**: Using combinators to create readable, maintainable parsers

5. **Type Safety**: Leveraging Rust's type system to model the problem domain accurately

The solution scales efficiently with the size of the input due to its O(XY) complexity where X and Y are the maximum coordinate values. By working backward from the prize and using memoization, we avoid the exponential explosion that would occur with a naive approach.
