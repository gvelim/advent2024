mod segment;
mod plot;

use std::collections::BTreeSet;

use itertools::Itertools;
use plot::{Garden, area, parse_garden, perimeter};

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/sample.txt").unwrap();

    let garden = parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(_, plot)|
            _display_plot(plot)
        )
        .map(|(_,v)|
            (area(v), perimeter(v))
        )
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    println!("Garden total cost : {total}");
}

fn _display_garden(garden: &Garden) {
    let segments = garden
        .values()
        .flat_map(|plot|  plot.iter())
        .collect::<BTreeSet<_>>();

    segments
        .into_iter()
        .chunk_by(|(y,_)| y)
        .into_iter()
        .for_each(|(_,segs)| {
            segs.into_iter()
                .for_each(|(_,seg)|{
                    (seg.start() .. seg.end()).for_each(|_| {
                        print!("{}", seg.plant());
                    });
                });
            println!();
        });
}

fn _display_plot(plot: &plot::Plot) {
    use plot::get_plot_bounding_segs;
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
