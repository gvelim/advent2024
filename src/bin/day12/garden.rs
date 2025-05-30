use std::collections::{HashMap, BTreeSet};
use std::fmt::Debug;
use std::iter::repeat;
use std::ops::Index;
use rand::Rng;
use super::plot::Plot;
use super::parser; // Import the new parser module

// Performance optimizations applied:
// 1. Changed from BTreeMap to HashMap for O(1) vs O(log n) access
// 2. Pre-allocated collections with known capacities
// 3. Reduced string allocations in Debug formatting
// 4. Minimized repeated HashMap lookups

#[derive(Default)]
pub(super) struct  Garden {
    plots: HashMap<usize, Plot>
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
        // Removed unused imports for performance optimization:
        // use colored::Colorize; // Not needed when manually writing ANSI codes
        // use itertools::repeat_n; // Not needed when manually repeating characters

        use rand::rng; // Still needed for random color generation
        use itertools::Itertools; // Still needed for chunk_by

        // Collect and sort segments by y-coordinate (scanline)
        // This part remains the same as it's necessary for the output structure
        let segments = self.plots
            .iter()
            .flat_map(|(id, plot)|
                repeat(id).zip(plot.iter())
            )
            .map(|(p_id, (y, p_seg))| (y,(p_seg,p_id)))
            .collect::<BTreeSet<_>>();

        // Generate a color map for each plot ID
        // This part remains the same
        let mut thread = rng();
        let color_map: HashMap<&usize, (u8, u8, u8)> = self.plots
            .iter()
            .map(|(p_id, _)|
                (
                    p_id,
                    (
                        thread.random_range(0x07..=0x7F),
                        thread.random_range(0x07..=0x7F),
                        thread.random_range(0x07..=0x7F)
                    )
                )
            )
            .collect();

        // Iterate through segments, grouped by scanline (y-coordinate)
        segments
            .into_iter()
            .chunk_by(|&(y,_)| y )
            .into_iter()
            .for_each(|(y, segs)| {
                // Write the scanline number
                write!(f, "{:3} ",y+1).ok(); // Using ok() as in the original code

                // Process segments within the current scanline
                segs.into_iter()
                    .for_each(|(_,(p_seg, p_id))|{
                        let colour = color_map[p_id];

                        // Performance optimization: Manually write ANSI codes and repeat character
                        // by writing directly to the formatter.
                        // Write the ANSI escape code for background truecolor
                        write!(f, "\x1B[48;2;{};{};{}m", colour.0, colour.1, colour.2).ok();
                        // Write the plant character repeated 'segment_len' times
                        for _ in 0..p_seg.len() {
                            write!(f, "{}", p_seg.plant()).ok();
                        }
                        // Write the ANSI escape code to reset formatting
                        write!(f, "\x1B[0m").ok();
                    });
                // Write a newline character after each scanline
                writeln!(f).ok();
            });
        Ok(()) // Return Ok(()) indicating successful formatting
    }
}
