mod segment;
mod plot;
mod garden;
mod parser;

use std::time;
use garden::Garden;

fn main() {
    let args = std::env::args();
    let input = std::fs::read_to_string(
        match args.skip(1).next() {
            None => "src/bin/day12/input.txt".to_string(),
            Some(str) => str,
        }
    ).unwrap();

    let garden = Garden::parse(&input);

    let t = time::Instant::now();
    let total = garden
        .iter()
        // .inspect(|(id, plot)| println!("ID:{id}\n{plot:?}"))
        .map(|(_,v)|
            (v.area(), v.perimeter())
        )
        .map(|(a,b)| {
            // println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    let el_puzzle = t.elapsed();
    println!("{:?}", &garden);
    let el_debug = t.elapsed();
    println!("Garden total cost : {total} = {el_puzzle:?}");
    println!("Rendered Garden in {el_debug:?}");
    assert_eq!(total, 1533024)
}
