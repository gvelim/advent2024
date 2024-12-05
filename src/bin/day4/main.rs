
mod location;
mod field;

use field::Field;
use location::Location;

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    let sum = (0..field.length).map(|x| {
        (0..field.cells.len() / field.length).map(|y| {
            let loc = Location(x,y);
            [(1_isize,0_isize),(-1,0),(0,1),(0,-1),(1,1),(-1,1),(-1,-1),(1,-1)]
                .iter()
                .filter(|&dir| scan_for_xmas(&field, "XMAS", loc, *dir))
                // .inspect(|r| println!("{:?}",(r,loc)))
                .count()
        })
        .sum::<usize>()
    })
    .sum::<usize>();

    println!("{sum}")
}

fn scan_for_xmas(field: &Field<char>, word: &str, start: Location, dir: (isize, isize)) -> bool {
    word
        .char_indices()
        // calculate location offset based on index and starting position
        .map(|(i,c)|
            start
                .move_by((dir.0 * i as isize, dir.1 * i as isize))
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

    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(9,9),(-1,-1)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(8,9),(-1,-1)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(7,9),(-1,-1)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(6,9),(-1,-1)));
    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(5,9),(-1,-1)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(4,9),(-1,-1)));
    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(3,9),(-1,-1)));

    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(9,9),(1,0)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(8,9),(1,0)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(7,9),(1,0)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(6,9),(1,0)));
    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(5,9),(1,0)));
    assert_eq!(false, scan_for_xmas(&field, "XMAS", Location(4,9),(1,0)));
    assert_eq!(true, scan_for_xmas(&field, "XMAS", Location(3,9),(1,0)));
}
