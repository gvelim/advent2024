mod segment;
mod plot;

use plot::{parse_garden, area, perimeter};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").unwrap();

    let garden = parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(id, plot)|
            print!("{id}::{:?} = ", plot)
        )
        .map(|(_,v)|
            (area(v), perimeter(v))
        )
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    println!("Garden total cost : {total}");
}
