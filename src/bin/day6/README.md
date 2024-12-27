## Documentation: Solution to Advent2024 Day 6 Challenge

### Overview

The challenge involves navigating a lab represented as a grid, where a guard moves around following specific rules. The goal is to determine the number of unique locations the guard visits and identify loop obstacles within the lab.

### Approach

The solution is divided into two main parts:
1. **Finding the number of unique locations visited by the guard.**
2. **Identifying loop obstacles within the lab.**

### Detailed Steps

#### Part 1: Finding Unique Locations Visited by the Guard

1. **Reading Input:**
   - The input is read from a file (`src/bin/day6/input.txt`) and parsed into a `Lab` structure, which is essentially a grid of characters.

    ```rust
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let mut lab = input.parse::<Lab>().expect("Field parse err");
    ```

2. **Finding the Guard:**
   - The `find_guard` function is used to locate the guard's initial position and direction within the lab. The guard is represented by one of the characters `^`, `>`, `v`, or `<`, indicating the direction they are facing.

    ```rust
    let (pos, dir) = find_guard(&lab, &['^', '>', 'v', '<']).expect("there is no Lab Guard !!");
    ```

3. **Simulating Guard Movement:**
   - A `Guard` struct is created, which holds a reference to the lab, the guard's current position, and direction.
   - The `Guard` struct implements the `Iterator` trait, allowing it to move through the lab according to the rules:
     - The guard turns clockwise if it encounters an obstacle (`'#'`).
     - The guard moves forward if the next position is within the lab's bounds.

    ```rust
    let mut unique_locations = Guard { lab: &lab, pos, dir }.collect::<HashMap<_, _>>();
    unique_locations.insert(pos, dir);
    ```

4. **Output:**
   - The number of unique locations visited by the guard is printed and asserted to be correct.

    ```rust
    println!("Part 1: Guard visited {:?} unique locations - {:?}", unique_locations.len(), t.elapsed());
    assert_eq!(unique_locations.len(), 5534);
    ```

#### Part 2: Identifying Loop Obstacles

1. **Identifying Obstacles:**
   - For each unique location visited by the guard, the solution checks if the location is part of a loop obstacle *by actually placing an interim obstacle* before proceeding with the loop check.
   - A loop obstacle is defined as a position where the guard can return to the same position and direction after a series of moves.

    ```rust
    let obstacles = unique_locations
        .iter()
        .filter(|&(l, _)| {
            path.clear();
            *lab.get_mut(*l).unwrap() = '#';
            let in_loop = Guard { lab: &lab, pos, dir }
                .any(|(nl, nd)| {
                    let in_loop = path.get(&nl).is_some_and(|&pd| nd == pd);
                    path.entry(nl).or_insert(nd);
                    in_loop
                });
            *lab.get_mut(*l).unwrap() = '.';
            in_loop
        })
        .count();
    ```

2. **Output:**
   - The number of loop obstacles is printed and asserted to be correct.

    ```rust
    println!("Part 2: There are {:?} loop obstacles - {:?}", obstacles, t.elapsed());
    assert_eq!(obstacles, 2262);
    ```
