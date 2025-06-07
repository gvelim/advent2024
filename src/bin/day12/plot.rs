use std::{collections::{BTreeSet, HashSet}, fmt::Debug, ops::RangeInclusive, usize};
use itertools::Itertools;

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

    pub(super) fn perimeter_count(&self) -> usize {
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
                    self.rows.range((usize::MAX-1, west_bound.clone())..=(usize::MAX, east_bound.clone())),
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

    pub(crate) fn sides_count(&self) -> usize {
        let (west, east) = self.get_plot_bounding_segs();
        let start = self.rows.first().expect("Plot Empty!").0;
        // reuse HashSet across iterations so to avoid heap allocations overhead
        let mut corners = HashSet::<u16>::with_capacity(10);

        // number of sides == number of corners
        // 1 ..XXX.. <- Seg A
        // 2 .XXX... <- Seg B
        // a corner is formed between two segments on vertically adjucent lines; current_line and last_line (above)
        // when seg_a.start != seg_b.start OR seg_a.end != seg_b.end
        // therefore for
        // current line = 1 -> last_line is empty hence count = 2 corners
        // current line = 2 -> last_line has ..XXX.. hence count = 4 corners
        // current line = OUT OF BOUNDS -> last_line has .XXX... hence count = 2 corners
        // total corners = 8
        let (last_line, _, sum) = self.get_plot_y_range()
            .fold((
                self.rows.range((usize::MAX,west.clone())..(usize::MAX,east.clone())), // line above
                self.rows.range((start,west.clone())..(start,east.clone())), // current line
                0,  // accumulator : total number of corners
            ),
            |(last_line, current_line, sum), y|
            {
                // clear corners HashMap
                corners.clear();
                // we count all unique corners that are formed between 2 lines
                last_line
                    .chain(current_line.clone())
                    // `*10`, and `*10 - 1` in order to handle edge cases like this below
                    // ..XXXXXX...
                    // XXX...XXX.. <- end() = 3
                    // XX.X..XX... <- start() = 3 MUST not be processed as coinciding with above end()
                    // X..XXXX....
                    // by offseting all end() by -1 we eliminate such cases
                    .flat_map(|(_,s)| [s.start()*10, s.end()*10 - 1])
                    .for_each(|p| {
                        if !corners.insert(p) { // have we seen this corner position before ?
                            corners.remove(&p); // remove position as it matches one above
                        }
                    });
                (
                    current_line,   // current_line becomes last_line
                    self.rows.range((y+1, west.clone())..(y+1,east.clone())),  // next line becomes current_line
                    sum + corners.len()     // count non-overlapping / unique corners that have been seen once
                )
            });
        // add 2 corners for each bottom line segment
        sum + last_line.count() * 2
    }
}

impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        const SPACE_ANSI : &str = "\x1B[38;2;128;128;128;48;2;16;16;128m";
        const PLANT_ANSI : &str = "\x1B[38;2;255;255;0;48;2;16;16;128m";
        // use a line buffer to render the output
        let mut buffer = String::with_capacity(200);

        // capture plot's left & right bounds
        let (left, right) = self.rows
          .iter()
          .fold((Seed::MAX, Seed::MIN), |(left,right), (_, seg)|
              (left.min(seg.start()), right.max(seg.end()))
          );

        // create tmp buffer to store the ranges per line of segments
        let mut segs = Vec::with_capacity(20);

        // given all segments are ordered by 'y' and 'seg.start'
        // it is easy and cheap to iterate per line; we chunk by 'y'
        for (y, line_segments) in &self.rows.iter().chunk_by(|(y,_)| *y) {
          let mut ptr = left;
          segs.clear();
          buffer.clear();

          write!(&mut buffer, "{y:<3} ")?;
          for (_, seg) in line_segments {
            // every segment is prefixed with 0..* '.' starting from 'ptr'
            write!(&mut buffer, "{SPACE_ANSI}")?;
            for _ in ptr..seg.start() { write!(&mut buffer, ".")? }
            // write the segment
            write!(&mut buffer, "{PLANT_ANSI}")?;
            for _ in seg.start()..seg.end() { write!(&mut buffer, "{}", seg.plant())? }
            // capture new start position of '.'
            ptr = seg.end();
            // save segment for display
            segs.push(seg.start()..seg.end());
          }

          // every line finishes with 0..* '.' starting from 'ptr'
          write!(&mut buffer, "{SPACE_ANSI}")?;
          for _ in ptr..right { write!(&mut buffer, ".")? }
          write!(&mut buffer, "\x1B[0m")?;

          // write buffer to output
          write!(f, "{buffer} = " )?;
          // display the ranges of all the line segments drawn
          f.debug_list().entries(segs.iter()).finish()?;
          writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::parser::parse_plots;

    #[test]
    fn test_count_corners() {
        let runs = [
            (
                "src/bin/day12/sample.txt",
                HashMap::from([
                    (4, 10),
                    (0, 10),
                    (5, 12),
                    (8, 16),
                    (1, 4),
                    (3, 12),
                    (2, 22),
                    (6, 4),
                    (9, 6),
                    (10, 6),
                    (7, 8),
                ])
            ), (
                "src/bin/day12/sample4.txt",
                HashMap::from([
                    (3, 6),
                    (0, 10),
                    (2, 6),
                    (5, 12),
                    (7, 10),
                    (10, 8),
                    (12, 6),
                    (13, 4),
                    (1, 50),
                    (8, 8),
                    (9, 4),
                    (11, 4)
                ])
            )
        ];

        for (file, results) in runs {
            println!("\nRUN ==========");
            let plots = parse_plots(
                &std::fs::read_to_string(file).expect("cannot read file")
            );

            for (id, plot) in plots {
                println!("ID:{id}");
                print!("{plot:?}");
                println!(" No of Sides = {}\n", plot.sides_count());
                assert_eq!(plot.sides_count(), results[&id]);
            }
        }
    }
}
