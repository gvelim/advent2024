mod segment;

use std::collections::{BTreeSet, HashMap};
use segment::{extract_ranges, PlotSegment};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden = parse_garden(&input);

    garden
        .iter()
        .for_each(|(id,v)| println!("{id}::{:?} = {}", v, area(v)));
}

// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
fn parse_garden(input: &str) -> HashMap<usize, BTreeSet<(usize,PlotSegment)>> {
    // parsing logic is as follows:
    // set active map structures 1 & 2; holding (K,V) as (active segment, ID)
    let mut actseg1: Vec<(PlotSegment, usize, bool)> = Vec::new();
    let mut actseg2: Vec<(PlotSegment, usize, bool)> = Vec::new();

    // id generator fn()
    let mut get_id = (|mut start: usize| move || { start += 1; start })(0);
    // line counter
    let mut line = 0;

    let mut garden = input.lines()
        .map(extract_ranges)
        .enumerate()
        // for each line of plant segments(plant type, range)
        .fold(HashMap::new(), |mut garden, (l, mut segments)| {
            line = l;

            // for each plant segment
            while let Some(segment) = segments.next() {

                // find within map 1, all active segments indeces that (a) overlap with && (b) have same plant type and flag those as matched
                let mut matched = actseg1
                    .iter_mut()
                    .enumerate()
                    .filter_map(|(i, (seg, _, m))| {
                        if seg.plant() == segment.plant() && seg.is_overlapping(&segment) {
                            *m = true;
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // if empty, then push a new (K,V) (segment, ID) into active segments map 2 and process next segment
                if matched.is_empty() {
                    actseg2.push((segment, get_id() , false));
                } else {
                    // push new segment to active segments map 2 using same ID
                    let id = actseg1[ matched[0] ].1;
                    actseg2.push((segment, id, false));
                    // pop active segment(s) and push into garden map using same ID and current line number
                    while let Some(index) = matched.pop() {
                        garden.entry(id)
                            .or_insert(BTreeSet::new())
                            .insert((line, actseg1[index].0.clone()));
                    }
                }
            }

            // Empty map 1 and move any unmatched active segments to the garden map using same ID and current line number
            while let Some((seg, id, m)) = actseg1.pop() {
                if !m {
                    garden.entry(id).or_insert(BTreeSet::new()).insert((line, seg));
                }
            }

            // swap active map 1 with active map 2, so map 2 is the new active map
            std::mem::swap(&mut actseg1, &mut actseg2);
            garden
        });

    // Move any leftover active segments to the garden map
    while let Some((seg, id, _)) = actseg1.pop() {
        garden.entry(id).or_insert(BTreeSet::new()).insert((line+1, seg));
    }

    // return garden map
    garden
}

fn area(rows: &BTreeSet<(usize,PlotSegment)>) -> usize {
    rows.iter().map(|seg| seg.1.len()).sum::<usize>()
}

#[cfg(test)]
mod tests {

}
