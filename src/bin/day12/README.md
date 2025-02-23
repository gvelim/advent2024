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

# Understanding the `perimeter` Function: A Step-by-Step Guide

This document explains how the `perimeter` function in the provided Rust code calculates the perimeter of a garden plot.  A plot is represented as a collection of vertical segments.

## 1. The Big Picture: How We Approach Perimeter

Imagine a garden plot made up of little squares.  To find the perimeter, we need to count all the *exposed* edges of these squares.  An edge is exposed if it's not touching another square.

The code breaks this problem down into manageable parts:

*   **Horizontal Edges (North and South):**  We look at each row of the plot and figure out which top and bottom edges of the squares are exposed.  We do this by comparing each row to the row above and below it.
*   **Vertical Edges (East and West):**  We handle the left and right edges of the squares separately. Because of how the input data is structured (segments within a row don't overlap), this is simpler.

## 2. Data Representation: The `Plot`

Before diving into the steps, let's understand how a plot is represented.  A `Plot` is a `BTreeSet` (a sorted set) of tuples: `(usize, PlotSegment)`.

*   `usize`:  The row number (y-coordinate).
*   `PlotSegment`: Represents a continuous horizontal segment within that row.  It has:
    *   `plant()`:  The type of plant (not relevant for perimeter calculation).
    *   `Range`: The start and end x-coordinates of the segment (e.g., `3..7` means the segment covers columns 3, 4, 5, and 6).
    *   `len()`: The length of the segment (e.g., `3..7` has a length of 4).
    *   `is_overlapping()`: Checks if two segments overlap.
    * `get_overlap()`: Calculates by how much two segments overlap.

**Example:**  A `Plot` might look like this (conceptually):

```
Plot:
  Row 1:  Segment covering columns 2..5
  Row 2:  Segment covering columns 1..4
  Row 4:  Segment covering columns 6..8
```

## 3. Step-by-Step Perimeter Calculation

### Step 1: Find the Top and Bottom Rows

We need to know the range of rows we're dealing with.

```rust
let &(y_start, ref seg) = rows.first().unwrap();
let y_end = rows.last().unwrap().0;
```

*   `y_start`:  The row number of the very first segment in the plot (the lowest y-value).
*   `y_end`: The row number of the very last segment in the plot (the highest y-value).

### Step 2: Calculate North and South Perimeters (Horizontal Edges)

This is the core of the algorithm. We use a helper function (closure) called `count_north_perimeter`.  The trick is that we use it *twice*: once to calculate the "north" (top) edges and once to calculate the "south" (bottom) edges.

#### Step 2a: Understanding `count_north_perimeter`

```rust
let count_north_perimeter = | range: Box<dyn Iterator<Item = usize>>| -> usize  {
    range.map(|y| { // For each row 'y' in the given range...
        rows
            .range( (y, rng_start.clone()) ..= (y, rng_end.clone()) ) // Get all segments in row 'y'.
            .map(|(_, seg)| { // For each segment 'seg' in row 'y'...
                // Calculate the exposed part of the NORTH edge of 'seg'.
                seg.len() as usize - rows // Start with the full length of the segment.
                    .range( (y-1, rng_start.clone()) ..= (y-1, rng_end.clone()) ) // Get segments in the row ABOVE 'y'.
                    .filter(|(_,nseg)| nseg.is_overlapping(seg) ) // Keep only the segments that overlap with 'seg'.
                    .map(|(_,nseg)| nseg.get_overlap(seg) as usize) // Get the length of each overlap.
                    .sum::<usize>() // Sum up all the overlaps.
            })
            .sum::<usize>() // Sum the exposed north edges for all segments in row 'y'.
    })
    .sum::<usize>() // Sum the exposed north edges for all rows in the range.
};
```

**Intuition:**

1.  **Iterate through Rows:**  The `range` argument determines which rows we process.
2.  **Get Segments in Current Row:**  For each row `y`, we get all `PlotSegment`s in that row. `rng_start` and `rng_end` are just dummy segments that span the entire possible x-range, ensuring we select *all* segments in row `y`.
3.  **Calculate Exposed North Edge of Each Segment:**
    *   Start with the full length of the segment (`seg.len()`).
    *   Find all segments in the row *above* (`y-1`).
    *   Filter to keep only the segments that *overlap* with the current segment (`seg`).
    *   Calculate the *length* of each overlap.
    *   *Subtract* the total overlap length from the segment's length. This gives us the exposed portion of the north edge.
4.  **Sum Across Segments and Rows:** We sum the exposed north edges for all segments in the row, and then sum those results for all rows in the given range.

#### Step 2b: Calculating the North Perimeter

```rust
count_north_perimeter(Box::new(y_start..=y_end)) // North
```

We call `count_north_perimeter` with the range of rows from `y_start` to `y_end`.  This calculates the exposed *top* edges of all segments in the plot.

**Example:**

```
Row 1:   ####    (Segment: 2..6)
Row 2:  #####    (Segment: 1..6)
```

*   **Row 1:**  The segment in Row 1 has no segments above it.  So, its entire length (4) contributes to the north perimeter.
*   **Row 2:** The segment in row 2 overlaps with row 1 from x=2 to x=5. The overlap is of length 4. The segment is of length 5, and therefore the north perimeter contribution is 5-4=1.

#### Step 2c: Calculating the South Perimeter

```rust
count_north_perimeter(Box::new((y_start..=y_end).rev())) // South
```

We call `count_north_perimeter` *again*, but this time with the row range *reversed* (`(y_start..=y_end).rev()`). This cleverly reuses the same logic to calculate the exposed *bottom* edges.  By reversing the row order, we're effectively treating the row *below* as the "row above" in the `count_north_perimeter` logic.

**Example (Continuing from above):**

*   **Row 2 (Processed First):** The segment in Row 2 has no row below. The segment's entire length (5) goes to the south perimeter.
*   **Row 1 (Processed second):**  The segment in row 1 overlaps with the segment in row 2 with overlap length of 4, so it contributes a length 4-4=0 to the south perimeter.

### Step 3: Calculate East and West Perimeters (Vertical Edges)

```rust
(y_start ..= y_end).map(|y|
    rows.range( (y,rng_start.clone()) ..= (y,rng_end.clone()) ).count() * 2
).sum::<usize>()
```

This part is much simpler.  Because segments within the same row *do not overlap*, we know that *every* segment contributes two vertical edges (one on the left, one on the right).

1.  **Iterate through Rows:**  For each row `y`...
2.  **Count Segments:**  Count the number of segments in that row.
3.  **Multiply by Two:**  Multiply the segment count by 2 (for the two vertical edges per segment).
4.  **Sum:**  Sum the results for all rows.

**Example:**

```
Row 1:  ####
Row 2:  #####
Row 3:  ##
```

*   Row 1: 1 segment * 2 = 2
*   Row 2: 1 segment * 2 = 2
*   Row 3: 1 segment * 2 = 2
*   Total East/West Perimeter: 2 + 2 + 2 = 6

### Step 4: Combine All Perimeters

Finally, we add up the north, south, and east/west perimeters to get the total perimeter:

```rust
// Total Perimeter
north_perimeter + south_perimeter + east_west_perimeter
```

## 4. Complete Example

Let's consider a slightly more complex example:

```
Row 1:  ####       ##
Row 2:  #####    #####
Row 3:              ##
```

1.  **`y_start` = 1, `y_end` = 3**

2.  **North Perimeter:**
    *   Row 1: (4 + 2) = 6 (no row above)
    *   Row 2: (5-4) + (5-2) = 1 + 3 = 4 (overlap with row 1)
    *   Row 3: 0 + (2-2)=0 (overlap with row 2)
    *   Total North: 6 + 4 + 0 = 10

3.  **South Perimeter:**
    *   Row 3: (0 + 2) = 2 (no row below)
    *   Row 2: (5-2) + (5-0) = 3 + 5 = 8
    *   Row 1: 4-4 + 2-2 = 0
    *   Total South: 2 + 8 + 0 = 10

4.  **East/West Perimeter:**
    *   Row 1: 2 segments * 2 = 4
    *   Row 2: 2 segments * 2 = 4
    *   Row 3: 1 segment * 2 = 2
    *   Total East/West: 4 + 4 + 2 = 10

5.  **Total Perimeter:** 10 + 10 + 10 = 30

**Core Idea:** The algorithm cleverly uses overlap detection to avoid explicit boundary tracing. By considering overlaps between consecutive lines, it accurately subtracts shared edges, resulting in the correct external perimeter. The top-down and bottom-up traversals ensure complete coverage of all edges.  The use of iterators and functional programming enhances efficiency and readability.
