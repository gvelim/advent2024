# Understanding Advent of Code 2023 Day 5: Manual Sorting Algorithm

This document explains the solution to the Day 5 puzzle, which involves verifying and fixing ordered lists of pages based on specific ordering rules.

## Solution Intuition

The problem presents:
1. A set of rules where certain pages must follow others (e.g., page 53 must come after page 47)
2. Lists of page numbers that represent manual updates
3. Two tasks:
   - Identify which manual updates follow the rules
   - Fix the ones that don't follow the rules

## The Ordering Problem Explained

### What the Rules Mean

Let's clarify the ordering concept with a simple example. If we have a rule "47|53", it means:
- Page 53 must appear somewhere after page 47 in any valid list
- It doesn't have to be immediately after - other pages can appear between them
- But page 53 cannot appear before page 47

If we have multiple rules like:
```
47|53
53|29
```

This creates a transitive relationship: 47 must come before 53, and 53 must come before 29, so 47 must also come before 29.

### Directed Graph Representation

This naturally maps to a directed graph:
- Each page is a node
- Each rule creates a directed edge from one page to another
- A valid ordering is any topological sort of this graph

In the above example, valid orderings would be:
- 47, 53, 29
- 47, 53, 13, 29
- 47, 61, 53, 29
...and many more, as long as the directed edges are respected.

## Core Data Structures

### OrderRules

The `OrderRules` structure captures this directed graph:

```rust
pub struct OrderRules {
    // Key: Page, Value: Set of Pages that MUST follow
    rules: HashMap<Page, PageSet>
}
```

For each page (node), we store a set of pages (nodes) that must come after it. This is an adjacency list representation of the directed graph.

When parsing the input, each rule "X|Y" adds Y to the set of pages that must follow X:

```rust
rules
    .entry(x)
    .and_modify(|s: &mut PageSet| {s.insert(y);})
    .or_insert(HashSet::new())
    .insert(y);
```

### ManualUpdates

The `ManualUpdates` structure represents a list of pages that needs to be verified or fixed:

```rust
pub(crate) struct ManualUpdates {
    list: Vec<Page>,
}
```

## Validation Logic Deep Dive

### The Challenge of Verifying Ordering

To verify if a manual update follows the rules, we need to check if it respects all directed edges in our graph. This is different from checking if it's a valid topological sort, because:

1. The list might not contain all nodes in the graph
2. We only care about the relative ordering of nodes that are present in the list

### The Checking Algorithm

The validation logic uses Rust's `is_sorted_by` to check if pages maintain required ordering:

```rust
pub fn make_validator(rules: &OrderRules) -> impl Fn(&ManualUpdates) -> bool {
    |updates: &ManualUpdates| {
        updates
            .entries()
            .is_sorted_by(|&a,b|
                rules
                    .followed_by(*a)
                    .map(|set| set.contains(b))
                    .unwrap_or(false)
            )
    }
}
```

This works because:

1. `is_sorted_by` looks at consecutive pairs of elements (a, b)
2. For each pair, our comparator checks if b must follow a according to our rules
3. If any pair violates this, the list isn't correctly ordered

Consider this example with rules "47|53" and "53|29":
- If we have the list [47, 53, 29], all pairs ([47, 53], [53, 29]) respect the rules
- If we have [47, 29, 53], the pair [29, 53] violates the rules because 53 doesn't need to follow 29

The logic correctly handles cases where pages aren't directly connected in our graph.

## Fixing Invalid Orders

### The Challenge of Reordering

Fixing an invalid ordering is more complex than just verifying it. We need to rearrange the list to respect all rules.

### The Sorting Algorithm

The solution uses a *topological sort* via Rust's standard sorting function:

```rust
pub fn sort_update(rules: &OrderRules) -> impl Fn(&ManualUpdates) -> ManualUpdates {
    use std::cmp;

    |updates: &ManualUpdates| {
        let mut list = updates.entries().cloned().collect::<Vec<_>>();
        list.sort_by(|&a,b| {
            rules
                .followed_by(a)
                .map(|set|
                    if set.contains(b) { cmp::Ordering::Less } else { cmp::Ordering::Greater }
                )
                .unwrap_or(cmp::Ordering::Equal)
        });
        ManualUpdates { list }
    }
}
```

This works because:

1. We define a custom comparator for sorting
2. For any two pages a and b, the comparator returns:
   - Less if a must come before b (b is in a's "must follow" set)
   - Greater if a must come after b (implied by the rules)
   - Equal if there's no direct relationship

The sorting algorithm uses this comparator to rearrange all pages to satisfy the ordering rules.

### Why This Sorts Correctly

The custom comparator creates a partial ordering based on our directed graph:
- If there's a path from node A to node B in the graph, A should come before B
- If there's no path in either direction, their order doesn't matter

The standard sorting algorithm extends this partial ordering to a total ordering that respects all our rules.

## Example Walkthrough

Let's trace through a simple example with these rules:
```
47|53
53|29
```

And this manual update: [29, 47, 53]

### Validation
When checking if this update is valid:
1. For pair [29, 47]: Is 47 in the "must follow" set of 29? No, so this is okay.
2. For pair [47, 53]: Is 53 in the "must follow" set of 47? Yes, so this is correct.

However, the original ordering is [29, 47, 53], but 29 must come after 53 according to the rules, so this update is invalid.

### Fixing
When fixing this update:
1. We sort using our custom comparator
2. For 29 vs 47: No direct relationship, so they remain as-is
3. For 47 vs 53: 53 must follow 47, so 47 comes first
4. For 53 vs 29: 29 must follow 53, so 53 comes first
5. Final sorted list: [47, 53, 29]

## Bringing It All Together

The main function combines these components:
1. Parse the input into ordering rules and manual updates
2. For part 1: Filter valid updates, get their middle values, and sum them
3. For part 2: Filter invalid updates, reorder them, get their middle values, and sum them

```rust
fn main() {
    // [parsing code omitted]

    // Part 1: Find valid updates
    let is_valid_order = ManualUpdates::make_validator(&rules);
    let score = manual_updates
        .iter()
        .filter(|&update| is_valid_order(update))
        .map(|update| update.middle())
        .sum::<usize>();

    // Part 2: Fix invalid updates
    let reorder_update = ManualUpdates::sort_update(&rules);
    let score = manual_updates
        .iter()
        .filter(|update| !is_valid_order(update))
        .map(reorder_update)
        .map(|update| update.middle())
        .sum::<usize>();
}
```

## Key Insights

1. **Graph-Based Reasoning**: The ordering rules form a directed graph that defines the allowed orderings.

2. **Partial vs Total Ordering**: The rules define a partial ordering (some elements are comparable, others aren't), but we need a total ordering (all elements are comparable) for our final list.

3. **Custom Comparator Logic**: The custom comparator for sorting translates the graph-based constraints into a format that standard sorting algorithms can use.

4. **Efficient Validation**: Checking if a list respects the ordering can be done in linear time using the `is_sorted_by` function, which avoids quadratic comparisons.

This solution demonstrates how to efficiently encode ordering constraints as a graph and leverage Rust's standard library to implement the verification and sorting algorithms.
