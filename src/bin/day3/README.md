# Day 3 Challenge - README

## Overview

In this challenge, we are tasked with simulating a CPU that processes a series of instructions. The challenge is divided into two parts: running the CPU with simple instructions and running it with enhanced instructions. The instructions include multiplication operations and control flow operations that can enable or disable the CPU.

## Problem Statement

### Part 1

For Part 1, we need to run the CPU using simple instructions. The CPU processes a series of multiplication instructions and sums the results. The instructions are read from an input file and parsed into a `Program` structure.

### Part 2

For Part 2, we need to run the CPU using enhanced instructions. In addition to multiplication instructions, the CPU can also process control flow instructions (`DO` and `DONT`) that enable or disable the CPU's ability to execute multiplication instructions.

## Approach

The solution involves parsing the input file to extract the instructions, initializing the CPU, and running the instructions while summing the results. We use the `nom` crate for parsing the instructions and implement the CPU logic in Rust.

### Step 1: Defining the Instruction Enum

We start by defining an `Instruction` enum to represent the different types of instructions that the CPU can process.

```rust
enum Instruction {
    MUL(u32, u32),
    DONT,
    DO,
}
```

### Step 2: Parsing the Instructions

Next, we use the `nom` crate to parse the instructions from the input file. The `Program` structure implements the `FromStr` trait to facilitate parsing.

```rust
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];
        while let Ok((remaining, (_, instruction))) = many_till(anychar, alt((parse_mul, parse_do, parse_dont)))(s) {
            instructions.push(instruction);
            s = remaining;
        }
        Ok(Self { instructions })
    }
}

fn parse_mul(i: &str) -> IResult<&str, Instruction> {
    delimited(tag("mul("), map(separated_pair(nom::character::complete::u32, char(','), nom::character::complete::u32), |(x, y)| Instruction::MUL(x, y)), tag(")"))(i)
}

fn parse_do(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DO, tag("do()"))(i)
}

fn parse_dont(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DONT, tag("don't()"))(i)
}
```

### Step 3: Reading and Parsing the Input

The input is read from a file and parsed into a `Program` structure, which contains a list of instructions.

```rust
let input = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
let pgm = input.parse::<Program>().unwrap();
```

### Step 4: Defining the CPU Structure

We define a `CPU` structure that holds the state of the CPU and a flag indicating whether enhanced instructions are used. The `CPU` structure includes methods for running the instructions and summing the results.

```rust
struct CPU {
    run_state: bool,
    use_enhanced: bool,
}

impl CPU {
    fn use_simple_instructions() -> CPU {
        CPU { run_state: true, use_enhanced: false }
    }

    fn use_enhanced_instructions() -> CPU {
        CPU { run_state: true, use_enhanced: true }
    }

    fn run(&mut self, pgm: &Program) -> u32 {
        pgm.instructions.iter()
            .filter_map(|&i| self.run_instruction(i))
            .sum()
    }

    fn run_instruction(&mut self, instruction: Instruction) -> Option<u32> {
        match instruction {
            Instruction::MUL(x, y) if self.run_state => Some(x * y),
            Instruction::DONT if self.use_enhanced => {
                self.run_state = false;
                None
            }
            Instruction::DO if self.use_enhanced => {
                self.run_state = true;
                None
            }
            _ => None,
        }
    }
}
```

### Step 5: Implementing Part 1

For Part 1, we initialize the CPU with simple instructions and run the program, summing the results of the multiplication instructions.

```rust
let t = Instant::now();
let sum = CPU::use_simple_instructions().run(&pgm);
println!("part1: {} - {:?}", sum, t.elapsed());
assert_eq!(185797128, sum);
```

### Step 6: Implementing Part 2

For Part 2, we initialize the CPU with enhanced instructions and run the program, summing the results while respecting the control flow instructions.

```rust
let t = Instant::now();
let sum = CPU::use_enhanced_instructions().run(&pgm);
println!("part2: {} - {:?}", sum, t.elapsed());
assert_eq!(89798695, sum);
```

## Conclusion

This challenge involves parsing a series of instructions and simulating a CPU that processes these instructions. By implementing the CPU logic and using the `nom` crate for parsing, we can efficiently solve both parts of the challenge.
