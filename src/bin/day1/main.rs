use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let input = fs::read_to_string("./src/bin/day1/input.txt").expect("File not found");

    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut values = line.split_whitespace();
            (
                values
                    .next()
                    .expect("no values found")
                    .parse::<usize>()
                    .expect("invalid numberic"),
                values
                    .next()
                    .expect("Only one value found")
                    .parse::<usize>()
                    .expect("invalid numberic"),
            )
        })
        .unzip();

    a.sort();
    b.sort();

    let hash_b = b
        .iter()
        .fold(HashMap::new(), |mut map, key| {
            map.entry(key)
                .and_modify(|val| *val += 1)
                .or_insert(1);
            map
        });

    let t = Instant::now();
    println!(
        "Part 1: {} - ({:?})",
        a.iter()
            .zip(b.iter())
            .map(|(x, &y)| x.abs_diff(y))
            .sum::<usize>(),
        t.elapsed()
    );

    let t = Instant::now();
    println!(
        "Part 2: {} - ({:?})",
        a.iter()
            .map(|key| key * hash_b.get(key).unwrap_or(&0))
            .sum::<usize>(),
        t.elapsed()
    );
}
