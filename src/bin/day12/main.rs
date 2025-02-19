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
            // find all active segments in map 1 that (a) overlap with && (b) have same plant type
            // if empty, then push a new (K,V) (segment, ID) into active map collection 2 and process next segment
            // For each active segment(s) matched
                // pop active segment(s) and push into garden map using same ID and line number
                // push new segment to active map with same ID
        // Move remaining unmatched active segments to the garden map using same ID and line number
        // swap active map 1 with active map 2, so map 2 is the new active map
    // return garden map

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
    fn test_plot_append_segment() {
        let seg1 = PlotSegment::new('R', 0..5);
        let seg2 = PlotSegment::new('R', 4..6);
        let seg3 = PlotSegment::new('R', 6..9);
        let seg4 = PlotSegment::new('A', 5..10);

        let mut plot = Plot::new('R');
        plot.append(0,PlotSegment::new('R',0..2));
        assert!(plot.append(1,seg1), "{:?}", plot);
        assert!(plot.append(2,seg2), "{:?}", plot);
        assert!(plot.append(3,seg3), "{:?}", plot);
        assert!(!plot.append(4,seg4), "{:?}", plot);
        println!("{:?}",plot);
    }
}
