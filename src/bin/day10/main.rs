mod topographical_map;
mod trailhead;

use std::time::Instant;
use topographical_map::TopographicalMap;
use trailhead::TrailHead;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").unwrap();
    let map = input.parse::<TopographicalMap>().unwrap();

    let t = Instant::now();
    let sum = map
        .lowests()
        .filter_map(|start| TrailHead::trail_heads().count_trails(&map, start, |d| d == 9))
        .sum::<usize>();
    println!(
        "Part 1: Sum of the scores of all trailheads = {sum} - {:?}",
        t.elapsed()
    );
    assert_eq!(786, sum);

    let t = Instant::now();
    let sum = map
        .lowests()
        .filter_map(|start| TrailHead::unique_trails().count_trails(&map, start, |d| d == 9))
        .sum::<usize>();
    println!(
        "Part 2: Sum of the ratings of all unique trailheads = {sum} - {:?}",
        t.elapsed()
    );
    assert_eq!(1722, sum);
}
