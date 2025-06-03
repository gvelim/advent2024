use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Index;
use super::segment::{PlotSegment, Seed};
use super::plot::Plot;
use advent2024::id_generator;

#[derive(Debug,Default)]
struct LastGardenScanLine {
    segments: Vec<(PlotSegment, usize, bool)>,
}

impl Index<usize> for LastGardenScanLine {
    type Output = (PlotSegment, usize, bool);

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments[index]
    }
}

impl LastGardenScanLine {
    fn overlaps(&self, segment: &PlotSegment) -> Vec<(usize,usize)> {
        self.segments
            .iter()
            .take_while(|(lseg,_,_)| segment.end() > lseg.start())
            .enumerate()
            .filter_map(|(idx, (aseg, id, _))|
                if aseg.plant() == segment.plant() &&
                    aseg.is_overlapping(segment) { Some((idx,*id)) } else { None }
            )
            .collect::<Vec<_>>()
    }
    fn drain(&mut self) -> impl Iterator<Item = (PlotSegment, usize, bool)> {
        self.segments.drain(..)
    }
    fn push(&mut self, segment: PlotSegment, id: usize) {
        self.segments.push((segment, id, false));
    }
    fn flag_matched(&mut self, index: usize) {
        self.segments[index].2 = true;
    }
    fn drain_unmatched(&mut self) -> impl Iterator<Item = (PlotSegment, usize, bool)> {
        self.segments.drain(..).filter(|(_,_,mathced)| !mathced)
    }
    fn find_replace_plot_id(&mut self, from_id: usize, to_id: usize ) -> bool {
        self.segments
            .iter_mut()
            .filter(|(_,s_id,_)| from_id.eq(s_id) )
            .all(|(_,id,_)| {
                *id = to_id;
                true
            })
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
            idx += chunk.len() as Seed;
            PlotSegment::new(chunk[0] as char, start..idx)
        })
}


fn push_segments(plots: &mut HashMap<usize, Plot>, g_line: impl Iterator<Item = (PlotSegment, usize, bool)>, line: usize) {
    g_line
        .for_each(|(seg, id, _)| {
            plots.entry(id).or_default().insert(line-1, seg);
        });
}

// for each new segment identify the plot that is overlapping with and assign the segment the plot's ID
fn process_line(
    input: &str,
    plots: HashMap<usize, Plot>,
    g_line: LastGardenScanLine,
    mut get_plot_id: impl FnMut() -> usize,
    line: usize
) -> (HashMap<usize, Plot>, LastGardenScanLine)
{
    let mut new_g_line = LastGardenScanLine::default();

    // for each plant segment
    let (mut plots, mut g_line) = extract_ranges(input)
        .fold(
            (plots, g_line),
            |(plots, g_line), segment|
            {
                // process segment against last Garden Scan Line
                let (
                    mut plots,
                    mut g_line,
                    seg_id,
                    depr_ids
                ) = process_segment( &segment, plots, g_line, line, &mut get_plot_id );

                // segmened allocated to a plot ID and
                new_g_line.push(segment, seg_id);

                // deal with IDs that got depracated by the segment we just procesed
                if let Some(ids) = depr_ids {
                    ids.into_iter()
                        .all(|plot_id| {
                            // remove depracated ID from garden map and hold onto its segments
                            let plot = plots.remove(&plot_id).unwrap();
                            // merge removed segments with processed segment's plot ID
                            plots.entry(seg_id).or_default().extend(plot);
                            // Active LastGardenScanLine might contain segments with deprecated IDs
                            // hence such segments must have their ID replaced with seg_id
                            g_line.find_replace_plot_id(plot_id, seg_id);
                            // Also, we might have stored segments in the next LastGardenScanLine
                            // with their IDs being depracated by the segment we just processed, e.g.
                            // Cur LastGardenLine -> ZZZ5ZZZZ..ZZ2ZZZZZZZ
                            // New LastGardenLine -> Z5Z...ZZ2ZZ...ZZZ2ZZ
                            // above first segment got processed first and stored with id 5, however
                            // when the 2nd segment got processed depracated 5 for 2 hence
                            new_g_line.find_replace_plot_id(plot_id, seg_id)
                        });
                }
                (plots, g_line)
            }
        );

    // Any scanline segments that didn't match indicate the end of plot region
    // therefore we move such segments to the garden map using their respective plot ID and current line number
    push_segments(&mut plots, g_line.drain_unmatched(), line);

    (plots, new_g_line)
}

// every segment is tested against the known plots
// if no overlaps found then a new plot ID is returned marking the start of a new plot region
// otherwise the ID of the overlapping plot region is returned
// the function also marks known plots as matched
// If the segment overlaps with multiple plots, the plots are merged under a single ID (master_id)
// We return a list of depracated plot IDs for post-processing
fn process_segment(
    segment: &PlotSegment,
    plots: HashMap<usize, Plot>,
    g_line: LastGardenScanLine,
    line: usize,
    mut get_plot_id: impl FnMut() -> usize
) -> (HashMap<usize, Plot>, LastGardenScanLine, usize, Option<Vec<usize>>)
    {
        // find active plots matching this segment
        // matching = (a) overlapping with && (b) have same plant type
        let mut matched = g_line.overlaps(segment);
        // if empty, then this we form a new plot by creating a new ID for the segment
        if matched.is_empty() {
            return (plots, g_line, get_plot_id(), None);
        }
        // otherwise, use the smallest plot ID matched as our master ID
        // Critical insight: very first plot instance formation always has the smallest ID,
        // hence when two areas are merged the area with the smallest ID
        // is quaranteed to have formed first hence must absorb the other area
        matched.sort_by_key(|(_,id)| *id);
        let (_, master_id, _) = g_line[ matched[0].0 ];

        matched
            .iter()
            // for each matched plot segment
            .fold(
                (plots, g_line, master_id, None),
                |(mut garden, mut g_line, master_id, mut depr_ids), &(index, _id)|
                {
                    // flag it as matched; that is, plot region continues to next line
                    g_line.flag_matched(index);
                    // clone plot segment and plot_id; don't remove it until all remaining new segments are processed
                    let (seg, plot_id, _) = g_line[index].clone();
                    // move cloned plot segment onto the garden map under the current line number
                    garden.entry(plot_id).or_default().insert(line-1, seg);
                    // if plot_id is NOT equal to master_id, then consolidate plots
                    if plot_id != master_id {
                        // push plot ID to the depracated plot ID list
                        depr_ids.get_or_insert_default().push(plot_id);
                    }
                    (garden, g_line, master_id, depr_ids)
                }
            )
}


// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
pub fn parse_plots(input: &str) -> HashMap<usize,Plot> {
    // id generator fn()
    let mut get_new_plot_id = id_generator(0);
    // line counter
    let mut get_line_number = id_generator(0);

    let (mut plots, mut g_line) = input
        .lines()
        .fold((HashMap::<usize, Plot>::new(), LastGardenScanLine::default()), |(plots, g_line), input| {
            process_line(
                input,
                plots,
                g_line,
                &mut get_new_plot_id,
                get_line_number()
            )
        });

    // move plot segments remaining to the garden map under their respective plot ID
    push_segments(&mut plots, g_line.drain(), get_line_number());

    // return garden map
    plots
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::parser; // Import items from the parent module (parser)

    #[test]
    fn test_garden_parser() {
        let test_runs: [(&str, HashMap<usize,(usize,usize)>);3] = [
            (
                "src/bin/day12/sample.txt",
                HashMap::from(
                    [
                        (6, (1 ,4)),
                        (7, (13, 18)),
                        (5, (11, 20)),
                        (0, (12, 18)),
                        (2, (14, 28)),
                        (9, (5 ,12)),
                        (3, (10, 18)),
                        (10, (3 ,8)),
                        (4, (13, 20)),
                        (8, (14, 22)),
                        (1, (4 ,8))
                    ]
                )
            ), (
                "src/bin/day12/sample8.txt",
                HashMap::from([
                    (15, (5,12)),
                    (4, (59,68)),
                    (17, (1,4)),
                    (9, (1,4)),
                    (7, (5,12)),
                    (0, (4,10)),
                    (18, (1,4)),
                    (2, (15,20)),
                    (16, (2,6)),
                    (3, (12,16)),
                    (11, (18,18)),
                    (10, (1,4)),
                    (5, (4,10)),
                    (1, (2,6)),
                    (12, (15,24)),
                    (14, (10,14)),
                    (6, (5,12)),
                    (13, (1,4))
                ])
            ), (
                "src/bin/day12/sample4.txt",
                HashMap::from([
                    (3, (5,10)),
                    (0, (11,18)),
                    (12, (3,8)),
                    (1, (69,88)),
                    (5, (16,22)),
                    (7, (13,22)),
                    (8, (5,12)),
                    (11, (2,6)),
                    (9, (1,4)),
                    (2, (7,12)),
                    (13, (2,6)),
                    (10, (6,12))
                ])
            )
        ];

        for (file, results) in test_runs {
            let garden = parser::parse_plots(
                &std::fs::read_to_string(file).unwrap()
            );
            println!("\nTest Run ===========================");
            for (id, plot) in garden {
                println!("ID:{id}\n{plot:?}");
                let (a,b) = (plot.area(), plot.perimeter());
                println!("area: {} * perimeter: {} = {}", a, b, a * b);
                assert_eq!((a,b), results[&id]);
            }
        }
    }
}
