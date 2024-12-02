use std::{collections::BinaryHeap, fs};

fn main() {
    let input = fs::read_to_string("./src/bin/day1/input.txt").expect("File not found");

    let (mut a, mut b) = input
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
        .fold(
            (BinaryHeap::<usize>::new(), BinaryHeap::<usize>::new()),
            |(mut a, mut b), (x, y)| {
                a.push(x);
                b.push(y);
                (a, b)
            },
        );

    let mut sum = 0;
    while let Some(x) = a.pop() {
        let y = b.pop().unwrap();
        let diff = x.abs_diff(y);
        println!("{x},{y} = {diff}",);
        sum += diff;
    }
    println!("{sum}");
}
