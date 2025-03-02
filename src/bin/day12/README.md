## Parsing a Garden Layout: A Functional Approach in Rust

This document explains the logic behind a Rust program designed to parse a garden layout from a textual input. We will explore the problem, the intuition behind the solution, and then delve into the implementation details, highlighting functional programming principles and algorithmic approaches used.

### 1. Problem Description: Representing a Garden as Plots

Imagine a garden represented in a text format, where each line describes horizontal segments of plants. Different characters represent different plant types. For example, an input might look like this:

```
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
```

Each line signifies a scanline of the garden. Contiguous segments of the same character (plant type) on the same line represent plant regions. However, a "plot" in our garden is not just a segment on a single line. Instead, a **plot** is a region of connected, same-plant-type segments across multiple consecutive lines.

Our goal is to parse this textual representation and transform it into a structured format that represents the garden as a collection of distinct plots. For each plot, we want to identify its constituent segments and eventually be able to calculate properties like its area and perimeter.

**Input:** A string representing the garden layout, with each line being a scanline of plant segments.

**Output:** A `Garden` data structure, which is a mapping from a unique plot ID to a `Plot`. A `Plot` is a collection of `PlotSegment`s along with their line numbers, representing a connected region of the same plant type.

### 2. Intuition: Scanline Processing and Plot Merging

To solve this problem, we employ a **scanline processing** approach combined with a **plot merging** strategy. We process the input line by line, maintaining state about the plant segments encountered in the *previous* line.

The core idea is:

1.  **Segment Extraction:** For each input line, we first extract the plant segments.  A segment is defined by its plant type and the range of horizontal positions it occupies on the line.

2.  **Overlap Detection:**  When processing a new line's segments, we check for overlaps with segments from the *previous* line that have the *same plant type*.

3.  **Plot Continuity and Merging:**
    *   If a new segment **overlaps** with one or more segments from the previous line (and has the same plant type), it means this new segment is part of an *existing* plot. We assign it the ID of the overlapping plot (or one of them if multiple overlap – we'll need a strategy for merging in this case).
    *   If a new segment **does not overlap** with any segments from the previous line of the same plant type, it signifies the start of a **new plot**. We assign it a new, unique plot ID.

4.  **Maintaining State Across Lines:** We need to keep track of the segments from the *last* processed line and their associated plot IDs. We'll call this the `LastGardenScanLine`. This structure is crucial for determining plot continuity as we move from one line to the next.

5.  **Plot Consolidation:** If a new segment overlaps with segments from the previous line that belong to *different* plots (due to prior merges in earlier lines), we need to **merge** these plots. This involves combining the segments of the merged plots under a single, "master" plot ID.

By iterating through each line and applying these steps, we gradually build up the `Garden` structure, correctly identifying and merging plots as we process the garden layout.  This approach ensures that connected regions of the same plant type across lines are grouped together into single plots.

### 3. Implementation Step-by-Step: Functional Parsing in Rust

Let's now break down the Rust code, step by step, and see how these ideas are implemented using functional programming patterns.

#### Step 1: Data Structures (`Plot`, `Garden`, `LastGardenScanLine`)

First, let's examine the data structures that organize our garden information.

```rs
use std::{collections::{BTreeMap, BTreeSet}, ops::{Index, RangeInclusive}};
use advent2024::id_generator;
use super::segment::{extract_ranges, PlotSegment, Seed};

pub(super) type Plot = BTreeSet<(usize, PlotSegment)>;
pub(super) type Garden = BTreeMap<usize, Plot>;
```

*   **`PlotSegment`:** (Defined in `super::segment`) Represents a horizontal segment of plants, characterized by its `plant` type (e.g., 'R', 'I', 'C') and a horizontal `range` (e.g., `0..12`).

*   **`Plot`:** `BTreeSet<(usize, PlotSegment)>`.  This represents a single plot in the garden. It's a `BTreeSet` of tuples. Each tuple contains:
    *   `usize`: The line number (vertical position) of the segment.
    *   `PlotSegment`: The plant segment itself.
    The `BTreeSet` ensures segments within a plot are stored in order and avoids duplicates.

*   **`Garden`:** `BTreeMap<usize, Plot>`. This is the top-level structure representing the entire garden. It's a `BTreeMap` where:
    *   `usize`:  A unique plot ID, generated sequentially.
    *   `Plot`: The `Plot` (BTreeSet of segments) associated with that ID.
    Using a `BTreeMap` allows us to easily access plots by their IDs.

```rs
#[derive(Default)]
struct LastGardenScanLine {
    segments: Vec::<(PlotSegment, usize, bool)>,
}
```

*   **`LastGardenScanLine`:** This `struct` manages the state for the previously processed line. It contains:
    *   `segments: Vec::<(PlotSegment, usize, bool)>`: A vector of tuples, where each tuple represents a segment from the last line and includes:
        *   `PlotSegment`: The segment itself.
        *   `usize`: The plot ID to which this segment belongs.
        *   `bool`: A `matched` flag. This flag is used during processing of the *current* line to mark segments from the *previous* line that have been extended by segments in the current line.  Unmatched segments indicate the end of a plot region in the vertical direction.

#### Step 2: `parse_garden` - The Main Parsing Function

The `parse_garden` function is the entry point for parsing the input string and constructing the `Garden`. It uses a functional `fold` operation to process the input lines.

```rs
pub(super) fn parse_garden(input: &str) -> Garden {
    // id generator fn()
    let mut get_new_plot_id = id_generator(0);
    // line counter
    let mut get_line_number = id_generator(0);

    let (mut garden, mut g_line) = input
        .lines()
        .fold((Garden::new(), LastGardenScanLine::default()), |(garden, g_line), input| {
            process_line(
                input,
                garden,
                g_line,
                &mut get_new_plot_id,
                get_line_number()
            )
        });

    // move plot segments remaining to the garden map under their respective plot ID
    let line = get_line_number();
    g_line
        .drain()
        .for_each(|(seg, id, _)| {
            garden.entry(id).or_default().insert((line, seg));
        });

    // return garden map
    garden
}
```

*   **`id_generator`:**  `get_new_plot_id` and `get_line_number` are initialized using an `id_generator` function (not shown in the extract, but assumed to be provided by `advent2024::id_generator`). These are closures that act as simple stateful counters, generating unique IDs for plots and line numbers.

*   **`fold` operation:** The core logic is within the `fold` operation on `input.lines()`.  `fold` is a functional higher-order function that iterates over a collection (lines of input) and accumulates a result.
    *   **Initial Accumulator:** `(Garden::new(), LastGardenScanLine::default())`. We start with an empty `Garden` and an empty `LastGardenScanLine`.
    *   **Accumulator Function:** The closure `|(garden, g_line), input| { ... }` is the function applied to each line. It takes the current accumulator `(garden, g_line)` and the current `input` line. It calls `process_line` (which we will examine next) to process the line and returns the updated accumulator `(garden, new_g_line)`.

*   **`process_line` call:** Inside the `fold`, `process_line` is the function responsible for handling a single input line, updating both the `Garden` and generating a new `LastGardenScanLine` for the next iteration.

*   **Handling Remaining Segments:** After the `fold` is complete (all lines processed), there might be segments left in `g_line` (the `LastGardenScanLine` from the very last processed line). These are segments that were the "bottom-most" parts of plots. We need to add these to the `garden` as well. This is done in the lines after the `fold`.

*   **Return Value:**  Finally, `parse_garden` returns the accumulated `garden`.

#### Step 3: `process_line` - Processing Each Input Line

`process_line` is responsible for processing the segments within a single input line. It extracts segments, associates them with plot IDs, and handles plot continuity and merging.

```rs
fn process_line(
    input: &str,
    garden: Garden,
    g_line: LastGardenScanLine,
    mut get_plot_id: impl FnMut() -> usize,
    line: usize
) -> (Garden, LastGardenScanLine)
{
    let mut new_g_line = LastGardenScanLine::default();
    // for each plant segment
    let (mut garden, mut g_line) = extract_ranges(input)
        .fold((garden, g_line), |(garden, g_line), segment| {
            let (garden, g_line, seg_id) = process_segment(
                &segment,
                garden,
                g_line,
                line,
                &mut get_plot_id
            );
            new_g_line.push(segment, seg_id);
            (garden, g_line)
        });

    // Any scanline segments that didn't match indicate the end of plot region
    // therefore we move such segments to the garden map using their respective plot ID and current line number
    g_line
        .drain_unmatched()
        .for_each(|(seg, id, _)| {
            garden.entry(id).or_default().insert((line, seg));
        });

    (garden, new_g_line)
}
```

*   **`extract_ranges(input)`:** This function (from `super::segment`) is responsible for parsing the input string of a line and extracting `PlotSegment`s from it. It likely identifies contiguous sequences of the same character and creates `PlotSegment` objects with the plant type and range.

*   **Inner `fold`:**  `process_line` itself uses another `fold` operation, this time on the `PlotSegment`s extracted from the current line.
    *   **Initial Accumulator (Inner Fold):** `(garden, g_line)`. It starts with the `garden` and `g_line` passed to `process_line`.
    *   **Accumulator Function (Inner Fold):** `|(garden, g_line), segment| { ... }`. This closure processes each `segment` from the current line. It calls `process_segment` (next step) to determine the plot ID for the segment and update `garden` and `g_line`. It then pushes the segment and its ID into `new_g_line`.

*   **`process_segment` call:** The crucial logic of determining plot IDs, handling overlaps, and merging happens inside `process_segment`.

*   **Handling Unmatched Segments in `g_line`:** After processing all segments of the current line, `g_line.drain_unmatched().for_each(...)` handles segments from the *previous* line that were *not* matched by any segments in the *current* line. These unmatched segments are considered the end of their vertical plot region and are added to the `garden` at the current line number. This ensures that plots that end vertically are properly captured in the `Garden`.

*   **Return Value:** `process_line` returns the updated `(garden, new_g_line)`.  `new_g_line` becomes the `g_line` (representing the "last garden scan line") for the next line's processing in the outer `fold` within `parse_garden`.

#### Step 4: `process_segment` - Handling a Single Segment

`process_segment` is the heart of the plot identification and merging logic. It takes a `PlotSegment` from the current line and determines its plot ID, considering overlaps with segments from the previous line.

```rs
fn process_segment(
    segment: &PlotSegment,
    garden: Garden,
    g_line: LastGardenScanLine,
    line: usize,
    mut get_plot_id: impl FnMut() -> usize
) -> (Garden, LastGardenScanLine, usize)
{
    // find active plots matching this segment
    // matching = (a) overlapping with && (b) have same plant type
    let matched_indices = g_line.overlaps(segment);

    // if empty, then return a new plot ID for the segment
    if matched_indices.is_empty() {
        return (garden, g_line, get_plot_id());
    }

    // otherwise, use the first matching plot ID as the master ID for consolidating all matched plots
    let (_, master_id, _) = g_line[ matched_indices[0] ];

    matched_indices.iter()
        // for each matched plot segment
        .fold((garden, g_line, master_id), |(mut garden, mut g_line, master_id), &index| {
            // flag it as matched; that is, plot region continues to next line
            g_line.flag_matched(index);

            // clone plot segment and plot_id; don't remove it until all remaining new segments are processed
            let (seg, plot_id, _) = g_line[index].clone();

            // move plot segment onto the garden map under the current line number
            garden.entry(plot_id).or_default().insert((line, seg));

            // if plot_id is NOT equal to master_id, then consolidate plots
            if plot_id != master_id {
                // remove plot ID from garden map and hold onto its segments
                let plot = garden.remove(&plot_id).unwrap();
                // merge removed segments into the plot with master ID
                garden.entry(master_id)
                .or_default()
                .extend(plot);
            }
            (garden, g_line, master_id)
        })
}
```

*   **`g_line.overlaps(segment)`:** This method of `LastGardenScanLine` (explained later) finds indices of segments in the `LastGardenScanLine` that *overlap* with the current `segment` and have the *same plant type*. It returns a `Vec<usize>` of indices.

*   **No Overlap (New Plot):** `if matched_indices.is_empty() { ... }`. If there are no overlapping segments from the previous line, it means this `segment` starts a new plot. We call `get_plot_id()` to get a new unique plot ID and return it along with the unchanged `garden` and `g_line`.

*   **Overlap(s) Found (Plot Continuity and Merging):** If there are overlapping segments:
    *   **`master_id` selection:** `let (_, master_id, _) = g_line[ matched_indices[0] ];`. We pick the plot ID of the *first* overlapping segment as the `master_id`. This ID will be used for the current `segment` and for merging any other plots that also overlap.
    *   **Inner `fold` (for handling multiple overlaps and merging):** Another `fold` is used to iterate over the `matched_indices`. This handles cases where the current segment might overlap with multiple segments from the previous line, potentially belonging to different plots (that need to be merged).
        *   **Accumulator (Inner Fold):** `(garden, g_line, master_id)`.  It includes the `garden`, `g_line`, and the `master_id` we selected.
        *   **Accumulator Function (Inner Fold):**  `|(mut garden, mut g_line, master_id), &index| { ... }`.  For each `index` of an overlapping segment:
            *   **`g_line.flag_matched(index)`:** Mark the corresponding segment in `g_line` as `matched`.
            *   **`garden.entry(plot_id).or_default().insert((line, seg));`:** Add the *current* `segment` (from the input line being processed in `process_line`), along with the current line number, to the `Plot` in the `garden` associated with the `plot_id` of the *overlapping* segment from the *previous* line (obtained from `g_line[index]`).
            *   **Plot Merging Logic:** `if plot_id != master_id { ... }`. If the `plot_id` of the currently processed overlapping segment is *different* from the `master_id`, it means we've encountered a plot that needs to be merged into the plot represented by `master_id`.
                *   **`garden.remove(&plot_id).unwrap();`:** Remove the `Plot` associated with `plot_id` from the `garden`.
                *   **`garden.entry(master_id).or_default().extend(plot);`:** Extend the `Plot` associated with `master_id` by adding all the segments from the removed `Plot`. This merges the two plots in the `garden`.
    *   **Return Value:** `process_segment` returns the updated `(garden, g_line, master_id)`. The `master_id` is the plot ID assigned to the processed `segment`.


### 5. Functional Programming Principles Illustrated

This code demonstrates several functional programming principles:

*   **Immutability (Emphasis on Data Flow):** While some data structures like `Garden` and `LastGardenScanLine` are mutated *in place* for efficiency within loops, the overall approach emphasizes data transformations. Functions like `process_line` and `process_segment` take input state and return new state, making the data flow explicit and easier to follow. The use of `fold` reinforces this idea of transforming data through iterations.

*   **Function Decomposition:** The problem is broken down into smaller, well-defined functions (`parse_garden`, `process_line`, `process_segment`, `area`, `perimeter`, etc.). Each function has a specific purpose, making the code modular and easier to understand and test.

*   **Higher-Order Functions:**  The code heavily utilizes higher-order functions like `fold`, `map`, `filter`, `filter_map`, and `for_each`. These functions operate on collections and functions, allowing for concise and declarative data processing. `fold` is particularly prominent for accumulating results across iterations.

*   **Explicit State Management:** State (`garden`, `g_line`, plot IDs, line numbers) is managed explicitly and passed as arguments between functions. This makes the state transitions clear and avoids hidden side effects.

*   **Declarative Style:** The code aims for a declarative style, focusing on *what* to do rather than *how* to do it step-by-step in an imperative manner.  Higher-order functions and functional idioms contribute to this style, making the code more expressive and less verbose than equivalent imperative code might be.

### Conclusion

This Rust code provides a good example of applying functional programming principles to solve a parsing and data processing problem. By using scanline processing, plot merging, and functional constructs like `fold` and higher-order functions, it effectively parses a garden layout and represents it in a structured, functional manner. The code is well-decomposed, emphasizes data transformations, and promotes a more declarative and readable style, making it suitable for educational purposes in functional programming and algorithmic problem-solving.

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
