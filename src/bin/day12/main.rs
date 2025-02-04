use std::{collections::{BTreeMap, BTreeSet, HashMap}, fmt::Debug, ops::Range};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden: HashMap::<char,Plot> = parse_garden(&input);

    garden.iter()
        .for_each(|(k,v)|
            println!("{}: {:?} = {}", k, v, v.area())
        );
}

// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
fn parse_garden(input: &str) -> HashMap<char,Plot> {
    input
        .lines()
        .map(plot_ranges)
        .enumerate()
        .fold(HashMap::new(), |mut map, (idx,prng)| {
            prng//.into_group_map()
                .into_iter()
                .all(|(plot_name, range)| {
                    map.entry(plot_name)
                        .or_default()
                        .append(PlotSegment(idx, (plot_name, range)));
                    true
                });
            map
        })
}

// single line description of a plot, capturing a range's line position
#[derive(Default,Eq, PartialEq)]
struct PlotSegment(usize, Segment);

impl PlotSegment {
    fn is_overlaping(&self, other: &Self) -> bool {
        self.1.1.start < other.1.1.end && self.1.1.end > other.1.1.start
    }
}

impl PartialOrd for PlotSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Order by line then by range start position
// required for BTreeSet to keep the segments sorted by line location
impl Ord for PlotSegment  {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Equal => match self.1.1.start.cmp(&other.1.1.start) {
                std::cmp::Ordering::Equal => self.1.1.end.cmp(&other.1.1.end),
                res => res
            } ,
            res => res
        }
    }
}

impl Debug for PlotSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.0, self.1)
    }
}

// Plot structure holds collection of overlapping vertical segments
// e.g. "RRRRIICCFF\nRRRRIICCCF" has 4 plots of 2 scanlines each
// ('R', [0..4,0..4]), ('I', [4..6,4..6]), ('C', [6..8,6..9)], ('F', [8..10,9..10])
#[derive(Debug,Default)]
struct Plot {
    id: u8,
    plant: char,
    rows: BTreeSet<PlotSegment>,
}

impl Plot {
    /// Append a segment to the plot as long as it matches the plant type
    fn append(&mut self, seg: PlotSegment) -> bool {
        if self.plant != seg.1.0 { return false };
        self.is_overlapping(&seg) && self.rows.insert(seg)
    }
    fn is_overlapping(&self, seg: &PlotSegment) -> bool {
        self.rows.last()
            .map(|last| last.is_overlaping(seg))
            .unwrap_or(false)
    }
    fn area(&self) -> usize {
        self.rows.iter()
            .map(|seg| seg.1.1.len())
            .sum::<usize>()
    }
}

// Segment covers the start/end position of a plot at a given line
type Segment = (char, Range<u8>);

// given a line RRRRIICCFF
// will return ('R', 0..4), ('I', 4..6), ('C', 6..8), ('F', 8..10)
fn plot_ranges(line: &str) -> impl Iterator<Item = Segment> {
    let mut idx = 0;
    line.as_bytes()
        .chunk_by(|a,b| a == b)
        .map(move |chunk| {
            let start = idx;
            idx += chunk.len() as u8;
            (chunk[0] as char, start..idx)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_append_segment() {
        let mut plot = Plot { id: 1, plant: 'R', rows: BTreeSet::from_iter([PlotSegment(0,('R',0..2))]) };
        let seg1 = PlotSegment(0, ('R', 0..5));
        let seg2 = PlotSegment(1, ('R', 4..6));
        let seg3 = PlotSegment(1, ('R', 6..9));
        let seg4 = PlotSegment(1, ('R', 8..10));
        assert!(plot.append(seg1), "{:?}",plot);
        assert!(plot.append(seg2), "{:?}",plot);
        assert!(!plot.append(seg3), "{:?}",plot);
        assert!(!plot.append(seg4), "{:?}",plot);
    }

    #[test]
    fn test_plotsegment_overlap() {
        let seg1 = PlotSegment(0, ('R', 2..4));
        let seg2 = PlotSegment(1, ('R', 3..6));
        let seg3 = PlotSegment(1, ('R', 0..3));
        let seg4 = PlotSegment(1, ('R', 4..6));
        let seg5 = PlotSegment(1, ('R', 0..2));
        assert!(seg1.is_overlaping(&seg1));
        assert!(seg1.is_overlaping(&seg2));
        assert!(seg1.is_overlaping(&seg3));
        assert!(!seg2.is_overlaping(&seg3));
        assert!(!seg3.is_overlaping(&seg2));
        assert!(!seg1.is_overlaping(&seg4));
        assert!(!seg1.is_overlaping(&seg5));
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
}
