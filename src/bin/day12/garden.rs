use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{repeat, repeat_n};
use std::{collections::BTreeMap, ops::Index};
use advent2024::id_generator;
use rand::Rng;
use super::segment::{extract_ranges, PlotSegment};
use super::plot::Plot;

#[derive(Default)]
pub(super) struct  Garden {
    plots: BTreeMap<usize, Plot>
}

impl Garden {
    pub(super) fn iter(&self) -> impl Iterator<Item = (&usize, &Plot)> {
        self.plots.iter()
    }

    // garden is a collection of plots expressed by a 1 or more overlapping vertical segments
    // parser extracts and composes plots per scanline
    // a plot is composed out of multiple scanlines
    pub(super) fn parse_garden(input: &str) -> Garden {
        // id generator fn()
        let mut get_new_plot_id = id_generator(0);
        // line counter
        let mut get_line_number = id_generator(0);

        let (mut plots, mut g_line) = input
            .lines()
            .fold((BTreeMap::<usize, Plot>::new(), LastGardenScanLine::default()), |(plots, g_line), input| {
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
        Garden { plots }
    }
}

fn push_segments(plots: &mut BTreeMap<usize, Plot>, g_line: impl Iterator<Item = (PlotSegment, usize, bool)>, line: usize) {
    g_line
        .for_each(|(seg, id, _)| {
            plots.entry(id).or_default().insert(line-1, seg);
        });
}

// for each new segment identify the plot that is overlapping with and assign the segment the plot's ID
fn process_line(
    input: &str,
    plots: BTreeMap<usize, Plot>,
    g_line: LastGardenScanLine,
    mut get_plot_id: impl FnMut() -> usize,
    line: usize
) -> (BTreeMap<usize, Plot>, LastGardenScanLine)
{
    let mut new_g_line = LastGardenScanLine::default();
    // println!("In: {input}\n{:?}",g_line);
    // for each plant segment
    let (mut plots, mut g_line) = extract_ranges(input)
        .fold((plots, g_line), |(plots, g_line), segment| {
            let (plots, g_line, seg_id, depr_ids) = process_segment(
                &segment,
                plots,
                g_line,
                line,
                &mut get_plot_id
            );
            new_g_line.push(segment, seg_id);
            // replace any ids from already processed segments
            // that have just got depracated as result of segment processed
            // Cur LastGardenLine -> ZZZ5ZZZZ..ZZ2ZZZZZZZ
            // New LastGardenLine -> Z5Z...ZZ2ZZ...ZZZZZZ
            // above first segment processed with id 5, however
            // the processing of the 2nd segment causes the id 5 to be depracated
            if let Some(ids) = depr_ids {
                ids.into_iter()
                    .all(|id|
                        new_g_line.find_replace_plot_id(id, seg_id)
                    );
            }
            (plots, g_line)
        });

    // println!("WIP: {:?}",g_line);
    // Any scanline segments that didn't match indicate the end of plot region
    // therefore we move such segments to the garden map using their respective plot ID and current line number
    push_segments(&mut plots, g_line.drain_unmatched(), line);
    // println!("New: {:?}\n",new_g_line);
    (plots, new_g_line)
}

// every segment is tested against the known plots
// if no overlaps found then a new plot ID is returned marking the start of a new plot region
// otherwise the ID of the overlapping plot region is returned
// the function also marks known plots as matched
// If the segment overlaps with multiple plots, the plots are merged under a single ID (master_id)
// We return a list of depracated plot IDs as the caller will need to know
// if any previous plot IDs passed under the new LastGardenLine have been depracated
fn process_segment(
    segment: &PlotSegment,
    plots: BTreeMap<usize, Plot>,
    g_line: LastGardenScanLine,
    line: usize,
    mut get_plot_id: impl FnMut() -> usize
) -> (BTreeMap<usize, Plot>, LastGardenScanLine, usize, Option<Vec<usize>>)
    {
    // find active plots matching this segment
    // matching = (a) overlapping with && (b) have same plant type
    let mut matched = g_line.overlaps(segment);

    // if empty, then return a new plot ID for the segment
    if matched.is_empty() {
        return (plots, g_line, get_plot_id(), None);
    }

    // otherwise, use the first matching plot ID as the master ID for consolidating all matched plots
    matched.sort_by_key(|(_,id)| *id);
    let (_, master_id, _) = g_line[ matched[0].0 ];

    matched.iter()
        // for each matched plot segment
        .fold((plots, g_line, master_id, None), |(mut garden, mut g_line, master_id, mut depr_ids), &(index, _id)| {
            // flag it as matched; that is, plot region continues to next line
            g_line.flag_matched(index);

            // clone plot segment and plot_id; don't remove it until all remaining new segments are processed
            let (seg, plot_id, _) = g_line[index].clone();

            // move cloned plot segment onto the garden map under the current line number
            garden.entry(plot_id).or_default().insert(line-1, seg);

            // if plot_id is NOT equal to master_id, then consolidate plots
            if plot_id != master_id {
                // Keep LastGardenScanLine consisent with Garden
                // by replacing segments with plot_id to master_id
                g_line.find_replace_plot_id(plot_id, master_id);

                // remove plot ID from garden map and hold onto its segments
                let plot = garden.remove(&plot_id).unwrap();

                // merge removed segments into the plot with master ID
                garden.entry(master_id)
                .or_default()
                .extend(plot);

                // add to the depracated plot ID list
                depr_ids.get_or_insert_default().push(plot_id);
            }
            (garden, g_line, master_id, depr_ids)
        })
}

impl Index<&usize> for Garden {
    type Output = Plot;

    fn index(&self, index:&usize) -> &Self::Output {
        &self.plots[index]
    }
}

impl Debug for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        use rand::rng;
        use std::collections::BTreeSet;
        use itertools::Itertools;

        let segments = self.plots
            .iter()
            .flat_map(|(id, plot)|
                repeat(id).zip(plot.iter())
            )
            .map(|(p_id, (y, p_seg))| (y,(p_seg,p_id)))
            .collect::<BTreeSet<_>>();

        let mut thread = rng();
        let color_map = segments
            .clone()
            .iter()
            .map(|&(_,(seg,&p_id))|
                (
                    p_id,
                    (
                        thread.random_range(0x00..= 0xFF) ^ seg.plant() as u8,
                        thread.random_range(0x00..= 0xFF) ^ seg.plant() as u8,
                        thread.random_range(0x00..= 0xFF) ^ seg.plant() as u8
                    )
                )
            )
            .collect::<HashMap<_,_>>();

        segments
            .into_iter()
            .chunk_by(|&(y,_)| y )
            .into_iter()
            .for_each(|(y, segs)| {
                write!(f, "{:3} ",y+1).ok();
                segs.into_iter()
                    .for_each(|(_,(p_seg, p_id))|{
                        let colour = color_map[p_id];
                        let plant = repeat_n(p_seg.plant(), p_seg.len() as usize)
                            .collect::<String>()
                            .on_truecolor(colour.0, colour.1, colour.2);
                        write!(f, "{plant}").ok();
                    });
                writeln!(f).ok();
            });
        Ok(())
    }
}

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
            .enumerate()
            .filter_map(|(i, (aseg, id, _))|
                if aseg.plant() == segment.plant() &&
                    aseg.is_overlapping(segment) { Some((i,*id)) } else { None }
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
