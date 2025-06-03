use std::{ops::Range,fmt::Debug};

pub type Seed = u16;

// single line description of a plot, capturing a range's line position
#[derive(Default, Eq, PartialEq, Clone)]
pub(super) struct PlotSegment(char, Range<Seed>);

impl PlotSegment {
    pub(super) fn _contains(&self, seed: Seed) -> bool {
        self.1.contains(&seed)
    }
    pub(super) fn new(plant: char, range: Range<Seed>) -> Self {
        PlotSegment(plant, range)
    }
    pub(super) fn plant(&self) -> char {
        self.0
    }
    pub(super) fn start(&self) -> Seed {
        self.1.start
    }
    pub(super) fn end(&self) -> Seed {
        self.1.end
    }
    pub(super) fn len(&self) -> Seed {
        self.1.end as Seed - self.1.start as Seed
    }
    pub(super) fn is_overlapping(&self, other: &Self) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }
    pub(super) fn get_overlap(&self, other: &Self) -> Seed {
        // find the absolute overlap between the two segments
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        end - start
    }
    pub(super) fn count_horizontal_edges<'a>(&self, row_segs: impl Iterator<Item = &'a (usize, PlotSegment)>) -> usize {
        row_segs
            .take_while(|(_,nseg)| nseg.1.start < self.1.end)
            .filter(|(_,nseg)| nseg.is_overlapping(self))
            .map(|(_,nseg)| nseg.get_overlap(self) as usize)
            .sum::<usize>()
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
        self.1.start.cmp(&other.1.start)
            .then_with(|| self.1.end.cmp(&other.1.end))
    }
}

impl Debug for PlotSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.0, self.1)
    }
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
        let seg6 = PlotSegment('R', 2..6);
        assert!(seg1.is_overlapping(&seg1));
        assert!(seg1.is_overlapping(&seg2));
        assert!(seg1.is_overlapping(&seg3));
        assert!(!seg2.is_overlapping(&seg3));
        assert!(!seg3.is_overlapping(&seg2));
        assert!(!seg1.is_overlapping(&seg4));
        assert!(!seg1.is_overlapping(&seg5));
        assert!(seg1.is_overlapping(&seg6));
    }
}
