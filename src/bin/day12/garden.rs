use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{repeat, repeat_n};
use std::{collections::BTreeMap, ops::Index};
use rand::Rng;
use super::plot::Plot;
use super::parser; // Import the new parser module

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
    pub(super) fn parse(input: &str) -> Garden {
        Garden { plots: parser::parse_plots(input) }
    }
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
            // .clone()
            .iter()
            .map(|&(_,(seg,&p_id))|
                (
                    p_id,
                    (
                        thread.random_range(0x07..=0x7F) ^ seg.plant() as u8,
                        thread.random_range(0x07..=0x7F) ^ seg.plant() as u8,
                        thread.random_range(0x07..=0x7F) ^ seg.plant() as u8
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
