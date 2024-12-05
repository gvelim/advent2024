
mod location;
mod field;



use field::Field;
use location::Location;

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    let start = Location(4, 1);
    let dir = (-1,0);
    println!("{:?}",
             "XMAS"
                 .chars()
                 .enumerate()
                 .filter_map(|(i,c)|
                     start
                         .move_by((dir.0 * i as isize, dir.1 * i as isize))
                         .inspect(|p| print!("{:?},", (p.0,p.1,c)))
                         .map(|p| (p,c))
                 )
                 .all(|(p, c)|
                     field
                         .get_pos(p)
                         .map(|val| val == c)
                         .unwrap_or(false)
                 )
    )
}