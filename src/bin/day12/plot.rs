use std::{collections::BTreeSet, ops::RangeInclusive};
use super::segment::{PlotSegment, Seed};

pub(super) type Plot = BTreeSet<(usize, PlotSegment)>;

pub(super) fn area(rows: &Plot) -> usize {
    rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
}

pub(super) fn perimeter(rows: &Plot) -> usize {
    let y_range = get_plot_y_range(rows);

    north_perimeter_counter(rows, y_range.clone()) +
        // Scan South Perimeter from bottom->up == scanning top->bottom using the reverse line numbers
        north_perimeter_counter(rows, y_range.clone().rev()) +
        rows.iter().count() * 2
}

fn get_plot_y_range(rows: &Plot) -> RangeInclusive<usize> {
    let y_start  = rows.first().unwrap().0;
    let y_end  = rows.last().unwrap().0;
    y_start..=y_end
}

pub fn get_plot_bounding_segs(rows: &Plot) -> (PlotSegment, PlotSegment) {
    let (_, seg) = rows.first().unwrap();
    let west_bound = PlotSegment::new(seg.plant(), 0..1);
    let east_bound = PlotSegment::new(seg.plant(), Seed::MAX-1..Seed::MAX);
    (west_bound, east_bound)
}

fn north_perimeter_counter(rows: &Plot, range: impl Iterator<Item = usize>) -> usize  {
    let (west_bound, east_bound) = get_plot_bounding_segs(rows);

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

pub(super) fn _display_plot(plot: &Plot) {
    use std::rc::Rc;

    let last = plot.last().unwrap().0;
    let first = plot.first().unwrap().0;
    let (left_vals, right_vals): (Vec<_>,Vec<_>) = plot.iter()
        .map(|(_, seg)| (seg.start(), seg.end() ))
        .unzip();
    let left = *left_vals.iter().min().unwrap();
    let right = *right_vals.iter().max().unwrap();

    (first..=last).for_each(|y| {
        let (west_bound, east_bound) = get_plot_bounding_segs(plot);
        let line_segments = plot.range((y, west_bound) ..= (y, east_bound)).collect::<Rc<[_]>>();

        (left..right).for_each(|x| {
            let segment = line_segments.iter().find(|(_, seg)| seg.contains(x));
            match segment {
                Some((_, seg)) => print!("{}", seg.plant()),
                None => print!("."),
            }
        });
        println!(" = {:?}", line_segments);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse_garden;

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
