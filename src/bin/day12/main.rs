mod segment;

use std::{
    collections::BTreeSet,
    fmt::Debug
};

use segment::PlotSegment;

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden: Vec<Plot> = parse_garden(&input);

    garden
        .iter()
        .for_each(|v| println!("{:?} = {}", v, v.area()));
}

// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
fn parse_garden(input: &str) -> Vec<Plot> {
    // parsing logic is as follows:
    // set active map structures 1 & 2; holding (K,V) as (active segment, ID)
    // set garden map structure; holding (K,V) as (ID, Vec<Segment>)
    // for each line of plant segments(plant type, range)
    // for each plant segment
    // does it match (overlapping range & plant type) any active segment in map 1
    // if not, then push a new (K,V) (segment, ID) into active map collection 2
    // if yes, then
    // pop active segment(s) and push into garden map using same ID
    // push new segment to active mapusing same ID
    // Check, are there any map 1 active segments left without match ?
    // if yes, then move them into garden map against same ID
    // swap active map 1 with active map 2, so map 2 is the new active map
    //

    Vec::new()
}


// Plot structure holds collection of overlapping vertical segments
// e.g. "RRRRIICCFF\nRRRRIICCCF" has 4 plots of 2 scanlines each
// ('R', [0..4,0..4]), ('I', [4..6,4..6]), ('C', [6..8,6..9)], ('F', [8..10,9..10])
#[derive(Debug)]
struct Plot {
    plant: char,
    rows: BTreeSet<(usize, PlotSegment)>,
}
impl Plot {
    fn new(plant: char) -> Self {
        Plot {
            plant,
            rows: BTreeSet::new(),
        }
    }
    /// Append a segment to the plot as long as it matches the plant type
    fn append(&mut self, line: usize, seg: PlotSegment) -> bool {
        if self.plant == seg.plant() {
            self.rows.insert((line, seg))
        } else {
            false
        }
    }
    fn is_overlapping(&self, seg: &PlotSegment) -> bool {
        let start = PlotSegment(seg.plant(), 0 .. 1);
        let end = PlotSegment(seg.plant(), u8::MAX-1 .. u8::MAX);
        println!("Cmp: {:?}, {:?}, {:?}",seg, start, end);
        self.rows
            .range(start ..= end)
            .inspect(|p| println!("iter: {:?}",p))
            .any(|last| last.1.is_overlapping(seg))
    }
    fn area(&self) -> usize {
        self.rows.iter().map(|seg| seg.1.len()).sum::<usize>()
    }
}

impl PartialOrd for Plot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.plant.cmp(&other.plant))
    }
}

impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.plant == other.plant
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_overlap() {
        let mut plot = Plot::new('R');
        let seg1 = PlotSegment(0, ('R', 0..4));
        let seg2 = PlotSegment(0, ('R', 5..8));
        let seg3 = PlotSegment(1, ('R', 2..6));
        let seg4 = PlotSegment(2, ('R', 6..9));
        let seg5 = PlotSegment(3, ('R', 5..10));

        assert!(plot.append(seg1));
        assert!(!plot.is_overlapping(&seg2));
        plot.append(seg2);
        assert!(plot.is_overlapping(&seg3));
        plot.append(seg3);
        assert!(!plot.is_overlapping(&seg4));
        assert!(!plot.is_overlapping(&seg5));
    }

    #[test]
    fn test_plot_append_segment() {
        let seg1 = PlotSegment(1, ('R', 0..5));
        let seg2 = PlotSegment(2, ('R', 4..6));
        let seg3 = PlotSegment(3, ('R', 6..9));
        let seg4 = PlotSegment(4, ('A', 5..10));

        let mut plot = Plot {plant: 'R', rows: BTreeSet::new()};
        plot.append(PlotSegment(0,('R',0..2)));
        assert!(plot.append(seg1), "{:?}", plot);
        assert!(plot.append(seg2), "{:?}", plot);
        assert!(plot.append(seg3), "{:?}", plot);
        assert!(!plot.append(seg4), "{:?}", plot);
        println!("{:?}",plot);
    }
}
