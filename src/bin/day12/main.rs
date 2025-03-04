mod segment;
mod plot;
mod garden;

use garden::Garden;//, _display_garden};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").unwrap();

    let garden = Garden::parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(_, plot)| println!("{:?}", plot))
        .map(|(_,v)|
            (v.area(), v.perimeter())
        )
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    println!("{:?}", &garden);
    println!("Garden total cost : {total}");
}
