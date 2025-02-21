# Day 12 Challenge - README

## Overview

The `parse_garden` function processes a garden map (puzzle input) to group plant segments into plot regions using a scan-line algorithm. The algorithm tracks vertical adjacency between plant segments, id merges regions across lines.

## Problem Statement

### Step 1: Scan-Line Processing with Active Segments

The parser iterates over each line of the input, extracting horizontal plant segments of garden plots. For each segment, it checks for overlaps with active plot segments from the previous line (stored in `cur_aseg`).

```rust
let mut cur_aseg: Vec<(PlotSegment, usize, bool)> = Vec::new();
let mut next_aseg: Vec<(PlotSegment, usize, bool)> = Vec::new();
// ...
for segment in segments {
    // Check overlaps with active segments
    let mut matched = cur_aseg.iter_mut().enumerate().filter_map(|(i, (aseg, _, m))| {
        if aseg.plant() == segment.plant() && aseg.is_overlapping(&segment) {
            *m = true;
            Some(i)
        } else { None }
    }).collect::<Vec<_>>();
    // ...
}
```

### Step 2: Plot Region Merging and ID Assignment

When overlapping segments are found, they are merged under a "master" plot region ID. New segments without overlaps start new plot regions.

```rust
if matched.is_empty() {
    next_aseg.push((segment, get_plot_id(), false));
    continue;
}
let (_, master_id, _) = cur_aseg[matched[0]];
// Merge segments into master region
while let Some(index) = matched.pop() {
    let (seg, plot_id, _) = cur_aseg[index].clone();
    garden.entry(plot_id).or_default().insert((line, seg));
    if plot_id != master_id {
        let plot = garden.remove(&plot_id).unwrap();
        garden.entry(master_id).or_default().extend(plot);
    }
}
```

### Step 3: Finalizing Regions

Unmatched active plot segments are moved to the garden after processing each line.

```rust
// After processing line segments
while let Some((seg, id, matched)) = cur_aseg.pop() {
    if !matched {
        garden.entry(id).or_default().insert((line, seg));
    }
}
```

### Step 4: Tidy up post line processing

Add active plot segments remaining after all lines have been processed.

```rust
// At end of input
while let Some((seg, id, _)) = cur_aseg.pop() {
    garden.entry(id).or_default().insert((line+1, seg));
}
```

This approach efficiently groups adjacent garden plots into regions while handling vertical and horizontal adjacencies.
