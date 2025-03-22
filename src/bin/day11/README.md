# Cosmic Blinker Simulation: A Recursive Solution with Memoization

This document explains the Cosmic Blinker program, which models the behavior of magical stones that can "blink" and multiply. It demonstrates key programming concepts including recursion, memoization, trait implementation, and enumeration patterns.

## Problem Intuition

We're simulating magical stones that follow these rules when they "blink":
- If a stone's value is 0, it becomes 1
- If a stone has an even number of digits, it splits into two stones (first half and second half)
- If a stone has an odd number of digits, it multiplies by 2024

We need to count how many stones we'll have after a specified number of blinks.

## Key Concepts Overview

This solution demonstrates:
1. **Recursion with Memoization**: Efficiently calculating results by caching intermediate values
2. **Custom Traits**: Extending existing types with new behaviors
3. **Enumerations for Multiple Return Types**: Representing different possible outcomes
4. **Efficient Number Manipulation**: Working with digits and powers of 10

## Step 1: Modeling the Stone Behavior

Our first task is to model how stones behave when they "blink."

### Insight
Instead of tracking individual stones, we define operations that transform stone values according to specific rules.

### Implementation
We create a `Blink` trait that adds behaviors to our `Stone` type (which is just a `u64`):

```rust
pub type Stone = u64;

trait Blink {
    fn blink(self) -> BlinkResult;
    fn has_even_digits(&self) -> bool;
}
```

For the blink results, we need to represent two different outcomes: either one new stone or two new stones. An enum is perfect for this:

```rust
#[derive(Debug, PartialEq, Eq)]
enum BlinkResult {
    One(Stone),
    Two(Stone, Stone)
}
```

## Step 2: Implementing the Blinking Logic

Now we'll implement the blink transformation rules.

### Insight
The behavior depends on the number of digits in the stone's value, so we need a way to determine if a number has an even or odd number of digits, and to extract specific parts of a number.

### Implementation
First, the digit counting helper:

```rust
fn has_even_digits(&self) -> bool {
    self.ilog10() % 2 == 1
}
```

This cleverly uses `ilog10()` to count digits: a number with n digits has a log10 value between n-1 and n.

Then, the complete blinking rules:

```rust
fn blink(self) -> BlinkResult {
    if self == 0 {
        BlinkResult::One(1)
    } else if self.has_even_digits() {
        let m = (10 as Stone).pow(self.ilog10().div_ceil(2));
        BlinkResult::Two(self / m, self % m)
    } else {
        BlinkResult::One(self * 2024)
    }
}
```

## Step 3: Recursive Counting with Memoization

Next, we need to simulate multiple "blinks" and count the resulting stones.

### Insight
This is naturally a recursive problem. For each stone, we can "blink" it and then recursively count how many stones result from those after further blinks.

However, naive recursion would be extremely inefficient as the same calculations would be repeated many times. Memoization solves this by caching intermediate results.

### Implementation
We create a `Blinker` struct with a `cache` to store previous calculations:

```rust
#[derive(Default)]
pub(crate) struct Blinker {
    cache: HashMap<(usize, Stone), usize>
}
```

The key recursive function is `count`, which returns the number of stones that will result from a given stone after a specific number of blinks:

```rust
pub(crate) fn count(&mut self, blink: usize, stone: Stone) -> usize {
    if blink == 0 { return 1 }
    if let Some(&ret) = self.cache.get(&(blink, stone)) { return ret }

    let ret = match stone.blink() {
        BlinkResult::One(a) => self.count(blink-1, a),
        BlinkResult::Two(a, b) =>
            self.count(blink-1, a)
            + self.count(blink-1, b),
    };

    self.cache.insert((blink, stone), ret);
    ret
}
```

This function has three key parts:
1. Base case: if no blinks remain, we have 1 stone
2. Cache check: return cached results if available
3. Recursive case: count stones after blinking, cache the result, then return it

## Step 4: Program Entry Point

Finally, we bring everything together in our main function.

### Insight
We want a flexible solution that can handle different input sets and different blink counts.

### Implementation
The main function processes the input and uses a local function to count stones:

```rust
fn main() {
    let stones = vec![1 as Stone, 24596, 0, 740994, 60, 803, 8918, 9405859];

    let blink_counter = |stones: &[Stone], blinks: usize| {
        let mut blinker = Blinker::default();
        stones
            .iter()
            .map(|&stone| blinker.count(blinks, stone))
            .sum::<usize>()
    };

    let count = blink_counter(&stones, 25);
    println!("Part 1: {count} stones after blinking 25 times");

    let count = blink_counter(&stones, 75);
    println!("Part 2: {count} stones after blinking 75 times");
}
```

This approach allows us to reuse the same counting logic for different parts of the problem.

## Performance Analysis

The memoization strategy is critical for performance:
- Without it, the solution would be exponential: O(2^n) where n is the number of blinks
- With memoization, it's closer to O(k*n) where k is the number of unique stone values encountered
- For large blink counts (like 75), this makes the difference between a solution that completes in milliseconds versus one that would take billions of years

## Key Takeaways

1. **Recursion with Memoization**: Powerful pattern for problems with overlapping subproblems
2. **Custom Traits**: Extend existing types with domain-specific behavior
3. **Enums as Return Types**: Elegantly handle multiple possible outcomes
4. **Type Aliases**: Improve code readability and maintainability
5. **Functional Programming Style**: Using iterators and closures for clean, expressive code

This solution illustrates how combining these programming concepts creates an elegant and efficient solution to a complex problem.
