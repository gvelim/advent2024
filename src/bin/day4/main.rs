use std::time::Instant;
use advent2024::field::Field;
use advent2024::location::{Location,DirVector};

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");
    let (height, width) = (field.height(), field.width());

    let t = Instant::now();
    // we will scan also the reverse string at the same time hence we need only half directions
    let xmas_scanner = search_directions(&field, &[(1,0),(0,1),(1,1),(1,-1)]);
    let sum = (0..width)
        .map(|x| (0..height)
            .map(|y|
                xmas_scanner("XMAS", Location(x,y)).count()
                + xmas_scanner("SAMX", Location(x,y)).count()
            )
            .sum::<usize>()
        )
        .sum::<usize>();
    println!("Part 1: Found ({sum}) XMAS words - {:?}",t.elapsed());
    assert_eq!(2603,sum);

    let t = Instant::now();
    let mas_leg1_scanner = search_directions(&field, &[(1,1)]);
    let mas_leg2_scanner = search_directions(&field, &[(1,-1)]);
    let sum = (0..height)
        .map(|y| (0..width)
            .filter(|&x|
                (mas_leg1_scanner("MAS",Location(x,y)).count() == 1 ||
                    mas_leg1_scanner("SAM",Location(x,y)).count() == 1) &&
                    (mas_leg2_scanner("MAS",Location(x,y+2)).count() == 1 ||
                        mas_leg2_scanner("SAM",Location(x,y+2)).count() == 1)
            )
            .count()
        )
        .sum::<usize>();
    println!("Part 2: Found ({sum}) MAS crosses - {:?}",t.elapsed());
    assert_eq!(1965,sum);
}

fn search_directions<'a>(field: &'a Field<char>, dirs: &'a [DirVector]) -> impl Fn(&'a str, Location) -> Box<dyn Iterator<Item=(Location,DirVector)> + 'a> {
    // return a function that takes a world and location
    // and performs a scan on field and set of directions that has be constructed with
    move |word: &'a str, pos: Location| {
        let ret = dirs.iter()
            .copied()
            .filter(move |&dir| is_word_matched(field, word, pos, dir))
            .map(move |dir| (pos,dir));
        // iterator must be boxed as it doesn;t compile with "-> impl Iterator"
        Box::new(ret)
    }
}

fn is_word_matched(field: &Field<char>, word: &str, start: Location, dir: DirVector) -> bool {
    word.char_indices()
        .all(|(i,c)| start
            // calculate new location based on (a) current index (b) starting position & (c) direction
            .move_relative((dir.0 * i as isize, dir.1 * i as isize))
            .map(|p| field
                // match the value in position with input's character
                .value_at(p)
                .map(|&val| val == c)
                .unwrap_or(false)
            ).unwrap_or(false)
        )
}


#[test]
fn test_scan_for_xmas() {
    let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
    let field = input.parse::<Field<char>>().expect("Doesn't error");

    assert!(is_word_matched(&field, "XMAS", Location(9, 9), (-1, -1)));
    assert!(!is_word_matched(&field, "XMAS", Location(8, 9), (-1, -1)));
    assert!(!is_word_matched(&field, "XMAS", Location(7, 9), (-1, -1)));
    assert!(!is_word_matched(&field, "XMAS", Location(6, 9), (-1, -1)));
    assert!(is_word_matched(&field, "XMAS", Location(5, 9), (-1, -1)));
    assert!(!is_word_matched(&field, "XMAS", Location(4, 9), (-1, -1)));
    assert!(is_word_matched(&field, "XMAS", Location(3, 9), (-1, -1)));
}
