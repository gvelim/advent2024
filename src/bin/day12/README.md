# Garden Parsing Algorithm

The `parse_garden` function is responsible for parsing a garden map into a collection of regions, where each region is a contiguous group of garden plots growing the same type of plant. The function uses a combination of vertical scanning and region merging to efficiently identify and group these regions. Below is a step-by-step explanation of the algorithm, along with the relevant code snippets.

## Step-by-Step Explanation

### 1. **Initialization**

The function starts by initializing several data structures:

- **`cur_aseg` and `next_aseg`**: These are vectors that store active segments for the current and next scanlines, respectively. Each active segment is represented as a tuple `(PlotSegment, usize, bool)`, where:
  - `PlotSegment` represents a segment of a garden plot.
  - `usize` is the ID of the region to which the segment belongs.
  - `bool` indicates whether the segment has been matched with another segment in the current scanline.

- **`get_plot_id`**: This is a closure that generates unique IDs for new regions.

- **`line`**: This variable keeps track of the current line number being processed.

```rust
let mut cur_aseg: Vec<(PlotSegment, usize, bool)> = Vec::new();
let mut next_aseg: Vec<(PlotSegment, usize, bool)> = Vec::new();
let mut get_plot_id = id_generator(0);
let mut line = 0;
```

### 2. **Processing Each Line**

The garden map is processed line by line. For each line, the function extracts the segments of garden plots using the `extract_ranges` function. These segments are then processed to identify and merge regions.

```rust
let mut garden = input
    .lines()
    .map(extract_ranges)
    .enumerate()
    .fold(Garden::new(), |mut garden, (l, segments)| {
        line = l;
        // Process each segment in the current line
        for segment in segments {
            // Step 3: Matching and Merging Segments
        }
        // Step 4: Handling Unmatched Segments
        // Step 5: Swapping Active Segment Maps
        garden
    });
```

### 3. **Matching and Merging Segments**

For each segment in the current line, the function checks if it overlaps with any active segments from the previous line (`cur_aseg`). If an overlap is found, the segment is merged into the corresponding region.

- **Matching Segments**: The function iterates over `cur_aseg` to find segments that overlap with the current segment and have the same plant type. These segments are marked as matched.

```rust
let mut matched = cur_aseg
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
```

- **Creating New Regions**: If no matching segments are found, a new region is created with a unique ID.

```rust
if matched.is_empty() {
    next_aseg.push((segment, get_plot_id(), false));
    continue;
}
```

- **Merging Regions**: If matching segments are found, the current segment is added to the region with the smallest ID (referred to as the `master_id`). Any other matching regions are merged into this master region.

```rust
let (_, master_id, _) = cur_aseg[matched[0]];
next_aseg.push((segment, master_id, false));

while let Some(index) = matched.pop() {
    let (seg, plot_id, _) = cur_aseg[index].clone();
    garden.entry(plot_id).or_default().insert((line, seg));
    if plot_id != master_id {
        let plot = garden.remove(&plot_id).unwrap();
        garden.entry(master_id).or_default().extend(plot);
    }
}
```

### 4. **Handling Unmatched Segments**

After processing all segments in the current line, any active segments that were not matched are moved to the garden map under their respective region IDs.

```rust
while let Some((seg, id, matched)) = cur_aseg.pop() {
    if !matched {
        garden.entry(id).or_default().insert((line, seg));
    }
}
```

### 5. **Swapping Active Segment Maps**

The `cur_aseg` and `next_aseg` maps are swapped, so that `next_aseg` becomes the new `cur_aseg` for the next line.

```rust
std::mem::swap(&mut cur_aseg, &mut next_aseg);
```

### 6. **Finalizing the Garden Map**

After processing all lines, any remaining active segments are moved to the garden map.

```rust
while let Some((seg, id, _)) = cur_aseg.pop() {
    garden.entry(id).or_default().insert((line + 1, seg));
}
```

### 7. **Returning the Garden Map**

Finally, the function returns the garden map, which contains all the regions identified in the garden.

## Summary

The `parse_garden` function processes the garden map line by line, identifying and merging regions of contiguous garden plots. It uses a combination of vertical scanning and region merging to efficiently group segments into regions. The function maintains two active segment maps (`cur_aseg` and `next_aseg`) to track segments across lines and merges regions as needed. The final garden map contains all the regions, each represented by a unique ID and a set of segments.

This algorithm is efficient and ensures that all regions are correctly identified and merged, even in complex garden maps with nested or overlapping regions.

---
# Perimeter Calculation Algorithm
The `perimeter()` function calculates the perimeter of a plant region (plot).  The garden is a collection of plots, each composed of vertical plant segments. The challenge is to efficiently compute the perimeter of these irregularly shaped plots.  The algorithm cleverly avoids explicitly tracing the boundary, instead using line-by-line processing and overlap detection.

**Step-by-Step Breakdown of `perimeter()`:**

1. **Initialization:**
   The function starts by extracting key information from the input `Plot`:

   ```rust
   let (y_start, seg) = rows.first().unwrap().clone(); // Get starting y and a sample segment
   let y_end = rows.last().unwrap().0;             // Get ending y-coordinate
   let rng_start = PlotSegment::new(seg.plant(), 0..1); // Dummy segment for leftmost bound
   let rng_end = PlotSegment::new(seg.plant(), Seed::MAX-1..Seed::MAX); // Dummy segment for rightmost bound
   ```
   This sets up the y-coordinate range and creates dummy segments to simplify iteration across each line.

2. **North and South Perimeter Calculation (`north_perimeter_len`):**
   The core logic is within the nested `north_perimeter_len` function:

   ```rust
   let north_perimeter_len = |range: Box<dyn Iterator<Item = usize>>| -> usize {
       let mut curr_aseg: Vec<PlotSegment> = Vec::new(); // Active segments from previous line
       let mut next_aseg: Vec<PlotSegment> = Vec::new(); // Active segments from current line

       range.map(|y| { // Iterate through y-coordinates (top-down or bottom-up)
           let sum = rows.range((y, rng_start.clone())..=(y, rng_end.clone())) // Iterate through segments on current line
               .map(|(_, seg)| { // For each segment on the line
                   let overlapping_area = curr_aseg.iter() // Check for overlap with previous line's segments
                       .filter(|aseg| aseg.is_overlapping(seg))
                       .map(|aseg| aseg.get_overlap(seg) as usize)
                       .sum::<usize>();
                   next_aseg.push(seg.clone()); // Add current segment to next line's active segments
                   seg.len() as usize - overlapping_area // Perimeter contribution (length - overlap)
               })
               .sum::<usize>();
           curr_aseg.clear();
           std::mem::swap(&mut next_aseg, &mut curr_aseg); // Update active segments
           sum
       })
       .sum::<usize>()
   };
   ```
   This function iterates line by line, calculating the perimeter contribution of each segment by subtracting its overlap with the previous line.  The `curr_aseg` and `next_aseg` vectors efficiently track active segments across lines.

3. **East and West Perimeter Calculation:** This part is relatively straightforward:

   ```rust
   (y_start..=y_end).map(|y| rows.range((y, rng_start.clone())..=(y, rng_end.clone())).count() * 2).sum::<usize>()
   ```
   It counts the number of segments on each line and multiplies by 2 to account for both east and west sides.

4. **Total Perimeter:**  The final perimeter is computed by summing the north, south, and east/west components:

   ```rust
   north_perimeter_len(Box::new(y_start..=y_end)) + // Top-down perimeter
       north_perimeter_len(Box::new((y_start..=y_end).rev())) + // Bottom-up perimeter
       // ... East and West perimeter calculation ...
   ```

**Core Idea:** The algorithm cleverly uses overlap detection to avoid explicit boundary tracing. By considering overlaps between consecutive lines, it accurately subtracts shared edges, resulting in the correct external perimeter. The top-down and bottom-up traversals ensure complete coverage of all edges.  The use of iterators and functional programming enhances efficiency and readability.
