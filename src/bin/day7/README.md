# The Equation Solver - Exploring Recursive Solutions

This document provides an educational walkthrough of an equation solver program designed to tackle a specific puzzle challenge. The program solves mathematical equations where a target value must be constructed using a set of coefficient values combined with different operations.

## 1. Solution Intuition: Understanding the Problem

The problem presents equations in the format `result: coeff1 coeff2 coeff3...` where we need to find valid ways to combine coefficients to achieve the result value. The solution approach uses recursive backtracking to explore different operation combinations.

The operations available are:
- Multiplication: `a * b`
- Addition: `a + b`
- Concatenation: e.g., `a = 5, b = 7` → `57` (introduced in part 2)

Our strategy will be to recursively try each operation with each coefficient, exploring all possible paths until we find a valid solution that matches the target result.

## 2. Data Structure: Representing Equations

The first building block is a structure to represent an equation:

```rust
#[derive(Debug)]
pub(crate) struct Equation {
    result: u64,
    coeff: Rc<[u64]>
}
```

This structure stores:
- `result`: The target value we need to achieve
- `coeff`: The coefficients available for operations, stored as a reference-counted array for efficient memory management

Using `Rc<[u64]>` is a design choice allowing for shared ownership of the coefficients without unnecessary copying during recursive calls.

## 3. Parser: Converting Text to Data

We need to parse input strings like `"190: 10 19"` into `Equation` structures. The program uses the `nom` parsing library for robust, declarative parsing:

```rust
fn parse_equation(s: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
            u64,
            tuple(( space0, tag(":") )),
            tuple(( space0, separated_list1(space1,u64) ))
        ),
        |(result, (_, coeff))| Equation { result, coeff: coeff.into() }
    )(s)
}
```

This parser:
1. Captures a number before the colon as the result
2. Allows flexible spacing around the separator
3. Parses all remaining numbers as coefficients
4. Converts the parsed data into an `Equation` struct

Implementing the `FromStr` trait enables convenient parsing with Rust's `.parse()` method:

```rust
impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_equation(s) {
            Ok(e) => Ok(e.1),
            Err(e) => Err(e.to_string()),
        }
    }
}
```

## 4. Core Algorithm: Recursive Solver

The heart of the solution is the recursive solver function which explores all possible ways to combine coefficients:

```rust
fn solve(total: u64, coeff: &[u64], cop: bool) -> Option<u64> {
    fn ct(a:u64, b:u64) -> u64 { format!("{}{}",a,b).parse::<u64>().unwrap() }

    let idx = coeff.len() - 1;

    if idx == 0 { return Some(coeff[idx]) }

    let res_1 = Self::solve(total / coeff[idx], &coeff[..idx], cop).map(|s| s * coeff[idx]);
    let res_2 = if total >= coeff[0] {
        Self::solve(total - coeff[idx], &coeff[..idx], cop).map(|s| s + coeff[idx])
    } else { None };
    let res_3 = if cop && total >= coeff[0] {
        Self::solve((total - coeff[idx])/10u64.pow(coeff[idx].ilog10()+1), &coeff[..idx], cop)
            .map(|s| ct(s, coeff[idx]))
    } else { None };

    match (res_1 == Some(total), res_2 == Some(total), res_3 == Some(total)) {
        (true, _, _) => res_1,
        (_, true, _) => res_2,
        (_, _, true) => res_3,
        _ => None,
    }
}
```

The algorithm works by:

1. **Base Case**: When reaching the last coefficient, return it directly
2. **Recursive Exploration**: Try three different operations with the current coefficient:
   - Multiplication: Divide the target by the coefficient and recursively solve
   - Addition: Subtract the coefficient from the target and recursively solve
   - Concatenation (when `cop` is true): Handle digit concatenation as a special operation

3. **Solution Selection**: Once operations are tried, return the first valid solution that exactly matches the target value

The approach effectively creates a decision tree where each level tries all operations with the next coefficient.

## 5. Efficiency Optimizations

The solution includes several optimizations:

1. **Early Pruning**: Only attempt addition/concatenation if total ≥ first coefficient
   ```rust
   let res_2 = if total >= coeff[0] {
       Self::solve(total - coeff[idx], &coeff[..idx], cop).map(|s| s + coeff[idx])
   } else { None };
   ```

2. **Short-circuit Evaluation**: Return as soon as a valid solution is found
   ```rust
   match (res_1 == Some(total), res_2 == Some(total), res_3 == Some(total)) {
       (true, _, _) => res_1,
       (_, true, _) => res_2,
       (_, _, true) => res_3,
       _ => None,
   }
   ```

3. **Memory Efficiency**: Using slices of the coefficient array rather than creating new arrays in each recursive call
   ```rust
   Self::solve(total / coeff[idx], &coeff[..idx], cop)
   ```

## 6. Public Interface: Simplified Usage

The `Equation` struct provides a clean public interface through the `solver` method:

```rust
pub(crate) fn solver(&self, cop: bool) -> Option<u64> {
    Self::solve(self.result, &self.coeff, cop)
}
```

This design:
1. Encapsulates implementation details
2. Allows toggling concatenation operations via the `cop` parameter
3. Maintains a consistent interface regardless of internal algorithm changes

## 7. Main Program: Processing Input and Solving

The main program ties everything together:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day7/input.txt").expect("msg");
    let equations = input.lines()
        .map(|line| line.parse::<Equation>().unwrap())
        .collect::<Vec<_>>();

    let t = Instant::now();
    let sum = equations.iter()
        .filter_map(|eq| eq.solver(false))
        .sum::<u64>();
    println!("Part 1: total calibration result is {sum} - {:?}", t.elapsed());

    let t = Instant::now();
    let sum = equations.iter()
        .filter_map(|eq| eq.solver(true))
        .sum::<u64>();
    println!("Part 2: total calibration result with CompOp is {sum} - {:?}", t.elapsed());
}
```

The program:
1. Reads the input file and parses each line into an `Equation`
2. Solves each equation without concatenation for Part 1
3. Solves each equation with concatenation for Part 2
4. Measures and displays performance timing for each part

## 8. Testing: Ensuring Correctness

The implementation includes unit tests for the parser to validate it handles various input formats:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        assert!("190: 10 19".parse::<Equation>().is_ok());
        assert!("3267: 81 40 27".parse::<Equation>().is_ok());
        assert!("83:17 5".parse::<Equation>().is_ok());
        assert!("83 :17 5".parse::<Equation>().is_ok());
        assert!("83   :    17     5".parse::<Equation>().is_ok());
        assert!("83 : ".parse::<Equation>().is_err());
        assert!("363816188802: 5 601 3 603 2 2 93 6 3 5".parse::<Equation>().is_ok());
    }
}
```

This testing ensures the parser is robust against different spacing patterns and input variations.

## Conclusion

This equation solver demonstrates several advanced programming concepts:
1. **Recursive Backtracking**: Exploring a solution space by trying different operations
2. **Functional Programming**: Using map/filter operations on collections
3. **Parser Combinators**: Leveraging nom for declarative, maintainable parsing
4. **Memory Optimization**: Using reference counting and slices to minimize copying
5. **Error Handling**: Proper propagation of parsing errors

The program effectively solves a challenging problem by breaking it down into manageable components, each with clear responsibilities and interfaces. The recursive approach elegantly handles the combinatorial complexity of trying different operations to reach the target result.
