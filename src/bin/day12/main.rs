use std::{ops::Range, rc::Rc};

fn main() {
    todo!()
}


// Plot structure holds collection of scanlines corresponding to a plot name
// e.g. "RRRRIICCFF\nRRRRIICCCF" has 4 plots ('R', [0..4,0..4]), ('I', [4..6,4..6]), ('C', [6..8,6..9)], ('F', [8..10,9..10])
struct Plot {
    name: char,
    scanlines: Rc<[Range<u8>]>,
}

impl Plot {
    fn area(&self) -> u32 {
        self.scanlines
            .iter()
            .map(|r| r.len() as u32)
            .sum::<u32>()
    }
    fn perimeter(&self) -> u32 {
        todo!()
    }
}

// given a line RRRRIICCFF
// will return ('R', 0..4), ('I', 4..6), ('C', 6..8), ('F', 8..10)
fn scan_line(line: &str) -> impl Iterator<Item = (char,Range<u8>)> {
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
    let mut iter = scan_line(line);
    assert_eq!(iter.next(), Some(('R', 0u8..4)));
    assert_eq!(iter.next(), Some(('I', 4u8..6)));
    assert_eq!(iter.next(), Some(('C', 6u8..8)));
    assert_eq!(iter.next(), Some(('F', 8u8..10)));
    assert_eq!(iter.next(), None);
}
