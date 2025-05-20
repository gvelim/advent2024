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
// ...
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
```

These methods allow us to determine if a specific column is part of the segment, calculate its length, check for horizontal overlaps with other segments, and quantify the degree of overlap. The `Ord` and `PartialOrd` implementations are crucial for sorting `PlotSegment`s, particularly when stored in ordered collections like `BTreeSet`.

## 3. Step 2: Parsing Lines into Segments

The raw input is a string containing many lines. We need to transform each line into a sequence of `PlotSegment`s.

```rust
pub(super) fn extract_ranges(line: &str) -> impl Iterator<Item = PlotSegment> {
    let mut idx = 0;
    line.as_bytes()
        .chunk_by(|a, b| a == b)
        .map(move |chunk| {
            let start = idx;
            idx += chunk.len() as Seed;
            PlotSegment(chunk[0] as char, start..idx)
        })
}
```

**Insight:** The `extract_ranges` function iterates through the bytes of a line, using the `chunk_by` iterator adapter from the `itertools` crate (implicitly used here, although `as_bytes().chunk_by` is a standard slice method) to group consecutive identical characters. For each chunk of identical characters, it creates a `PlotSegment` with the character and the corresponding column range.

**Reasoning:** This provides a clean and efficient way to break down a raw text line into its fundamental horizontal components (`PlotSegment`s). For example, the line "RRRRIICCFF" is parsed into the following `PlotSegment`s: `('R', 0..4)`, `('I', 4..6)`, `('C', 6..8)`, `('F', 8..10)`.

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

## 5. Step 4: Assembling the Garden Across Lines

This is the most complex part of the program: connecting the horizontal `PlotSegment`s from different lines to form coherent, multi-line `Plot`s and managing plot identities, including merging plots that become connected by a new segment.

The main structure holding all identified plots is the `Garden`:

```rust
#[derive(Default)]
pub(super) struct  Garden {
    plots: BTreeMap<usize, Plot>
}
```

**Insight:** The `Garden` is a map (`BTreeMap`) where the keys are unique plot IDs (`usize`) and the values are the corresponding `Plot` structures. `BTreeMap` keeps plots sorted by ID, although the order of IDs isn't strictly necessary for correctness, it can be helpful for consistent debugging output.

**Reasoning:** Using a map allows quick access to any plot given its ID. The plot IDs are generated sequentially as new plots are discovered using a simple counter provided by the `id_generator` helper function.

The core logic for garden assembly resides in the `parse_garden` function, which orchestrates the processing of each line using helper functions `process_line` and `process_segment`.

Let's look at `parse_garden`:

```rust
    pub(super) fn parse_garden(input: &str) -> Garden {
        // id generator fn()
        let mut get_new_plot_id = id_generator(0);
        // line counter
        let mut get_line_number = id_generator(0);

        let (mut plots, mut g_line) = input
            .lines()
            .fold((BTreeMap::<usize, Plot>::new(), LastGardenScanLine::default()), |(plots, g_line), input| {
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
        Garden { plots }
    }
```

**Insight:** `parse_garden` uses a `fold` operation over the input lines. This is a common functional pattern to process a sequence while maintaining and updating an accumulator. Here, the accumulator holds the `plots` identified so far (`BTreeMap<usize, Plot>`) and the state from the *last* line processed (`LastGardenScanLine`). The `fold` applies the `process_line` function to each line, updating the accumulator state.

**Reasoning:** Using `fold` provides a clean, iterative way to build the `Garden` by processing lines in order. It ensures that the state from the previous line (`LastGardenScanLine`) is correctly passed to `process_line` for the current line.

### Processing Each Line: The `process_line` Function

The `process_line` function is called by `parse_garden` for every line of the input. Its role is to determine how the segments on the *current* line connect to the plots identified on the *previous* line.

```rust
fn process_line(
    input: &str,
    plots: BTreeMap<usize, Plot>, // plots collected so far
    g_line: LastGardenScanLine, // state from the previous line
    mut get_plot_id: impl FnMut() -> usize,
    line: usize // current line number
) -> (BTreeMap<usize, Plot>, LastGardenScanLine) // updated plots and state for the next line
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

The `process_segment` function is the heart of the plot identification and merging logic. It takes a single `segment` from the *current* line and compares it against the segments recorded in the `g_line` (the state from the *previous* line).

```rust
// for each new segment identify the plot that is overlapping with and assign the segment the plot's ID
fn process_segment(
    segment: &PlotSegment, // The segment from the current line being processed
    plots: BTreeMap<usize, Plot>, // The collection of all plots found so far (passed by value)
    g_line: LastGardenScanLine, // The segments and IDs from the previous line (passed by value)
    line: usize, // The current line number
    mut get_plot_id: impl FnMut() -> usize // Function to get a new unique plot ID
) -> (BTreeMap<usize, Plot>, LastGardenScanLine, usize, Option<Vec<usize>>)
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

## 6. Step 5: Calculating Area and Perimeter

Once the `Garden` is fully parsed and all `Plot`s are assembled, we can calculate their properties.

```rust
    pub(super) fn area(self: &Plot) -> usize {
        self.rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
    }
```

**Insight:** The area of a plot is simply the sum of the lengths of all its horizontal segments.

**Reasoning:** By definition, each `PlotSegment` represents a rectangular area of a single plant type on its row. Summing the lengths of all such segments within a `Plot` gives the total count of individual plant cells, which is the area.

Calculating the perimeter is more involved:

```rust
    pub(super) fn perimeter(&self) -> usize {
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

Let's look at `edge_count_north_south`:

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

**Insight:** This function calculates the contribution of horizontal (North/South) edges to the perimeter. It iterates through each row (`y`) containing segments of the plot. For each segment on the current row (`y`), it calculates the non-overlapping length against segments on the row *above* (`y-1`) and the row *below* (`y+1`). The non-overlapping portion represents the horizontal perimeter edge at that location. The calculation `2 * seg.len() - above - below` efficiently sums the horizontal edges (both north and south faces) for that segment. The `fold` implementation cleverly updates the ranges for the `above_row`, `current_row`, and `below_row` iterators in each step, avoiding repeated lookups for the same row data.

**Reasoning:** By summing the non-overlapping vertical lengths for every segment against its neighbors above and below, and adding the fixed horizontal edges (two per segment), we get the total perimeter. This approach correctly handles complex shapes with holes or indentations.

## 7. Step 6: Visualizing the Results

Understanding the output of a spatial algorithm is crucial for debugging and verification. The program includes `Debug` implementations that provide a visual representation.

```rust
impl Debug for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        // ... rendering code ...
    }
}
```

**Insight:** The `Debug` implementation for `Garden` generates a colored grid representation of the garden, where each plot is rendered with a unique color based on its ID and plant type. It also prints the row number and lists the segments present on each row.

**Reasoning:** This visualization allows you to see the identified plots overlaid on the garden grid. You can visually inspect if the plots were correctly identified, if merges happened as expected, and if the shapes match the input. This is invaluable for debugging the complex parsing logic.

There is also a `Debug` implementation for `Plot` in `plot.rs` which lists the segments for a single plot organized by row number, providing a more detailed look at the structure of an individual plot.

```rust
impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ... rendering code ...
    }
}
```

## 8. Step 7: The Main Program Loop

The `main` function ties all these components together to execute the program.

```rust
fn main() {
    let args = std::env::args();
    let input = std::fs::read_to_string(
        match args.skip(1).next() {
            None => "src/bin/day12/input.txt".to_string(),
            Some(str) => str,
        }
    ).unwrap();

    let garden = Garden::parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(id, plot)| println!("ID:{id}\n{plot:?}"))
        .map(|(_,v)|
            (v.area(), v.perimeter())
        )
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    println!("{:?}", &garden);
    println!("Garden total cost : {total}");
    //assert_eq!(total, 1533024)
}
```

**Insight:** The `main` function reads the input file (allowing an optional command-line argument for the path), calls `Garden::parse_garden` to build the data structure, then iterates through the identified `Plot`s in the `Garden`. For each plot, it calculates the area and perimeter, prints these values, calculates the product (area * perimeter), and sums these products for a total. It uses the `Debug` implementation to print each individual plot and the final assembled garden visualization.

**Reasoning:** This function provides the entry point and orchestrates the main workflow: input -> process -> output. The use of iterators (`iter`, `map`, `sum`) and the `inspect` adapter demonstrates a functional style for processing the collection of plots.

## 9. Design Decisions and Trade-offs

Throughout the development of this program, several design choices were made, each with its own trade-offs:

1.  **`BTreeSet` for Plot Rows:** Storing `PlotSegment`s in a `BTreeSet` within the `Plot` struct ensures they are always sorted by row and then by column range.
    *   *Benefit:* This simplifies algorithms that need ordered access (like perimeter calculation) and makes visualization consistent. Iteration is efficient in sorted order.
    *   *Trade-off:* Insertion into a `BTreeSet` is typically `O(log n)`, which is less performant than `O(1)` for `Vec` or `HashMap`. The requirement for `PlotSegment` to implement `Ord` adds a slight complexity to the type definition.

2.  **`BTreeMap` for Garden Plots:** Using a `BTreeMap` to map plot IDs to `Plot` structures.
    *   *Benefit:* Provides efficient `O(log n)` lookup, insertion, and removal of plots by ID, necessary during the merging process.
    *   *Trade-off:* Like `BTreeSet`, it has a performance cost compared to `HashMap` for hashing types, though `usize` hashing is very fast. The keys are kept sorted, which isn't strictly required but adds predictable iteration order.

3.  **Line-by-Line Processing with `LastGardenScanLine`:** The algorithm processes the garden one line at a time, relying on state from the *previous* line (`LastGardenScanLine`) to determine connectivity and perform merges.
    *   *Benefit:* This avoids loading the entire potentially very large garden grid into memory at once, making it memory-efficient for height. The logic focuses on local interactions between adjacent lines.
    *   *Trade-off:* The state management (`LastGardenScanLine`, handling deprecated IDs, passing structs by value in the fold) adds significant complexity to the `process_line` and `process_segment` functions.

4.  **Merging Logic based on Smallest ID:** When a segment connects multiple existing plots, they are merged under the plot ID that is numerically smallest among the connected plots.
    *   *Benefit:* Provides a simple and deterministic rule for merging, ensuring that plots are correctly consolidated regardless of the order segments are processed within a line.
    *   *Trade-off:* Requires keeping track of and updating deprecated IDs throughout the process, adding complexity to the state management.

5.  **Comprehensive Unit Tests:** The presence of `#[cfg(test)] mod tests` blocks (in `segment.rs`, `plot.rs`, and implicitly `garden.rs` via `plot.rs`'s test) demonstrates a commitment to testing key components like segment overlap, range extraction, and the `Garden::parse_garden` function with sample data.
    *   *Benefit:* Ensures the correctness of fundamental building blocks and the complex parsing logic, preventing regressions as the code evolves. This is crucial for spatial algorithms where visual correctness is hard to verify manually for large inputs.
    *   *Trade-off:* Writing comprehensive tests requires significant upfront effort.

## 10. Conclusion

This garden analysis program showcases several important programming principles:

1.  **Abstraction:** Breaking the problem down into logical layers: `PlotSegment` (horizontal piece), `Plot` (multi-line region), and `Garden` (collection of plots) simplifies complexity.
2.  **Data Structure Choice:** Selecting appropriate data structures (`BTreeSet`, `BTreeMap`, `Vec`) based on access patterns and ordering requirements is vital for both correctness and performance.
3.  **Algorithmic Design:** Implementing a line-by-line processing algorithm with state management (`LastGardenScanLine`) effectively handles the 2D spatial problem while managing memory.
4.  **Functional Programming:** Utilizing iterators (`map`, `fold`, `chunk_by`) leads to concise and expressive code for data transformation and processing.
5.  **Test-Driven Development/Testing:** Writing tests for components helps validate complex logic and builds confidence in the overall solution.
6.  **Visualization:** Implementing `Debug` traits for visual output is a powerful debugging tool for spatial or complex data structures.

By combining these principles, the program successfully identifies, measures, and visualizes distinct plant plots in a 2D grid, demonstrating a robust approach to this type of spatial analysis problem.
