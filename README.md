# Practical Rust - Advent of Code 2024

This is an educational repository that offers practical examples demonstrating programming principles, design patterns, and Rust language features. Each challenge document explores different aspects of software development through the lens of specific problem solutions.

The repository includes a complete Nix development environment setup for reproducible builds and consistent development experiences across different platforms.

## Purpose

This repository aims to:
1. Demonstrate practical applications of programming concepts
2. Explore design decisions in real-world coding scenarios
3. Illustrate how Rust language features can address common programming challenges
4. Provide educational resources for developers looking to improve their skills

## Key Concepts Covered

Across these problems, you'll find examples of:

- **Data Structures**: HashMaps, vectors, custom structs, trees, graphs
- **Algorithms**: Dynamic programming, recursion, path finding, sorting, graph traversal
- **Rust Patterns**: Iterators, functional programming, traits, ownership, pattern matching
- **Design Principles**: Separation of concerns, abstraction, encapsulation, testability
- **Performance Optimization**: Memoization, efficient data representation, algorithmic choices

We hope these examples help enhance your understanding of both fundamental programming concepts and Rust-specific features. Happy reading!

## Code Puzzles Index

Below you'll find an index to all the documented problems in this repository:

### 1. [Day 1: Historian Hysteria](src/bin/day1/README.md)
- Processing pairs of numbers with absolute differences and frequencies
- Demonstrates basic file I/O, vector manipulation, zipping iterators, and hashmap usage

### 2. [Day 2: Red-Nosed Reports](src/bin/day2/README.md)
- Validating numeric sequences according to direction and difference rules
- Explores validation algorithms and modification analysis

### 3. [Day 3: Mull It Over](src/bin/day3/README.md)
- Parsing and executing a simple instruction set with specific control flow rules
- Demonstrates parser combinators, state machine design, and functional programming techniques

### 4. [Day 4: Ceres Search](src/bin/day4/README.md)
- Implementing directional word search algorithms for finding patterns in a 2D grid
- Demonstrates higher-order functions, iterator chaining, and spatial relationship modeling

### 5. [Day 5: Print Queue](src/bin/day5/README.md)
- Implementing ordering constraints as graph structures for validating and fixing page sequences
- Explores graph representations, constraint satisfaction, and efficient sorting algorithms

### 6. [Day 6: Guard Gallivant](src/bin/day6/README.md)
- Simulating a security guard navigating through a lab environment
- Demonstrates iterator-based state management, path analysis, loop detection, and practical applications of Rust's ownership model

### 7. [Day 7: Bridge Repair](src/bin/day7/README.md)
- Solving complex mathematical equations using multiple operation types (multiplication, addition, concatenation)
- Demonstrates recursive backtracking with memoization, higher-order functions, and declarative parsing

### 8. [Day 8: Resonant Collinearity](src/bin/day8/README.md)
- Calculating interference patterns between distributed antennas with variable harmonics
- Demonstrates functional iterator pipelines, spatial coordinate manipulation, and efficient collection transformations

### 9. [Day 9: Disk Fragmenter](src/bin/day9/README.md)
- Implementing disk space management with file segment manipulation and optimization strategies
- Demonstrates run-length encoding, data structure design, custom iterators, and functional transformations

### 10. [Day 10: Hoof It](src/bin/day10/README.md)
- Implementing path finding through a topographical map with strict height progression constraints
- Demonstrates recursive depth-first search, functional programming patterns, and effective use of memoization via history tracking

### 11. [Day 11: Plutonian Pebbles](src/bin/day11/README.md)
- Simulating magical stones with complex transformation rules using recursion and efficient caching
- Key topics: trait implementation, enumeration patterns, memoization, recursive function design

### 12. [Day 12: Garden Groups](src/bin/day12/README.md)
- Identifying and analyzing contiguous plant regions with area and perimeter calculations
- Demonstrates region tracking, BTree data structures, spatial transformations, and visualizing complex 2D layouts

### 13. [Day 13: Claw Contraption](src/bin/day13/README.md)
- Optimizing button press sequences to reach prizes with minimum cost
- Demonstrates recursive dynamic programming, memoization, and backtracking with interior mutability patterns

## How to Use This Repository

1. **Explore by Topic**: Review the index to find examples that match your interests
2. **Read the Documentation**: Each problem includes detailed explanations of approaches and implementation
3. **Examine the Code**: Study the implementation to understand how concepts are applied
4. **Run the Examples**: Execute the code to see solutions in action
5. **Modify and Experiment**: Change parameters or approaches to deepen your understanding

### Development Environment

This repository includes a complete Nix flake configuration for a reproducible development environment. The setup provides:
- Rust toolchain with appropriate versions
- Development dependencies and build tools
- Cross-platform compatibility (Linux, macOS)
- Reproducible builds for consistent results

#### Quick Start Commands

```bash
# Enter the development environment
nix develop

# Build all solutions
cargo build --release

# Run a specific solution (replace dayX with actual day)
cargo run --bin day1
cargo run --bin day2
# ... etc

# Run tests
cargo test

# Build reproducible packages
nix build .#advent2024-solutions
```

For detailed setup instructions and flake configuration explanation, refer to the [Nix Documentation](flake_nix_documentation.md).
