# Day 3: Processing Instructions with a Simple CPU

This documentation explains a solution for Day 3 of Advent of Code 2024, which involves parsing and processing a program with special instructions.

## Solution Intuition

The key insight for this problem is recognizing that we need to parse a text file containing a mix of random characters and specific instruction patterns. These instructions need to be identified, extracted, and executed according to certain rules.

The solution involves:
1. Building a parser to extract valid instructions from noise
2. Creating a small virtual CPU to execute these instructions
3. Applying different execution modes for part 1 and part 2 of the challenge

## Step 1: Understanding the Input Format

The input consists of a text file with seemingly random characters interspersed with specific instruction patterns:

```rust
// Example of the input format
// mul(2,4) - multiply instruction
// do() and don't() - control flow instructions
```

The first task is to recognize and extract these patterns from the noise.

```rust
// This shows a small sample of the input to understand the format
// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
```

## Step 2: Defining the Program Structure

We need to represent our program and the instructions it contains:

```rust
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    MUL(u32, u32),  // Multiply two numbers
    DONT,           // Stop execution
    DO,             // Resume execution
}

struct Program {
    instructions: Vec<Instruction>,
}
```

This representation allows us to:
- Store each instruction type with its parameters
- Collect all valid instructions into a program structure
- Process them sequentially during execution

## Step 3: Parsing with Nom

For parsing, we use the Nom library, which provides parser combinators allowing us to build complex parsers from simpler ones:

```rust
impl FromStr for Program {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        while let Ok((remaining, (_, instruction))) =
            many_till(anychar, alt((parse_mul, parse_do, parse_dont)))(s)
        {
            instructions.push(instruction);
            s = remaining;
        }

        Ok(Self { instructions })
    }
}
```

The key insight here is using `many_till` to skip characters until we find a valid instruction pattern, then process that instruction and continue. This effectively ignores all the noise in the input.

## Step 4: Instruction Parsers

We define specialized parsers for each instruction type:

```rust
fn parse_mul(i: &str) -> IResult<&str, Instruction> {
    delimited(
        tag("mul("),
        map(
            separated_pair(
                nom::character::complete::u32,
                char(','),
                nom::character::complete::u32,
            ),
            |(x, y)| Instruction::MUL(x, y),
        ),
        tag(")"),
    )(i)
}

fn parse_do(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DO, tag("do()"))(i)
}

fn parse_dont(i: &str) -> IResult<&str, Instruction> {
    value(Instruction::DONT, tag("don't()"))(i)
}
```

These parsers recognize specific patterns in the input text:
- `mul(x,y)` - A multiplication instruction with two numeric parameters
- `do()` - A control flow instruction to resume execution
- `don't()` - A control flow instruction to pause execution

## Step 5: Creating a Simple CPU

Our CPU executes the instructions according to certain rules:

```rust
#[derive(Debug)]
struct Cpu {
    run_state: bool,        // Whether we're currently executing instructions
    use_enhanced: bool,     // Whether we respect control flow instructions
}

impl Cpu {
    fn use_simple_instructions() -> Cpu {
        Cpu {
            run_state: true,
            use_enhanced: false,
        }
    }

    fn use_enhanced_instructions() -> Cpu {
        Cpu {
            run_state: true,
            use_enhanced: true,
        }
    }
}
```

The CPU has two operation modes:
1. Simple mode (Part 1): Executes all `MUL` instructions, ignoring control flow
2. Enhanced mode (Part 2): Respects `DO` and `DON'T` instructions for controlling execution

## Step 6: Running the Program

The CPU executes the program by processing each instruction and accumulating results:

```rust
fn run(&mut self, pgm: &Program) -> u32 {
    pgm.instructions
        .iter()
        .filter_map(|&i| self.run_instruction(i))
        .sum::<u32>()
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
```

The insight here is:
- In simple mode, we always execute `MUL` instructions
- In enhanced mode, we only execute `MUL` when `run_state` is true
- `DO` and `DON'T` toggle the `run_state` flag in enhanced mode

## Step 7: Main Program Flow

The main function ties everything together:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day3/input.txt").unwrap();
    let pgm = input
        .parse::<Program>()
        .map_err(|e| panic!("{e:?}"))
        .unwrap();

    let t = Instant::now();
    let sum = Cpu::use_simple_instructions().run(&pgm);
    println!("part1: {} - {:?}", sum, t.elapsed());
    assert_eq!(185797128, sum);

    let t = Instant::now();
    let sum = Cpu::use_enhanced_instructions().run(&pgm);
    println!("part1: {} - {:?}", sum, t.elapsed());
    assert_eq!(89798695, sum)
}
```

This:
1. Reads the input file
2. Parses it into a program structure
3. Runs it in simple mode for part 1
4. Runs it in enhanced mode for part 2
5. Measures and displays execution time for each part

## Design Decisions

1. **Parser Combinators**: Using Nom makes the parsing elegant and maintainable compared to regex or manual string manipulation.

2. **Filtering During Execution**: The `filter_map` pattern efficiently collects only the results of instructions that produce values, avoiding the need for extra checks or intermediate collections.

3. **Enum for Instructions**: Using an enum with parameters keeps the instruction set extensible while maintaining type safety.

4. **State Machine**: The CPU behaves like a simple state machine, with the `run_state` flag determining whether instructions get executed.

5. **Mode Selection**: Parameterizing the behavior with `use_enhanced` makes it easy to switch between part 1 and part 2 without duplicating code.

## Conclusion

This implementation demonstrates several important programming principles:

1. **Separation of Concerns**: Parsing, instruction representation, and execution are kept separate.

2. **Declarative Parsing**: Nom allows us to describe what to parse rather than how to parse it.

3. **State Management**: The CPU's state is clearly defined and transitions are explicit.

4. **Functional Programming**: The use of iterators and combinators leads to concise, readable code.

The solution handles both parts of the challenge efficiently, processing a complex input format into a simple computational model.
