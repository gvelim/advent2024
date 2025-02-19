mod segment;

use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug
};

use segment::{extract_ranges, PlotSegment};

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
fn parse_garden(input: &str) -> HashMap<usize, BTreeSet<(usize,PlotSegment)>> {
    // parsing logic is as follows:
    // set active map structures 1 & 2; holding (K,V) as (active segment, ID)
    let mut actseg1: Vec<(PlotSegment, usize)> = Vec::new();
    let mut actseg2: Vec<(PlotSegment, usize)> = Vec::new();
    // set garden map structure; holding (K,V) as (ID, Vec<Segment>)
    let mut garden = HashMap::new();
    // for each line of plant segments(plant type, range)
    input.lines()
        .map(extract_ranges)
        .enumerate()
        .fold(garden, |mut g, (idx, segments)| {
            let mut segments = segments.enumerate();
            // for each plant segment
            while let Some((id, segment)) = segments.next() {
                // find all active segments in map 1 that (a) overlap with && (b) have same plant type
                let mut matched = actseg1
                    .iter()
                    .filter(|(seg, _)| seg.plant() == segment.plant() && seg.is_overlapping(&segment))
                    .collect::<Vec<_>>();
                // if empty, then push a new (K,V) (segment, ID) into active map collection 2 and process next segment
                if matched.is_empty() {
                    actseg2.push((segment, idx+id));
                } else {
                    // push new segment to active map with same ID
                    actseg2.push((segment, idx+id));
                    // pop active segment(s) and push into garden map using same ID and line number
                    while let Some(aseg) = matched.pop() {
                        g.entry(aseg.1).or_insert(BTreeSet::new()).insert((idx, aseg.0));
                    }
                }
            }
            // Move remaining unmatched active segments to the garden map using same ID and line number
            while let Some(aseg) = actseg1.pop() {
                g.entry(aseg.1).or_insert(BTreeSet::new()).insert((idx, aseg.0));
            }
            // swap active map 1 with active map 2, so map 2 is the new active map
            std::mem::swap(&mut actseg1, &mut actseg2);
            g
        })
    // return garden map
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
