mod segment;
mod plot;
mod garden;

use garden::Garden;

fn main() {
    let args = std::env::args();
    let input = std::fs::read_to_string(
        match args.skip(1).next() {
            None => "src/bin/day12/input.txt".to_string(),
            Some(str) => str,
        }
    ).unwrap();

    let garden = Garden::parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(id, plot)| println!("ID:{id}\n{plot:?}"))
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
