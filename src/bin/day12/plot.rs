use std::{collections::BTreeSet, fmt::Debug, ops::RangeInclusive};
use rayon::iter::{ParallelBridge,ParallelIterator};

use super::segment::{PlotSegment, Seed};

#[derive(Default)]
pub(super) struct Plot {
    rows: BTreeSet<(usize, PlotSegment)>
}

impl Plot {
    pub fn extend(&mut self, plot: Plot) {
        self.rows.extend(plot.rows);
    }
    pub fn iter(&self) -> impl Iterator<Item = &(usize, PlotSegment)> {
        self.rows.iter()
    }
    pub fn insert(&mut self, y: usize, segment: PlotSegment) {
        self.rows.insert((y, segment));
    }
    pub(super) fn area(self: &Plot) -> usize {
        self.rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
    }

    pub(super) fn perimeter(&self) -> usize {
        let y_range = self.get_plot_y_range();

        self.perimeter_counter(y_range.clone())
            // a raw has 1 or more segments
            // therefore all we care is the number of segments
            + self.rows.len() * 2
    }

    fn get_plot_y_range(self: &Plot) -> RangeInclusive<usize> {
        self.rows.first().unwrap().0 ..= self.rows.last().unwrap().0
    }

    pub fn get_plot_bounding_segs(&self) -> (PlotSegment, PlotSegment) {
        let plant = self.rows.first().unwrap().1.plant();
        (
            PlotSegment::new(plant, 0..1),
            PlotSegment::new(plant, Seed::MAX-1..Seed::MAX)
        )
    }

    fn perimeter_counter(&self, range: impl Iterator<Item = usize>) -> usize  {
        let (west_bound, east_bound) = self.get_plot_bounding_segs();

        range
            .map(|y| {
            let current_row = self.rows.range((y, west_bound.clone())..=(y, east_bound.clone()));
            let above_row = self.rows.range((y-1, west_bound.clone())..=(y-1, east_bound.clone()));
            let below_row = self.rows.range((y+1, west_bound.clone())..=(y+1, east_bound.clone()));

            // sum non-overlapping units between current raw vs above and below rows
            current_row
                .map(|(_, seg)| {
                    // count above overlapping units
                    let above = above_row.clone()
                        .filter(|(_,nseg)| nseg.is_overlapping(seg))
                        .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                        .sum::<usize>();
                    // count below overlapping units
                    let below = below_row.clone()
                        .filter(|(_,nseg)| nseg.is_overlapping(seg))
                        .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                        .sum::<usize>();
                    // perimeter = (segment length - above overlaping units) + (segment length - above overlaping units) =>
                    // perimeter = 2 * segment lengths - above - below overlapping units
                    2 * seg.len() as usize - above - below
                })
                .sum::<usize>()
        }).sum::<usize>()
    }
}

impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;

        let (first, last) = (self.rows.first().unwrap().0, self.rows.last().unwrap().0);
        let (left, right) = self.rows
            .iter()
            .fold((Seed::MAX, Seed::MIN), |(left,right), (_, seg)| {
                (left.min(seg.start()), right.max(seg.end()))
            });

        for y in first..=last {
            let (west_bound, east_bound) = self.get_plot_bounding_segs();
            let line_segments = self.rows
                .range((y, west_bound) ..= (y, east_bound))
                .peekable();

            let mut ls_iter = line_segments.clone();
            for x in left..right {
                match ls_iter.peek() {
                    Some((_, seg)) if seg.contains(x) =>
                        write!(f, "{}",
                            String::from(seg.plant()).on_truecolor(16,16,128).bright_yellow()
                        )?,
                    segment => {
                        write!(f, "{}", ".".on_truecolor(16,16,128))?;
                        if let Some((_,seg)) = segment {
                            if x >= seg.end() - 1 {
                                ls_iter.next();
                            }
                        }
                    }
                }
            }
            write!(f, " = " )?;
            f.debug_list().entries(line_segments).finish()?;
            writeln!(f)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::super::Garden;

    #[test]
    fn test_garden_parser() {
        let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();
        let garden = Garden::parse_garden(&input);

        // A region of R plants with price 12 * 18 = 216.
        assert_eq!(garden[&1].area(), 12, "expected 12 got {:?}",garden[&1]);
        assert_eq!(garden[&1].perimeter(), 18, "expected 18 got {:?}",garden[&1]);
        // A region of I plants with price 4 * 8 = 32.
        assert_eq!(garden[&2].area(), 4, "expected 4 got {:?}",garden[&2]);
        assert_eq!(garden[&2].perimeter(), 8, "expected 8 got {:?}",garden[&2]);
        // A region of C plants with price 14 * 28 = 392.
        assert_eq!(garden[&3].area(), 14, "expected 14 got {:?}",garden[&3]);
        assert_eq!(garden[&3].perimeter(), 28, "expected 28 got {:?}",garden[&3]);
        // A region of F plants with price 10 * 18 = 180.
        assert_eq!(garden[&4].area(), 10, "expected 10 got {:?}",garden[&4]);
        assert_eq!(garden[&4].perimeter(), 18, "expected 18 got {:?}",garden[&4]);
        // A region of V plants with price 13 * 20 = 260.
        assert_eq!(garden[&5].area(), 13, "expected 13 got {:?}",garden[&5]);
        assert_eq!(garden[&5].perimeter(), 20, "expected 20 got {:?}",garden[&5]);
        // A region of J plants with price 11 * 20 = 220.
        assert_eq!(garden[&6].area(), 11, "expected 11 got {:?}",garden[&6]);
        assert_eq!(garden[&6].perimeter(), 20, "expected 20 got {:?}",garden[&6]);
        // A region of C plants with price 1 * 4 = 4.
        assert_eq!(garden[&7].area(), 1, "expected 1 got {:?}",garden[&7]);
        assert_eq!(garden[&7].perimeter(), 4, "expected 4 got {:?}",garden[&7]);
        // A region of E plants with price 13 * 18 = 234.
        assert_eq!(garden[&8].area(), 13, "expected 13 got {:?}",garden[&8]);
        assert_eq!(garden[&8].perimeter(), 18, "expected 18 got {:?}",garden[&8]);
        // A region of I plants with price 14 * 22 = 308.
        assert_eq!(garden[&9].area(), 14, "expected 14 got {:?}",garden[&9]);
        assert_eq!(garden[&9].perimeter(), 22, "expected 22 got {:?}",garden[&9]);
        // A region of M plants with price 5 * 12 = 60.
        assert_eq!(garden[&10].area(), 5, "expected 5 got {:?}",garden[&10]);
        assert_eq!(garden[&10].perimeter(), 12, "expected 12 got {:?}",garden[&10]);
        // A region of S plants with price 3 * 8 = 24.
        assert_eq!(garden[&11].area(), 3, "expected 3 got {:?}",garden[&11]);
        assert_eq!(garden[&11].perimeter(), 8, "expected 8 got {:?}",garden[&11]);
    }

}
