
mod location;
mod field;

use std::time::Instant;
use field::Field;
use location::{Location,Direction};

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");
    let (height, width) = (field.height(), field.width());

    let t = Instant::now();
    let scanner = find_at_location(&field);
    let sum = (0..width).map(|x|
        (0..height).map(|y|
            scanner("XMAS", Location(x,y), &[(1,0),(0,1),(1,1),(1,-1),(-1,0),(0,-1),(-1,-1),(-1,1)])
                // .inspect(|r| println!("{:?}",r))
                .count()
        )
        .sum::<usize>()
    )
    .sum::<usize>();
    println!("Part 1: Found ({sum}) XMAS words - {:?}",t.elapsed());
    assert_eq!(2603,sum);

    let t = Instant::now();
    let sum = (0..height).map(|y|
        (0..width)
            .filter(|&x|
                (find_at_location(&field)("MAS",Location(x,y),&[(1,1)]).count() == 1 ||
                    find_at_location(&field)("SAM",Location(x,y),&[(1,1)]).count() == 1) &&
                    (find_at_location(&field)("MAS",Location(x,y+2),&[(1,-1)]).count() == 1 ||
                        find_at_location(&field)("SAM",Location(x,y+2),&[(1,-1)]).count() == 1)
            )
            // .inspect(|r| println!("Found at location: {:?}",(r,y)))
            .count()
        )
        .sum::<usize>();
    println!("Part 2: Found ({sum}) MAS crosses - {:?}",t.elapsed());
    assert_eq!(1965,sum-1);
}

fn find_at_location<'a>(field: &'a Field<char>) -> impl Fn(&'a str, Location, &'a [Direction]) -> Box<dyn Iterator<Item=(Location,Direction)> + 'a> {
    move |word: &'a str, loc: Location, dirs: &[Direction]| Box::new(
        dirs
        .iter()
        .copied()
        .filter(move |&d| find_in_direction(field, word, loc, d))
        .map(move |dir| (loc,dir))
    )
}

fn find_in_direction(field: &Field<char>, word: &str, start: Location, dir: Direction) -> bool {
    word.char_indices()
        // calculate new location based on (a) current index (b) starting position & (c) direction
        .map(|(i,c)| start
            .move_relative((dir.0 * i as isize, dir.1 * i as isize))
            .map(|p| (p,c))
        )
        // for offset location, check for matching letter
        .all(|val|
            val.map(|(p,c)| field
                .get_pos(p)
                .map(|val| val == c)
                .unwrap_or(false)
            ).unwrap_or(false)
        )
}


#[test]
fn test_scan_for_xmas() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    assert_eq!(true, find_in_direction(&field, "XMAS", Location(9, 9), (-1, -1)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(8, 9), (-1, -1)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(7, 9), (-1, -1)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(6, 9), (-1, -1)));
    assert_eq!(true, find_in_direction(&field, "XMAS", Location(5, 9), (-1, -1)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(4, 9), (-1, -1)));
    assert_eq!(true, find_in_direction(&field, "XMAS", Location(3, 9), (-1, -1)));

    assert_eq!(true, find_in_direction(&field, "XMAS", Location(9, 9), (1, 0)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(8, 9), (1, 0)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(7, 9), (1, 0)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(6, 9), (1, 0)));
    assert_eq!(true, find_in_direction(&field, "XMAS", Location(5, 9), (1, 0)));
    assert_eq!(false, find_in_direction(&field, "XMAS", Location(4, 9), (1, 0)));
    assert_eq!(true, find_in_direction(&field, "XMAS", Location(3, 9), (1, 0)));
}
