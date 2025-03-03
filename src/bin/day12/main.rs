mod segment;
mod plot;
mod garden;


use plot::{area, perimeter, _display_plot};
use garden::parse_garden;//, _display_garden};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").unwrap();

    let garden = parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(_, plot)|
            _display_plot(plot)
        )
        .map(|(_,v)|
            (area(v), perimeter(v))
        )
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    // _display_garden(&garden);
    println!("Garden total cost : {total}");
}
