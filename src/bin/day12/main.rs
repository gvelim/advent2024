use std::ops::Range;

fn main() {
    todo!()
}

// given a line RRRRIICCFF
// will return ('R', 0..=3), ('I', 4..=5), ('C', 6..=7), ('F', 8..=9)
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
