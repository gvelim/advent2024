mod topographical_map;
use topographical_map::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").unwrap();
    let map = input.parse::<TopographicalMap>().unwrap();

    let sum = map.lowests()
        .filter_map(|start|
            TrailHead::new(true).search_path(&map, start, |d| d == 9)
        )
        .sum::<usize>();
    println!("Part 1: Sum of the scores of all trailheads = {:?}", sum);
    assert_eq!(786,sum);

    let sum = map.lowests()
        .filter_map(|start|
            TrailHead::new(false).search_path(&map, start, |d| d == 9)
        )
        .sum::<usize>();
    println!("Part 2: Sum of the ratings of all unique trailheads = {:?}", sum);
    assert_eq!(1722,sum);
}
