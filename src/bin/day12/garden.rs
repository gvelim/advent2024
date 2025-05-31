use std::collections::{HashMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Index;
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
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use itertools::Itertools;

        // Collect all segments from all plots into a BTreeSet.
        // This flattens the data structure and sorts segments primarily by y-coordinate
        // and secondarily by segment start, which is crucial for the debug output
        // to be rendered scanline by scanline and segments within a scanline
        // in order.
        let segments = self.plots
            .iter()
            .flat_map(|(id, plot)|
                // For each plot, associate its ID with each of its segments so we can colour it correctly.
                std::iter::repeat(id).zip(plot.iter())
            )
            // Reformat the tuple to prioritize y-coordinate for sorting by BTreeSet.
            .map(|(p_id, (y, p_seg))| (y,(p_seg,p_id)))
            // Collect into a BTreeSet to automatically sort the segments.
            .collect::<BTreeSet<_>>();

        // Define a closure to generate a deterministic color based on the plot ID.
        // This ensures that the same plot ID always gets the same color across runs,
        // making the debug output more consistent and easier to follow.
        let get_color = |p_id: &usize| -> (u8, u8, u8) {
            let mut hasher = DefaultHasher::new();
            // Hash the plot ID.
            p_id.hash(&mut hasher);
            let hash = hasher.finish();
            // Extract R, G, B components from the hash value.
            (
                ((hash >> 16) & 0xFF) as u8, // Red component from bits 16-23
                ((hash >> 8) & 0xFF) as u8,  // Green component from bits 8-15
                (hash & 0xFF) as u8,         // Blue component from bits 0-7
            )
        };

        // Iterate through the collected segments, grouping them by their y-coordinate (scanline).
        // `chunk_by` from `itertools` is used to create these groups efficiently.
        // The output includes ANSI escape codes for background colors to visualize plots.
        for (y, segs) in segments.into_iter().chunk_by(|&(y,_)| y).into_iter() {
            // Write the scanline number (y + 1 because y is 0-indexed).
            // Use {:3} for fixed-width alignment. Handle potential write errors.
            write!(f, "{:3} ", y + 1)?;

            // Iterate through segments belonging to the current scanline.
            for (_, (p_seg, p_id)) in segs {
                // Get the deterministic color for the plot ID.
                let (r, g, b) = get_color(p_id);
                // Get the plant character for the segment.
                let plant_char = p_seg.plant();

                // Write the ANSI escape code to set the background color using 24-bit color (48;2;R;G;B).
                write!(f, "\x1B[48;2;{};{};{}m", r, g, b)?;
                // Write the plant character repeatedly for the length of the segment.
                for _ in 0..p_seg.len() {
                    write!(f, "{}", plant_char)?;
                }
                // Write the ANSI escape code to reset text attributes (back to default).
                write!(f, "\x1B[0m")?;
            }
            // After processing all segments for a scanline, write a newline character.
            writeln!(f)?;
        }
        // Return Ok(()) to indicate successful formatting.
        Ok(())
    }
}
