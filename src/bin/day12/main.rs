use std::{collections::HashMap, ops::Range, rc::Rc};
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden: HashMap::<char,Plot> = parse_garden(&input);

    garden.iter()
        .for_each(|(k,v)|
            println!("{}: {:?} = {}", k, v, v.area())
        );
}

// garden is a collection of scanlines that express plots
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
fn parse_garden(input: &str) -> HashMap<char,Plot> {
    input
        .lines()
        .map(plot_ranges)
        .fold(HashMap::new(), |mut map, prng| {
            prng.into_group_map()
                .into_iter()
                .all(|(plot_name, line_ranges)| {
                    map.entry(plot_name)
                        .or_default()
                        .push(line_ranges);
                    true
                });
            map
        })
}

// Plot structure holds collection of scanlines corresponding to a plot name
// e.g. "RRRRIICCFF\nRRRRIICCCF" has 4 plots of 2 scanlines each
// ('R', [0..4,0..4]), ('I', [4..6,4..6]), ('C', [6..8,6..9)], ('F', [8..10,9..10])
#[derive(Debug,Default)]
struct Plot {
    rows: Vec<Vec<Range<u8>>>,
}

impl Plot {
     fn push(&mut self, ranges: Vec<Range<u8>>) {
         self.rows.push(ranges);
     }
    fn area(&self) -> u32 {
        self.rows
            .iter()
            .map(|ranges|
                ranges.iter().fold(0, |acc,rng| acc + rng.len() as u32)
            )
            .sum::<u32>()
    }
    fn perimeter(&self) -> u32 {
        todo!()
    }
}

// given a line RRRRIICCFF
// will return ('R', 0..4), ('I', 4..6), ('C', 6..8), ('F', 8..10)
fn plot_ranges(line: &str) -> impl Iterator<Item = (char,Range<u8>)> {
    let mut idx = 0;
    line.as_bytes()
        .chunk_by(|a,b| a == b)
        .map(move |chunk| {
            let start = idx;
            idx += chunk.len() as u8;
            (chunk[0] as char, start..idx)
        })
}


#[test]
fn test_scan_line() {
    let line = "RRRRIICCFF";
    let mut iter = plot_ranges(line);
    assert_eq!(iter.next(), Some(('R', 0u8..4)));
    assert_eq!(iter.next(), Some(('I', 4u8..6)));
    assert_eq!(iter.next(), Some(('C', 6u8..8)));
    assert_eq!(iter.next(), Some(('F', 8u8..10)));
    assert_eq!(iter.next(), None);
}
