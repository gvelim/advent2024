use std::{collections::BTreeSet, fmt::Debug, ops::RangeInclusive};
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

        self.edge_count_north_south(y_range.clone())
            // a row may contain 1 or more segments of the same plot with gaps in between
            // plot segments in the same raw are *isolated*, that is, they are never next to each other, end of first != start of second
            // therefore vertical segments per row is 2 * number of segments
            // therefore sum(row) == total segments * 2
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

    fn edge_count_north_south(&self, lines: impl Iterator<Item = usize>) -> usize  {
        let (west_bound, east_bound) = self.get_plot_bounding_segs();

        let mut lines = lines.peekable();
        let Some(&start) = lines.peek() else { panic!("perimeter_counter(): Empty 'y' range")};

        // we fold each iteration using (above, current, below and sum) as input parameters
        // this reduces the number of BTreeSet queries from 3 down to 1 per iteration
        let (_, _, _, sum) = lines
            .fold(
                (
                    self.rows.range((start-1, west_bound.clone())..=(start-1, east_bound.clone())),
                    self.rows.range((start, west_bound.clone())..=(start, east_bound.clone())),
                    self.rows.range((start+1, west_bound.clone())..=(start+1, east_bound.clone())),
                    0
                ),
                |( above_row, current_row, below_row, sum), y| {

            // sum non-overlapping units of current raw against above and below segment lines
            let new_sum = sum + current_row.clone()
                .map(|(_, seg)| {
                    // non-overlapping  = (segment length - above overlaping units) + (segment length - below overlaping units) =>
                    // non-overlapping = 2 * segment lengths - above - below overlapping units
                    2 * seg.len() as usize
                    // count overlapping units above the line
                        - seg.count_horizontal_edges(above_row.clone())
                    // count overlapping units under the line
                        - seg.count_horizontal_edges(below_row.clone())
                })
                .sum::<usize>();

            (
                // contains y becomes y-1 in next cycle
                current_row,
                // contains y+1 becomes y in next cycle
                below_row,
                // we need y+2 so it becomes y+1 in next cycle
                self.rows.range((y+2, west_bound.clone())..=(y+2, east_bound.clone())),
                new_sum
            )
        });
        sum
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
