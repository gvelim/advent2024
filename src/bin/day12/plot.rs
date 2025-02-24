use std::{collections::{BTreeSet, HashMap}, ops::RangeInclusive};
use advent2024::id_generator;
use super::segment::{extract_ranges, PlotSegment, Seed};

pub(super) type Plot = BTreeSet<(usize, PlotSegment)>;
pub(super) type Garden = HashMap<usize, Plot>;

// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
pub(super) fn parse_garden(input: &str) -> Garden {
    // id generator fn()
    let mut get_plot_id = id_generator(0);

    // line counter
    let mut line = 0;

    let (mut garden, cur_aseg) = input
        .lines()
        // extract segments
        .map(extract_ranges)
        // capture the line number
        .enumerate()
        // for each line worth of plant segments(plant type, range)
        .fold(
            (Garden::new(), Vec::<(PlotSegment, usize, bool)>::new()),
            |(mut garden, mut curr_aseg), (l, segments)| {

            // set active map structures next; holding (K,V) as (active segment, ID, matched)
            let mut next_aseg: Vec<(PlotSegment, usize, bool)> = Vec::new();
            line = l;

            // for each plant segment
            for segment in segments.into_iter() {
                // find within current map 1, all active segments indeces that
                // (a) overlap with && (b) have same plant type and flag those as matched
                let matched = curr_aseg
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, (aseg, _, m))|
                        if aseg.plant() == segment.plant() &&
                            aseg.is_overlapping(&segment) { *m = true; Some(i) } else { None }
                    )
                    .collect::<Vec<_>>();

                // if empty, then push a new (K,V) (segment, plot ID) into next active segments map 2 and process next segment
                if matched.is_empty() {
                    next_aseg.push((segment, get_plot_id() , false));
                    continue
                }

                // set the master ID for consolidating all matching plot IDs
                let (_, master_id, _) = curr_aseg[ matched[0] ];
                // push new segment to next active segments map 2 under the master ID
                next_aseg.push((segment, master_id, false));
                // get index of each matching plot
                matched.into_iter().for_each(|index| {
                    // clone plot and plot_id; don't remove it as queued up segments may also match it
                    let (seg, plot_id, _) = curr_aseg[index].clone();
                    // push active segment into garden map under its original plot ID and using current line number
                    garden.entry(plot_id).or_default().insert((line, seg));
                    // if plot_id is NOT equal to master_id, then consolidate plots
                    if plot_id != master_id {
                        // remove plot ID from garden map and hold onto its segments
                        let plot = garden.remove(&plot_id).unwrap();
                        // merge removed segments into the plot with master ID
                        garden.entry(master_id)
                            .or_default()
                            .extend(plot);
                    }
                });
            }

            // Empty vector with unmatched segments by moving to the garden map under their plot ID and current line number
            curr_aseg
                .into_iter()
                .filter(|(_,_,mathced)| !mathced)
                .for_each(|(seg, id, _)| {
                    garden.entry(id).or_default().insert((line, seg));
                });

            (garden, next_aseg)
        });

    // move remaining segments to the garden map under their respective plot ID
    cur_aseg
        .into_iter()
        .for_each(|(seg, id, _)| {
            garden.entry(id).or_default().insert((line+1, seg));
        });

    // return garden map
    garden
}

pub(super) fn area(rows: &Plot) -> usize {
    rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
}

pub(super) fn perimeter(rows: &Plot) -> usize {
    let y_range = get_plot_y_range(rows);

    let count_east_west_perimeter = west_east_perimeter_counter(rows);
    let count_north_perimeter = north_perimeter_counter(rows);

    count_north_perimeter(Box::new(y_range.clone())) +
        // Scan South Perimeter from bottom->up == scanning top->bottom using the reverse line numbers
        count_north_perimeter(Box::new(y_range.clone().rev())) +
        count_east_west_perimeter(Box::new(y_range.clone()))
}

fn get_plot_y_range(rows: &Plot) -> RangeInclusive<usize> {
    let y_start  = rows.first().unwrap().0;
    let y_end  = rows.last().unwrap().0;
    y_start..=y_end
}

fn get_plot_bounding_segs(rows: &Plot) -> (PlotSegment, PlotSegment) {
    let (_, seg) = rows.first().unwrap();
    let west_bound = PlotSegment::new(seg.plant(), 0..1);
    let east_bound = PlotSegment::new(seg.plant(), Seed::MAX-1..Seed::MAX);
    (west_bound, east_bound)
}

fn north_perimeter_counter(rows: &Plot) -> impl Fn(Box<dyn Iterator<Item = usize>>) -> usize  {
    let (west_bound, east_bound) = get_plot_bounding_segs(rows);

    move | range: Box<dyn Iterator<Item = usize>>| -> usize {
        range.map(|y| {
            rows
                // for each segment in line `y`
                .range( (y, west_bound.clone()) ..= (y, east_bound.clone()) )
                .map(|(_, seg)| {
                    // calculate perimeter on segment's north side
                    // Sum( segment overlapping area against segment(s) above)
                    seg.len() as usize - rows
                        .range( (y-1, west_bound.clone()) ..= (y-1, east_bound.clone()) )
                        .filter(|(_,nseg)| nseg.is_overlapping(seg) )
                        .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
    }
}

fn west_east_perimeter_counter(rows: &Plot) -> impl Fn(Box<dyn Iterator<Item = usize>>) -> usize {
    let (west_bound, east_bound) = get_plot_bounding_segs(rows);

    move |range: Box<dyn Iterator<Item = usize>>| -> usize {
        range
            .map(|y|
                rows.range( (y,west_bound.clone()) ..= (y,east_bound.clone()) )
                    .count() * 2
            )
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garden_parser() {
        let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();
        let garden = parse_garden(&input);

        // A region of R plants with price 12 * 18 = 216.
        assert_eq!(area(&garden[&1]), 12, "expected 12 got {:?}",garden[&1]);
        assert_eq!(perimeter(&garden[&1]), 18, "expected 18 got {:?}",garden[&1]);
        // A region of I plants with price 4 * 8 = 32.
        assert_eq!(area(&garden[&2]), 4, "expected 4 got {:?}",garden[&2]);
        assert_eq!(perimeter(&garden[&2]), 8, "expected 8 got {:?}",garden[&2]);
        // A region of C plants with price 14 * 28 = 392.
        assert_eq!(area(&garden[&3]), 14, "expected 14 got {:?}",garden[&3]);
        assert_eq!(perimeter(&garden[&3]), 28, "expected 28 got {:?}",garden[&3]);
        // A region of F plants with price 10 * 18 = 180.
        assert_eq!(area(&garden[&4]), 10, "expected 10 got {:?}",garden[&4]);
        assert_eq!(perimeter(&garden[&4]), 18, "expected 18 got {:?}",garden[&4]);
        // A region of V plants with price 13 * 20 = 260.
        assert_eq!(area(&garden[&5]), 13, "expected 13 got {:?}",garden[&5]);
        assert_eq!(perimeter(&garden[&5]), 20, "expected 20 got {:?}",garden[&5]);
        // A region of J plants with price 11 * 20 = 220.
        assert_eq!(area(&garden[&6]), 11, "expected 11 got {:?}",garden[&6]);
        assert_eq!(perimeter(&garden[&6]), 20, "expected 20 got {:?}",garden[&6]);
        // A region of C plants with price 1 * 4 = 4.
        assert_eq!(area(&garden[&7]), 1, "expected 1 got {:?}",garden[&7]);
        assert_eq!(perimeter(&garden[&7]), 4, "expected 4 got {:?}",garden[&7]);
        // A region of E plants with price 13 * 18 = 234.
        assert_eq!(area(&garden[&8]), 13, "expected 13 got {:?}",garden[&8]);
        assert_eq!(perimeter(&garden[&8]), 18, "expected 18 got {:?}",garden[&8]);
        // A region of I plants with price 14 * 22 = 308.
        assert_eq!(area(&garden[&9]), 14, "expected 14 got {:?}",garden[&9]);
        assert_eq!(perimeter(&garden[&9]), 22, "expected 22 got {:?}",garden[&9]);
        // A region of M plants with price 5 * 12 = 60.
        assert_eq!(area(&garden[&10]), 5, "expected 5 got {:?}",garden[&10]);
        assert_eq!(perimeter(&garden[&10]), 12, "expected 12 got {:?}",garden[&10]);
        // A region of S plants with price 3 * 8 = 24.
        assert_eq!(area(&garden[&11]), 3, "expected 3 got {:?}",garden[&11]);
        assert_eq!(perimeter(&garden[&11]), 8, "expected 8 got {:?}",garden[&11]);
    }

}
