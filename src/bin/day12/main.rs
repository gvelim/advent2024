mod segment;

use std::collections::{BTreeSet, HashMap};
use advent2024::id_generator;
use segment::{extract_ranges, PlotSegment};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden = parse_garden(&input);

    garden
        .iter()
        .for_each(|(id,v)| println!("{id}::{:?} = {}", v, area(v)));
}

type Plot = BTreeSet<(usize, PlotSegment)>;
type Garden = HashMap<usize, Plot>;

// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
fn parse_garden(input: &str) -> Garden {
    // set active map structures 1 & 2; holding (K,V) as (active segment, ID)
    let mut actseg1: Vec<(PlotSegment, usize, bool)> = Vec::new();
    let mut actseg2: Vec<(PlotSegment, usize, bool)> = Vec::new();

    // id generator fn()
    let mut get_plot_id = id_generator(0);

    // line counter
    let mut line = 0;

    let mut garden = input.lines()
        .map(extract_ranges)
        .enumerate()
        // for each line of plant segments(plant type, range)
        .fold(Garden::new(), |mut garden, (l, segments)| {
            line = l;

            // for each plant segment
            for segment in segments {

                // find within map 1, all active segments indeces that (a) overlap with && (b) have same plant type and flag those as matched
                let mut matched = actseg1
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, (aseg, _, m))| {
                        if aseg.plant() == segment.plant() && aseg.is_overlapping(&segment) {
                            *m = true;
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // if empty, then push a new (K,V) (segment, plot ID) into active segments map 2 and process next segment
                if matched.is_empty() {
                    actseg2.push((segment, get_plot_id() , false));
                    continue
                }

                // set master ID for matching plot IDs to merge into
                let master_id = actseg1[ matched[0] ].1;

                // push new segment to active segments map 2 using first entry as the master ID
                actseg2.push((segment, master_id, false));

                // get index of each matching plot
                while let Some(index) = matched.pop() {
                    // clone plot and plot_id; don't remove it as queued up segments may also match it
                    let (seg, plot_id, _) = actseg1[index].clone();

                    // push active segment into garden map under its original plot ID and using current line number
                    garden.entry(plot_id).or_default().insert((line, seg));

                    // remove plot ID from garden map and hold onto its segments
                    let plot = garden.remove(&plot_id).unwrap();

                    // merge removed segments into the plot with master ID
                    garden.entry(master_id)
                        .or_default()
                        .extend( plot);
                }
            }

            // Empty map 1 and move any unmatched active segments to the garden map with matching plot ID and current line number
            while let Some((seg, id, matched)) = actseg1.pop() {
                if !matched {
                    garden.entry(id).or_default().insert((line, seg));
                }
            }

            // swap active map 1 with active map 2, so map 2 is the new active map
            std::mem::swap(&mut actseg1, &mut actseg2);
            garden
        });

    // Move any leftover active segments to the garden map
    while let Some((seg, id, _)) = actseg1.pop() {
        garden.entry(id).or_default().insert((line+1, seg));
    }

    // return garden map
    garden
}

fn area(rows: &Plot) -> usize {
    rows.iter().map(|seg| seg.1.len()).sum::<usize>()
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
        // A region of I plants with price 4 * 8 = 32.
        assert_eq!(area(&garden[&2]), 4, "expected 4 got {:?}",garden[&2]);
        // A region of C plants with price 14 * 28 = 392.
        assert_eq!(area(&garden[&3]), 14, "expected 14 got {:?}",garden[&3]);
        // A region of F plants with price 10 * 18 = 180.
        assert_eq!(area(&garden[&4]), 10, "expected 10 got {:?}",garden[&4]);
        // A region of V plants with price 13 * 20 = 260.
        assert_eq!(area(&garden[&5]), 13, "expected 13 got {:?}",garden[&5]);
        // A region of J plants with price 11 * 20 = 220.
        assert_eq!(area(&garden[&6]), 11, "expected 11 got {:?}",garden[&6]);
        // A region of C plants with price 1 * 4 = 4.
        assert_eq!(area(&garden[&7]), 1, "expected 1 got {:?}",garden[&7]);
        // A region of E plants with price 13 * 18 = 234.
        assert_eq!(area(&garden[&8]), 13, "expected 13 got {:?}",garden[&8]);
        // A region of I plants with price 14 * 22 = 308.
        assert_eq!(area(&garden[&9]), 14, "expected 14 got {:?}",garden[&9]);
        // A region of M plants with price 5 * 12 = 60.
        assert_eq!(area(&garden[&10]), 5, "expected 5 got {:?}",garden[&10]);
        // A region of S plants with price 3 * 8 = 24.
        assert_eq!(area(&garden[&11]), 3, "expected 3 got {:?}",garden[&11]);

    }

}
