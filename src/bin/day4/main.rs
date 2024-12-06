
mod location;
mod field;

use field::Field;
use location::{Location,Direction};

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    let sum = (0..field.width()).map(|x| {
        (0..field.height()).map(|y|
            find_at_location(&field, "XMAS", Location(x,y), &XMAS_RD)
                .inspect(|r| println!("{:?}",r))
                .count()
        )
        .sum::<usize>()
    })
    .sum::<usize>();
    println!("{sum}");
}

static XMAS_RD : [Direction;8] = [(1,0),(0,1),(1,1),(1,-1),(-1,0),(0,-1),(-1,-1),(-1,1)];

fn find_at_location(field: &Field<char>, word: &str, loc: Location, dirs: &[Direction]) -> impl Iterator<Item=(Location,Direction)> {
    dirs.iter()
        .copied()
        .filter(move |&d|
            find_at_direction(field, word, loc, d)
        )
        .map(move |dir| (loc,dir))
}

fn find_at_direction(field: &Field<char>, word: &str, start: Location, dir: Direction) -> bool {
    word
        .char_indices()
        // calculate location offset using current index and starting position
        .map(|(i,c)|
            start
                .move_relative((dir.0 * i as isize, dir.1 * i as isize))
                // .inspect(|p| print!("{:?},", (p.0,p.1,c)))
                .map(|p| (p,c))
        )
        // for offset location, check for matching letter
        .all(|val|
            val.map(|(p,c)|
                field
                    .get_pos(p)
                    // .inspect(|p| print!("={:?},", p))
                    .map(|val| val == c)
                    .unwrap_or(false)
            ).unwrap_or(false)
        )
}


#[test]
fn test_scan_for_xmas() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    assert_eq!(true, find_at_direction(&field, "XMAS", Location(9,9),(-1,-1)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(8,9),(-1,-1)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(7,9),(-1,-1)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(6,9),(-1,-1)));
    assert_eq!(true, find_at_direction(&field, "XMAS", Location(5,9),(-1,-1)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(4,9),(-1,-1)));
    assert_eq!(true, find_at_direction(&field, "XMAS", Location(3,9),(-1,-1)));

    assert_eq!(true, find_at_direction(&field, "XMAS", Location(9,9),(1,0)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(8,9),(1,0)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(7,9),(1,0)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(6,9),(1,0)));
    assert_eq!(true, find_at_direction(&field, "XMAS", Location(5,9),(1,0)));
    assert_eq!(false, find_at_direction(&field, "XMAS", Location(4,9),(1,0)));
    assert_eq!(true, find_at_direction(&field, "XMAS", Location(3,9),(1,0)));
}
