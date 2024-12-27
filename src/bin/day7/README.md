### Challenge Description

The challenge involves parsing and solving a series of equations from an input file. Each equation consists of a result and a list of coefficients. The goal is to determine if the coefficients can be combined (through multiplication, addition, or concatenation) to produce the given result. The solution needs to handle two parts:
1. Solving the equations using only multiplication and addition.
2. Solving the equations using multiplication, addition, and concatenation operations.

### Type of Problem

This problem is a combination of parsing and recursive problem-solving. The parsing part involves reading and interpreting the input data, while the recursive part involves exploring different combinations of operations to match the given result.

### Solution Approach

The solution is implemented in Rust and involves the following steps:

1. **Parsing the Input:**
   - The input is read from a file and each line is parsed into an `Equation` struct.
   - The `Equation` struct contains the result and a list of coefficients.

2. **Defining the Equation Struct:**
   - The `Equation` struct holds the result and coefficients.
   - The `solver` method is defined to solve the equation using the specified operations.

3. **Recursive Solving:**
   - The `solve` function is a recursive function that attempts to match the result using the coefficients.
   - It explores three operations: multiplication, addition, and concatenation (if enabled).

4. **Main Function:**
   - The main function reads the input, parses the equations, and solves them for both parts of the challenge.
   - It measures the time taken for each part and prints the results.

### Detailed Implementation

#### Parsing the Input

The input is read from a file and each line is parsed into an `Equation` struct using the `from_str` method and the `parse_equation` function.

```rust
fn parse_equation(s: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
            u64,
            tuple((space0, tag(":"))),
            tuple((space0, separated_list1(space1, u64)))
        ),
        |(result, (_, coeff))| Equation { result, coeff: coeff.into() }
    )(s)
}
```

#### Defining the Equation Struct

The `Equation` struct holds the result and coefficients. The `solver` method is defined to solve the equation with or without the `concatenation` operator.

```rust
#[derive(Debug)]
pub(crate) struct Equation {
    result: u64,
    coeff: Rc<[u64]>
}

impl Equation {
    pub(crate) fn solver(&self, cop: bool) -> Option<u64> {
        Self::solve(self.result, &self.coeff, cop)
    }
}
```

#### Recursive Solving

The `solve` function is a recursive function that attempts to match the result given the coefficients. It explores three operations: multiplication, addition, and concatenation (if enabled).

```rust
impl Equation {
    fn solve(total: u64, coeff: &[u64], cop: bool) -> Option<u64> {
        fn ct(a: u64, b: u64) -> u64 { format!("{}{}", a, b).parse::<u64>().unwrap() }

        let idx = coeff.len() - 1;
        if idx == 0 { return Some(coeff[idx]) }

        let res_1 = Self::solve(total / coeff[idx], &coeff[..idx], cop).map(|s| s * coeff[idx]);
        let res_2 = if total >= coeff[0] {
            Self::solve(total - coeff[idx], &coeff[..idx], cop).map(|s| s + coeff[idx])
        } else { None };
        let res_3 = if cop && total >= coeff[0] {
            Self::solve((total - coeff[idx]) / 10u64.pow(coeff[idx].ilog10() + 1), &coeff[..idx], cop).map(|s| ct(s, coeff[idx]))
        } else { None };

        match (res_1 == Some(total), res_2 == Some(total), res_3 == Some(total)) {
            (true, _, _) => res_1,
            (_, true, _) => res_2,
            (_, _, true) => res_3,
            _ => None,
        }
    }
}
```

The `solve` function works as follows:
1. **Base Case:** If there is only one coefficient, return it as the solution.
2. **Recursive Case:** For each coefficient, attempt to:
   - **Multiply:** Check if dividing the total by the coefficient and then multiplying back matches the total.
   - **Add:** Check if subtracting the coefficient from the total and then adding back matches the total.
   - **Concatenate (if enabled):** Check if concatenating the coefficient to the result of a recursive call matches the total.
3. **Match Results:** Return the first successful match or `None` if no match is found.

#### Main Function

The main function reads the input, parses the equations, and solves them for both parts of the challenge. It measures the time taken for each part and prints the results.

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
    assert_eq!(12553187650171, sum);

    let t = Instant::now();
    let sum = equations.iter()
        .filter_map(|eq| eq.solver(true))
        .sum::<u64>();
    println!("Part 2: total calibration result with CompOp is {sum} - {:?}", t.elapsed());
    assert_eq!(96779702119491, sum);
}
```
