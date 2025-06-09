# Garden Analysis Program: A Step-by-Step Technical Walkthrough

This document provides an educational walkthrough of a Rust program designed to analyze a text-based garden layout. It identifies distinct regions of the same plant type, calculates their area and perimeter, and visualizes the results. We'll explore the fundamental data structures, the parsing algorithm, and the calculation logic, highlighting key programming principles along the way.

## 1. Solution Intuition

Imagine a garden represented as a grid of characters, where each character signifies a different plant type. The goal is to find contiguous areas of the *same* plant and measure them. The core idea is to process the garden grid line by line, identifying segments of identical plants and then connecting these segments vertically across lines to form complete "plots".

At a high level, the program performs the following steps:
1. Reads the garden layout from a text input.
2. Parses each line to identify horizontal segments of plants.
3. Processes these segments line by line, determining which ones belong to existing plots or indicate the start of new plots.
4. Merges plots when segments connect previously separate regions.
5. Once the entire garden is parsed, calculates the area and perimeter for each identified plot.
6. Provides a visual representation of the identified plots for debugging and understanding.
7. Calculates a total "cost" based on the area and perimeter of each plot.

Let's break down the implementation step by step.

## 2. Step 1: Representing Horizontal Segments

The most basic unit of information on a single line is a continuous stretch of the same plant. We need a way to represent this.

```rust
pub(super) struct PlotSegment(char, Range<Seed>);
```

**Insight:** A `PlotSegment` captures a horizontal sequence of identical plants on a single row. It stores the `char` representing the plant type and a `Range<Seed>` indicating the start and end (exclusive) column indices where this plant appears on that line. The `Seed` type (aliased from `u16`) is used for column indices.

**Reasoning:** This structure is a clean abstraction for the horizontal components of our plots. By storing the plant type and its horizontal range, we can easily check for overlaps, calculate segment length, and extract necessary information for connecting segments vertically.

Key methods implemented for `PlotSegment` include:

```rust
pub(super) fn contains(&self, seed: Seed) -> bool {
    self.1.contains(&seed)
}
pub(super) fn new(plant: char, range: Range<Seed>) -> Self {
    PlotSegment(plant, range)
}
pub(super) fn plant(&self) -> char {
    self.0
}
pub(super) fn start(&self) -> Seed {
    self.1.start
}
pub(super) fn end(&self) -> Seed {
    self.1.end
}
pub(super) fn len(&self) -> Seed {
    self.1.end as Seed - self.1.start as Seed
}
pub(super) fn is_overlapping(&self, other: &Self) -> bool {
    self.start() < other.end() && self.end() > other.start()
}
pub(super) fn get_overlap(&self, other: &Self) -> Seed {
    // find the absolute overlap between the two segments
    let start = self.start().max(other.start());
    let end = self.end().min(other.end());
    end - start
}
pub(super) fn count_horizontal_edges<'a>(&self, row_segs: impl Iterator<Item = &'a (usize, PlotSegment)>) -> usize {
    row_segs
        .take_while(|(_,nseg)| nseg.1.start < self.1.end)
        .filter(|(_,nseg)| nseg.is_overlapping(self))
        .map(|(_,nseg)| nseg.get_overlap(self) as usize)
        .sum::<usize>()
}
```

These methods provide the core functionality for working with horizontal segments: checking if a specific column is within the segment (`contains`), creating a new segment (`new`), accessing segment properties like plant type, start, and end columns (`plant`, `start`, `end`), calculating length (`len`), checking for overlaps (`is_overlapping`), quantifying overlap amount (`get_overlap`), and counting common horizontal edges relative to other segments in a row (`count_horizontal_edges`).

Additionally, `PlotSegment` implements the `Debug` trait for easy visualization, and the `Ord` and `PartialOrd` traits, which are crucial for sorting segments. The `Ord` implementation sorts primarily by the segment's start column, then by its end column, which is necessary for storing and retrieving segments efficiently in ordered collections like `BTreeSet` within the `Plot` and `Garden` structures.

## 3. Step 2: Parsing Lines into Segments

The process of parsing the garden layout from a raw string input into a structured `Garden` involves several steps, primarily handled within the `parser` module. The first step, within the parser, is to break down each individual line into horizontal segments using the `extract_ranges` function:

```rust
pub(super) fn extract_ranges(line: &str) -> impl Iterator<Item = PlotSegment> {
    let mut idx = 0;
    line.as_bytes()
        .chunk_by(|a, b| a == b)
        .map(move |chunk| {
            let start = idx;
            idx += chunk.len() as Seed;
            PlotSegment::new(chunk[0] as char, start..idx)
        })
}
```

**Insight:** The `extract_ranges` function iterates through the bytes of a line, using the `chunk_by` method to group consecutive identical characters. For each chunk, it creates a `PlotSegment` with the character and its corresponding column range.

**Reasoning:** This provides a clean and efficient way to break down a raw text line into its fundamental horizontal components (`PlotSegment`s). For example, the line "RRRRIICCFF" is parsed into the following `PlotSegment`s: `('R':0..4)`, `('I':4..6)`, `('C':6..8)`, `('F':8..10)`. These segments are the building blocks for identifying and assembling larger plots.

## 4. Step 3: Grouping Segments into Plots

A single plot can span multiple lines and consist of multiple horizontal segments on a single line (if there are gaps of different plants within the plot's overall horizontal span). We need a data structure to represent these multi-line, potentially discontinuous, regions.

```rust
#[derive(Default)]
pub(super) struct Plot {
    rows: BTreeSet<(usize, PlotSegment)>
}
```

**Insight:** A `Plot` is defined as a collection of `PlotSegment`s, organized by their row number (`usize`). Using a `BTreeSet<(usize, PlotSegment)>` is key here because it automatically keeps the segments sorted first by row number (`usize`) and then by the `PlotSegment`'s ordering (which is based on the start and end columns, as defined in `impl Ord for PlotSegment`).

**Reasoning:** The `BTreeSet` provides efficient insertion and ordered iteration. Sorting segments by row and column range makes it easy to traverse the plot vertically (row by row) and horizontally (segment by segment within a row), which is essential for operations like calculating the perimeter and for visualization.

The `Plot` struct also contains methods for adding segments (`insert`), merging with other plots (`extend`), calculating its `area`, and calculating its `perimeter`.

```rust
pub fn extend(&mut self, plot: Plot) {
    self.rows.extend(plot.rows);
}
```

**Insight:** The `extend` method allows merging another `Plot`'s segments into the current one. This is used when two separate plots are found to connect via a new segment, requiring them to be consolidated into a single plot.

**Reasoning:** By simply extending the `BTreeSet`, we leverage its properties to handle duplicates and keep the combined segments sorted efficiently.

```rust
pub fn iter(&self) -> impl Iterator<Item = &(usize, PlotSegment)> {
    self.rows.iter()
}
```

**Insight:** The `iter` method provides an iterator over the segments within the plot, allowing easy access to its constituent parts.

**Reasoning:** Iterators are a standard Rust pattern for processing collections. This provides a convenient way to access all segments of a plot for calculations or visualization.

## 5. Step 4: Assembling the Garden Across Lines

This is the most complex part of the program: connecting the horizontal `PlotSegment`s from different lines to form coherent, multi-line `Plot`s and managing plot identities, including merging plots that become connected by a new segment.

The main structure holding all identified plots is the `Garden`:

```rust
#[derive(Default)]
pub(super) struct  Garden {
    plots: HashMap<usize, Plot>
}
```

**Insight:** The `Garden` is a map (`HashMap`) where the keys are unique plot IDs (`usize`) and the values are the corresponding `Plot` structures. We use a `HashMap` for efficient O(1) average time complexity for insertion and retrieval, which is beneficial for performance. The `Garden` struct provides an `iter` method to iterate over the contained plots:

```rust
    pub(super) fn iter(&self) -> impl Iterator<Item = (&usize, &Plot)> {
        self.plots.iter()
    }
```

It also implements the `Index` trait for convenient access to plots by their ID:

```rust
impl Index<&usize> for Garden {
    type Output = Plot;

    fn index(&self, index:&usize) -> &Self::Output {
        &self.plots[index]
    }
}
```

**Reasoning:** Using a map allows quick access to any plot given its ID. The `iter` method and `Index` implementation provide idiomatic Rust ways to interact with the collection of plots.

The primary function for parsing the input string into a `Garden` is the `parse` method on the `Garden` struct itself. This method delegates the core parsing logic to the `parser` module:

```rust
impl Garden {
    // ... other methods ...
    pub(super) fn parse(input: &str) -> Garden {
        Garden { plots: parser::parse_plots(input) }
    }
}
```

The actual line-by-line processing and plot assembly is handled by the `parser::parse_plots` function:

```rust
// garden is a collection of plots expressed by a 1 or more overlapping vertical segments
// parser extracts and composes plots per scanline
// a plot is composed out of multiple scanlines
pub fn parse_plots(input: &str) -> HashMap<usize,Plot> {
    // id generator fn()
    let mut get_new_plot_id = id_generator(0);
    // line counter
    let mut get_line_number = id_generator(0);

    let (mut plots, mut g_line) = input
        .lines()
        .fold((HashMap::<usize, Plot>::new(), LastGardenScanLine::default()), |(plots, g_line), input| {
            process_line(
                input,
                plots,
                g_line,
                &mut get_new_plot_id,
                get_line_number()
            )
        });

    // move plot segments remaining to the garden map under their respective plot ID
    push_segments(&mut plots, g_line.drain(), get_line_number());

    // return garden map
    plots
}
```

**Insight:** `parser::parse_plots` uses a `fold` operation over the input lines. This is a common functional pattern to process a sequence while maintaining and updating an accumulator. Here, the accumulator holds the `plots` identified so far (`HashMap<usize, Plot>`) and the state from the *last* line processed (`LastGardenScanLine`). The `fold` applies the `process_line` function from the `parser` module to each line, updating the accumulator state.

**Reasoning:** Using `fold` provides a clean, iterative way to build the `Garden` by processing lines in order. It ensures that the state from the previous line (`LastGardenScanLine`) is correctly passed to `process_line` for the current line.

### Processing Each Line: The `process_line` Function

The `process_line` function, located within the `parser` module, is called by `parser::parse_plots` for every line of the input. Its role is to determine how the segments on the *current* line connect to the plots identified on the *previous* line.

```rust
fn process_line(
    input: &str,
    plots: HashMap<usize, Plot>, // plots collected so far
    g_line: LastGardenScanLine, // state from the previous line
    mut get_plot_id: impl FnMut() -> usize,
    line: usize // current line number
) -> (HashMap<usize, Plot>, LastGardenScanLine) // updated plots and state for the next line
{
    let mut new_g_line = LastGardenScanLine::default();

    // Extract segments from the current line and process each one
    let (mut plots, mut g_line) = extract_ranges(input)
        .fold((plots, g_line), |(plots, g_line), segment| {
            // process segment against the last Garden Scan Line
            let (
                mut plots,
                mut g_line,
                seg_id, // The plot ID assigned to this segment
                depr_ids // IDs of plots that were merged into seg_id
            ) = process_segment( &segment, plots, g_line, line, &mut get_plot_id );

            // Add the current segment with its assigned ID to the state for the *next* line
            new_g_line.push(segment, seg_id);

            // Handle any plot IDs that were deprecated (merged) by this segment
            if let Some(ids) = depr_ids {
                ids.into_iter()
                    .all(|plot_id| {
                        // ... merge logic using push_segments and find_replace_plot_id ...
                        let plot = plots.remove(&plot_id).unwrap();
                        plots.entry(seg_id).or_default().extend(plot);
                        g_line.find_replace_plot_id(plot_id, seg_id);
                        new_g_line.find_replace_plot_id(plot_id, seg_id)
                    });
            }
            (plots, g_line) // Pass updated state to the next iteration of the fold
        });

    // Any segments from the *previous* line that didn't get matched means their plot ended
    // So, move these unmatched segments to the main plots map
    push_segments(&mut plots, g_line.drain_unmatched(), line);

    // Return the updated plots and the new state for the next line
    (plots, new_g_line)
}
```

**Insight:** `process_line` first extracts all `PlotSegment`s from the current `input` string using the `extract_ranges` function we discussed earlier. It then iterates through these segments using another `fold`. For each segment, it consults the `LastGardenScanLine` (`g_line`) from the *previous* row and calls `process_segment` to determine which plot the current segment belongs to. It also builds the `new_g_line` to be used as the `LastGardenScanLine` for the *next* iteration (the next row). Finally, it handles any segments from the *original* `g_line` that were *not* matched, indicating the end of a plot's vertical extent in those positions.

**Reasoning:** This function orchestrates the segment-by-segment analysis for a single row. By keeping track of both the previous row's state (`g_line`) and building the next row's state (`new_g_line`), it correctly propagates plot identities and handles plot termination. The use of `fold` within `process_line` for processing segments is another example of applying functional patterns to manage iterative state updates.

### Determining Plot Identity and Merging: The `process_segment` Function

The `process_segment` function, located within the `parser` module, is the heart of the plot identification and merging logic. It is called by `process_line` and takes a single `segment` from the *current* line, comparing it against the segments recorded in the `g_line` (the state from the *previous* line).

```rust
// for each new segment identify the plot that is overlapping with and assign the segment the plot's ID
fn process_segment(
    segment: &PlotSegment, // The segment from the current line being processed
    plots: HashMap<usize, Plot>, // The collection of all plots found so far (passed by value)
    g_line: LastGardenScanLine, // The segments and IDs from the previous line (passed by value)
    line: usize, // The current line number
    mut get_plot_id: impl FnMut() -> usize // Function to get a new unique plot ID
) -> (HashMap<usize, Plot>, LastGardenScanLine, usize, Option<Vec<usize>>)
    {
        // find active plots matching this segment
        // matching = (a) overlapping with && (b) have same plant type
        let mut matched = g_line.overlaps(segment);

        // Scenario 1: No overlap with any segment on the previous line.
        // This segment starts a brand new plot.
        if matched.is_empty() {
            // Return the current plots, g_line, a new plot ID, and no deprecated IDs.
            return (plots, g_line, get_plot_id(), None);
        }

        // Scenario 2: One or more overlaps found on the previous line.
        // This segment connects to existing plot(s).

        // Use the smallest plot ID among the matched segments as the master ID for potential merging.
        // Critical insight: The plot with the smallest ID is guaranteed to have been created first.
        // When merging, the older plot (smallest ID) absorbs the newer one(s).
        matched.sort_by_key(|(_,id)| *id); // Sort by plot ID to find the smallest
        let (_, master_id, _) = g_line[ matched[0].0 ]; // Get the smallest ID from the first matched segment

        // Now, iterate through all the matched segments on the previous line.
        // Use fold to update the garden, g_line, master_id, and collect deprecated IDs.
        matched.iter()
            .fold((plots, g_line, master_id, None), |(mut garden, mut g_line, master_id, mut depr_ids), &(index, _id)| {
                // For each matched segment from the previous line:
                // Flag it as matched, indicating its plot continues to the current line.
                g_line.flag_matched(index);

                // Get the segment and its original plot ID from the previous line's state.
                let (seg, plot_id, _) = g_line[index].clone();

                // Add this segment from the previous line (at the previous line's y-coordinate)
                // to the *main* garden map under its plot_id.
                // Note: We use line-1 as the y-coordinate because the g_line contains segments
                // from the line *before* the current 'line'.
                garden.entry(plot_id).or_default().insert(line-1, seg);

                // If the matched segment's plot ID is NOT the chosen master_id, it means this segment
                // connected two previously separate plots. The plot with 'plot_id' is now deprecated.
                if plot_id != master_id {
                    // Add this deprecated plot ID to our list.
                    depr_ids.get_or_insert_default().push(plot_id);
                }

                // Continue the fold with the potentially updated garden and g_line state.
                (garden, g_line, master_id, depr_ids)
            })
    }
```

**Insight:** `process_segment` implements the core logic for plot continuity and merging. It first checks if the current segment overlaps with any segments on the previous line (`g_line`) that have the same plant type.
- If there's no overlap, it's a new plot, and a unique ID is generated.
- If there are overlaps, it identifies the *smallest* plot ID among the overlapping segments from the previous line. This smallest ID becomes the `master_id` for the current segment and all other plots it connects to. The segments on the previous line that were matched are marked as such in `g_line`. Critically, if a segment on the current line overlaps with segments from *multiple* plots on the previous line, those plots are now considered connected and must be merged under the `master_id`. The IDs of the plots being merged *into* the master are collected as `depr_ids`.

**Reasoning:** This function ensures that horizontal segments are correctly grouped vertically into plots. The logic handles the creation of new plots, the continuation of existing plots, and the merging of plots that become connected. The `depr_ids` mechanism is essential for the calling `process_line` function to finalize the merge by transferring all segments from the deprecated plots into the master plot in the main `plots` map, and updating any references to deprecated IDs in the `LastGardenScanLine` structs.

## 6. Step 5: Calculating Area, Perimeter, and Sides

Once the `Garden` is fully parsed and all `Plot`s are assembled, we can calculate their properties for both Part 1 and Part 2 of the puzzle.

### Area Calculation

```rust
    pub(super) fn area(self: &Plot) -> usize {
        self.rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
    }
```

**Insight:** The area of a plot is simply the sum of the lengths of all its horizontal segments.

**Reasoning:** By definition, each `PlotSegment` represents a rectangular area of a single plant type on its row. Summing the lengths of all such segments within a `Plot` gives the total count of individual plant cells, which is the area. The `area` method iterates through the `BTreeSet` of segments and sums their lengths using a standard iterator pattern.

### Part 1: Perimeter Calculation

Calculating the perimeter is more involved:

```rust
    pub(super) fn perimeter_count(&self) -> usize {
        let y_range = self.get_plot_y_range();

        self.edge_count_north_south(y_range.clone())
            // a row may contain 1 or more segments of the same plot with gaps in between
            // plot segments in the same raw are *isolated*, that is, they are never next to each other, end of first != start of second
            // therefore vertical segments per row is 2 * number of segments
            // therefore sum(row) == total segments * 2
            + self.rows.len() * 2
    }
```

**Insight:** The perimeter calculation involves two parts:
1. Horizontal edges: These are counted by comparing segments on adjacent lines (North/South). The `edge_count_north_south` function handles this.
2. Vertical edges: These occur at the start and end of each `PlotSegment` on its row (Left/Right). Since segments on the same row for the same plot are guaranteed not to touch horizontally, each segment contributes a left and a right edge, totaling `self.rows.len() * 2`.

The `perimeter_count` method also uses helper methods `get_plot_y_range` to find the vertical bounds of the plot and `get_plot_bounding_segs` to get sentinel segments for range queries.

```rust
    fn edge_count_north_south(&self, lines: impl Iterator<Item = usize>) -> usize  {
        let (west_bound, east_bound) = self.get_plot_bounding_segs();

        let mut lines = lines.peekable();
        let Some(&start) = lines.peek() else { panic!("perimeter_counter(): Empty 'y' range")};

        // we fold each iteration using (above, current, below and sum) as input parameters
        // this reduces the number of BTreeSet queries from 3 down to 1 per iteration
        let (_, _, _, sum) = lines
            .fold(
                (
                    self.rows.range((start-1, west_bound.clone())..=(start-1, east_bound.clone())),
                    self.rows.range((start, west_bound.clone())..=(start, east_bound.clone())),
                    self.rows.range((start+1, west_bound.clone())..=(start+1, east_bound.clone())),
                    0
                ),
                |( above_row, current_row, below_row, sum), y| {

            // sum non-overlapping units of current raw against above and below segment lines
            let new_sum = sum + current_row.clone()
                .map(|(_, seg)| {
                    // count overlapping units above the line
                    let above = above_row.clone()
                        .filter(|(_,nseg)| nseg.is_overlapping(seg))
                        .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                        .sum::<usize>();

                    // count overlapping units under the line
                    let below = below_row.clone()
                        .filter(|(_,nseg)| nseg.is_overlapping(seg))
                        .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                        .sum::<usize>();

                    // non-overlapping  = (segment length - above overlaping units) + (segment length - above overlaping units) =>
                    // non-overlapping = 2 * segment lengths - above - below overlapping units
                    2 * seg.len() as usize - above - below
                })
                .sum::<usize>();

            (
                // contains y becomes y-1 in next cycle
                current_row,
                // contains y+1 becomes y in next cycle
                below_row,
                // we need y+2 so it becomes y+1 in next cycle
                self.rows.range((y+2, west_bound.clone())..=(y+2, east_bound.clone())),
                new_sum
            )
        });
        sum
    }
```

**Insight:** This function calculates the contribution of horizontal (North/South) edges to the perimeter. It iterates through each row (`y`) containing segments of the plot, bounded by the overall `y_range` of the plot. For each segment on the current row (`y`), it calculates the non-overlapping length against segments on the row *above* (`y-1`) and the row *below* (`y+1`). The non-overlapping portion represents the horizontal perimeter edge at that location. The calculation `2 * seg.len() - above - below` efficiently sums the horizontal edges (both north and south faces) for that segment. The use of `seg.count_horizontal_edges` method abstracts the overlap calculation against adjacent rows. The `fold` implementation cleverly updates the ranges for the `above_row`, `current_row`, and `below_row` iterators in each step, avoiding repeated lookups for the same row data.

**Reasoning:** By summing the non-overlapping vertical lengths for every segment against its neighbors above and below, and adding the fixed horizontal edges (two per segment, counted in the `perimeter_count` method), we get the total perimeter. This approach correctly handles complex shapes with holes or indentations by precisely accounting for which parts of a segment's edges are *not* adjacent to another segment of the same plot.
### Part 2: Sides Calculation

For Part 2 of the puzzle, we need to count the number of distinct sides rather than the total perimeter length. This is accomplished by the `sides_count` method:

```rust
pub(super) fn sides_count(&self) -> usize {
    let (west, east) = self.get_plot_bounding_segs();
    let start = self.rows.first().expect("Plot Empty!").0;
    // reuse HashSet across iterations so to avoid heap allocation overhead
    let mut corners = HashSet::<u16>::with_capacity(10);

    let (last_line, sum) = self.get_plot_y_range()
        .fold((
            // y and y-1 segment lines
            self.rows.range((start,west.clone())..(start,east.clone())), // init condition -> first line
            0,  // accumulator : total number of corners
        ),
        |(segments, sum), y|
        {
            // clear corners HashSet
            corners.clear();
            // we count all unique segment projections between the 2 lines
            segments
                .flat_map(|(_,s)| [s.start()*10, s.end()*10 - 1])
                .for_each(|p| {
                    if !corners.insert(p) { // have we seen this projection before ?
                        corners.remove(&p); // cancel out projection seen
                    }
                });
            (
                self.rows.range((y, west.clone())..(y+1,east.clone())),  // // shift 2 lines Window by 1
                sum + corners.len()  // count unique projections aka corners
            )
        });
    // add 2 corners per residual segment in the last line
    sum + last_line.count() * 2
}
```

**Insight:** The key insight for Part 2 is that for any simple polygon (or a collection of simple polygons, which our plots effectively are), the number of sides is equal to the number of "convex" or "concave" corners where the boundary changes direction. Instead of counting individual units of perimeter length, we count these distinct corner locations.

**Reasoning:** The algorithm leverages this corner-counting principle. It works by iterating through the rows containing segments of the plot, considering *a pair of rows* at a time. For each pair of lines, it identifies all the horizontal column positions where a segment starts or ends on either line.

Therefore a corner is defined when the below statement is `True`
```
(SegA.start != SegB.start) OR (SegB.end != SegB.end())
```

As a consequence of the above, **matching** start **or** end positions cancel each other out.

```
Given the following plot
Line 1: ...XXXXX.....XXX..
Line 2: ...XXX..XXX..XX...
Line 4: ....XXXXXXXXXX....

Calculation of corners by counting unique segment projections between two lines
        012345678901234567
No Line
↕ Proj  ---s---e-----s-e-- = 4 corners; 2 (s)tart + 2 (e)nd projections
           ^   ^     ^ ^
Line 0: ...XXXXX.....XXX..
           v   v     v v
↕ Proj  ---x-e-es-e--xee-- = 6 corners; 8 - 2 cancelled projections
           ^ ^  ^ ^  ^^
Line 1: ...XXX..XXX..XX...
           v v  v v  vv
↕ Proj  ---sse--s-e--se--- = 8 corners
                     e
            ^        ^
Line 2: ....XXXXXXXXXX....
No Line     v        v     = 2 corners
                           = 20 total corners
```

By using a `HashSet` as a toggle mechanism (`insert` if not present, `remove` if present), it efficiently identifies which of these start/end positions are *unpaired* between the two lines. An unpaired position signifies a point where the vertical boundary of the plot changes direction from one line to the next – i.e., a corner.

The multiplication and subtraction on the segment start and end positions (`s.start()*10`, `s.end()*10 - 1`) is a clever way to ensure that a segment ending at column `X` (the non-inclusive bound of a `Range`) and a segment starting at column `X` (the inclusive bound of a `Range`) are treated as events at distinct vertical lines in the grid, **preventing accidental cancellation**.

The total corner count accumulated across all pairs of lines gives the sum of corners along the internal vertical boundaries. Finally, we add the corners formed by the bottom edges of the segments on the plot's last row, as these always contribute two corners per segment since there's no row below to potentially align with their boundaries. This method correctly counts all corners for complex plot shapes, including those with internal holes.

## 7. Step 6: Visualizing the Results

Understanding the output of a spatial algorithm is crucial for debugging and verification. The program includes `Debug` implementations that provide a visual representation.

```rust
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
            }
            // After processing all segments for a scanline,
            // Write the ANSI escape code to reset text attributes (back to default)
            // and add a new line
            writeln!(f, "\x1B[0m")?;
        }
        // Return Ok(()) to indicate successful formatting.
        Ok(())
    }
}
```

**Insight:** The `Debug` implementation for `Garden` generates a colored grid representation of the garden using ANSI escape codes and the `colored` crate. Each plot is rendered with a deterministic unique background color based on its ID, displaying the plant character. The implementation collects all segments into a `BTreeSet` to sort them by row and then column, ensuring correct scanline rendering. This visualization includes the row number and provides a clear visual mapping of the identified plots.

**Reasoning:** This visualization allows you to see the identified plots overlaid on the garden grid with distinct colors. You can visually inspect if the plots were correctly identified, if merges happened as expected, and if the shapes match the input. This is invaluable for debugging the complex parsing logic, especially with spatial data.

There is also a `Debug` implementation for `Plot` in `plot.rs` which lists the segments for a single plot organized by row number, providing a more detailed look at the structure of an individual plot.

```rust
impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ... rendering code ...
    }
}
```

## 8. Step 7: The Main Program Loop

The `main` function ties all these components together to execute both Part 1 and Part 2 of the puzzle.

```rust
fn main() {
    let args = std::env::args();
    let input = std::fs::read_to_string(
        match args.skip(1).next() {
            None => "src/bin/day12/input.txt".to_string(),
            Some(str) => str,
        }
    ).unwrap();

    let garden = Garden::parse(&input);

    let calculate_cost = |garden: &Garden, plot_cost_fn: for<'a> fn((&'a usize, &'a Plot)) -> usize| -> usize {
        garden
        .iter()
        .map(plot_cost_fn)
        .sum::<usize>()
    };

    let t = time::Instant::now();
    let total_1 = calculate_cost(&garden, |(_, plot)| plot.area() * plot.perimeter_count());
    let el_puzzle_1 = t.elapsed();

    let total_2 = calculate_cost(&garden, |(_, plot)| plot.area() * plot.sides_count());
    let el_puzzle_2 = t.elapsed() - el_puzzle_1;

    println!("{:?}", &garden);
    let el_debug = t.elapsed() - el_puzzle_2 - el_puzzle_1;

    println!("Part 1 - Garden total cost : {total_1} = {el_puzzle_1:?}");
    println!("Part 2 - Garden total cost : {total_2} = {el_puzzle_2:?}");
    println!("Rendered Garden in {el_debug:?}");

    assert_eq!(total_1, 1533024);
    assert_eq!(total_2, 910066);
}
```

**Insight:** The `main` function serves as the program's entry point and now handles both parts of the puzzle. It reads the garden layout from a file (defaulting to `src/bin/day12/input.txt` or accepting a command-line argument), calls `Garden::parse` to construct the garden representation, and then calculates the total cost for both parts using a reusable `calculate_cost` closure.

**Reasoning:** This function orchestrates the overall flow: input reading, parsing, calculation for both parts, visualization, and output. The key improvements include:

1. **Reusable Cost Calculation:** The `calculate_cost` closure accepts a function parameter that determines how to calculate the cost for each plot. This allows the same iteration logic to be used for both Part 1 (`plot.area() * plot.perimeter_count()`) and Part 2 (`plot.area() * plot.sides_count()`).

2. **Cumulative Timing:** The timing is handled cumulatively with `el_puzzle_2` calculated as the difference from the total elapsed time minus `el_puzzle_1`, and `el_debug` calculated by subtracting both puzzle timing measurements from the total elapsed time.

3. **Dual Assertions:** The function includes assertions for both expected results (`total_1 = 1533024` for Part 1, `total_2 = 910066` for Part 2) to verify correctness.

4. **Clear Output:** The results are clearly labeled to distinguish between Part 1 and Part 2 solutions.

The use of closures and function parameters demonstrates a functional approach to code reuse, avoiding duplication while maintaining clarity about the different calculation methods used for each part of the puzzle.

## 9. Design Decisions and Trade-offs

Throughout the development of this program, several design choices were made, each with its own trade-offs:

1.  **`BTreeSet` for Plot Rows:** Storing `PlotSegment`s in a `BTreeSet` within the `Plot` struct ensures they are always sorted by row and then by column range.
    *   *Benefit:* This simplifies algorithms that need ordered access (like perimeter calculation) and makes visualization consistent. Iteration is efficient in sorted order.
    *   *Trade-off:* Insertion into a `BTreeSet` is typically `O(log n)`, which is less performant than `O(1)` for `Vec` or `HashMap`. The requirement for `PlotSegment` to implement `Ord` adds a slight complexity to the type definition.

2.  **`HashMap` for Garden Plots:** Using a `HashMap` to map plot IDs to `Plot` structures (`Garden::plots`).
    *   *Benefit:* Provides efficient `O(1)` average time complexity for lookup, insertion, and removal of plots by ID, which is crucial during the parsing and merging process. This was chosen over `BTreeMap` for better average performance.
    *   *Trade-off:* `HashMap` does not guarantee any order for keys, unlike `BTreeMap`. However, for the requirements of accessing plots by ID during parsing, the average time complexity benefit outweighs the lack of ordering.

3.  **Line-by-Line Processing with `LastGardenScanLine`:** The algorithm processes the garden one line at a time, relying on state from the *previous* line (`LastGardenScanLine`) to determine connectivity and perform merges.
    *   *Benefit:* This avoids loading the entire potentially very large garden grid into memory at once, making it memory-efficient for height. The logic focuses on local interactions between adjacent lines.
    *   *Trade-off:* The state management (`LastGardenScanLine`, handling deprecated IDs, passing structs by value in the fold) adds significant complexity to the `process_line` and `process_segment` functions.

4.  **Merging Logic based on Smallest ID:** When a segment connects multiple existing plots, they are merged under the plot ID that is numerically smallest among the connected plots.
    *   *Benefit:* Provides a simple and deterministic rule for merging, ensuring that plots are correctly consolidated regardless of the order segments are processed within a line.
    *   *Trade-off:* Requires keeping track of and updating deprecated IDs throughout the process, adding complexity to the state management.

## 10. Conclusion

This garden analysis program showcases several important programming principles:

1.  **Abstraction:** Breaking the problem down into logical layers: `PlotSegment` (horizontal piece), `Plot` (multi-line region), and `Garden` (collection of plots) simplifies complexity.
2.  **Data Structure Choice:** Selecting appropriate data structures (`BTreeSet`, `BTreeMap`, `Vec`) based on access patterns and ordering requirements is vital for both correctness and performance.
3.  **Algorithmic Design:** Implementing a line-by-line processing algorithm with state management (`LastGardenScanLine`) effectively handles the 2D spatial problem while managing memory.
4.  **Functional Programming:** Utilizing iterators (`map`, `fold`, `chunk_by`) leads to concise and expressive code for data transformation and processing.
5.  **Visualization:** Implementing `Debug` traits for visual output is a powerful debugging tool for spatial or complex data structures.

By combining these principles, the program successfully identifies, measures, and visualizes distinct plant plots in a 2D grid, demonstrating a robust approach to this type of spatial analysis problem.
