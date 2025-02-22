use std::{ops::Range,fmt::Debug};

// single line description of a plot, capturing a range's line position
#[derive(Default, Eq, PartialEq, Clone)]
pub(super) struct PlotSegment(char, Range<u8>);

impl PlotSegment {
    pub(super) fn new(plant: char, range: Range<u8>) -> Self {
        PlotSegment(plant, range)
    }
    pub(super) fn plant(&self) -> char {
        self.0
    }
    pub(super) fn start(&self) -> u8 {
        self.1.start
    }
    pub(super) fn end(&self) -> u8 {
        self.1.end
    }
    pub(super) fn len(&self) -> usize {
        self.1.end as usize - self.1.start as usize
    }
    pub(super) fn is_overlapping(&self, other: &Self) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }
    pub(super) fn get_overlap(&self, other: &Self) -> u8 {
        // find the absolute overlap between the two segments
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        end - start
    }
}

impl PartialOrd for PlotSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Order by line then by range start position
// required for BTreeSet to keep the segments sorted by line location
impl Ord for PlotSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .cmp(&other.0)
            .then_with(|| self.1.start.cmp(&other.1.start))
            .then_with(|| self.1.end.cmp(&other.1.end))
    }
}

impl Debug for PlotSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.0, self.1)
    }
}

// given a line RRRRIICCFF
// will return ('R', 0..4), ('I', 4..6), ('C', 6..8), ('F', 8..10)
pub(super) fn extract_ranges(line: &str) -> impl Iterator<Item = PlotSegment> {
    let mut idx = 0;
    line.as_bytes()
        .chunk_by(|a, b| a == b)
        .map(move |chunk| {
            let start = idx;
            idx += chunk.len() as u8;
            PlotSegment(chunk[0] as char, start..idx)
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plotsegment_overlap() {
        let seg1 = PlotSegment('R', 2..4);
        let seg2 = PlotSegment('R', 3..6);
        let seg3 = PlotSegment('R', 0..3);
        let seg4 = PlotSegment('R', 4..6);
        let seg5 = PlotSegment('R', 0..2);
        assert!(seg1.is_overlapping(&seg1));
        assert!(seg1.is_overlapping(&seg2));
        assert!(seg1.is_overlapping(&seg3));
        assert!(!seg2.is_overlapping(&seg3));
        assert!(!seg3.is_overlapping(&seg2));
        assert!(!seg1.is_overlapping(&seg4));
        assert!(!seg1.is_overlapping(&seg5));
    }

    #[test]
    fn test_scan_line() {
        let line = "RRRRIICCFF";
        let mut iter = extract_ranges(line);
        assert_eq!(iter.next(), Some(PlotSegment('R', 0u8..4)));
        assert_eq!(iter.next(), Some(PlotSegment('I', 4u8..6)));
        assert_eq!(iter.next(), Some(PlotSegment('C', 6u8..8)));
        assert_eq!(iter.next(), Some(PlotSegment('F', 8u8..10)));
        assert_eq!(iter.next(), None);
    }
}
