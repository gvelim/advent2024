mod segment;
mod plot;
mod garden;
mod parser;

use std::time;
use garden::Garden;
use crate::plot::Plot;

fn main() {
    let args = std::env::args();
    let input = std::fs::read_to_string(
        match args.skip(1).next() {
            None => "src/bin/day12/input.txt".to_string(),
            Some(str) => str,
        }
    ).unwrap();

    let garden = Garden::parse(&input);

    let calculate_cost = |garden: &Garden, fcalc: fn((&usize, &Plot)) -> usize| -> usize {
        garden
        .iter()
        // .inspect(|(id, plot)| print!("ID:{id}\n{plot:?}"))
        // .inspect(|(_, plot)| print!("area: {} * perimeter: {} = ", plot.area(), plot.perimeter()))
        .map(fcalc)
        // .inspect(|res| println!("{res}\n"))
        .sum::<usize>()
    };

    let mut t = time::Instant::now();
    let total_1 = calculate_cost(&garden, |(_, plot)| plot.area() * plot.perimeter_count());
    let el_puzzle_1 = t.elapsed();

    t = time::Instant::now();
    let total_2 = calculate_cost(&garden, |(_, plot)| plot.area() * plot.sides_count());
    let el_puzzle_2 = t.elapsed();

    t = time::Instant::now();
    println!("{:?}", &garden);
    let el_debug = t.elapsed();

    println!("Part 1 - Garden total cost : {total_1} = {el_puzzle_1:?}");
    println!("Part 2 - Garden total cost : {total_2} = {el_puzzle_2:?}");
    println!("Rendered Garden in {el_debug:?}");

    assert_eq!(total_1, 1533024);
    assert_eq!(total_2, 910066);
}
