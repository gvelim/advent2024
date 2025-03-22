# Garden Analysis Program: A Step-by-Step Technical Walkthrough

This document explains the design and implementation of a garden analysis program that processes a text-based garden layout, identifies distinct plant regions, and calculates their areas and perimeters.

## 1. Solution Intuition

The core challenge is identifying contiguous regions of the same plant type from a 2D text representation. The solution treats each same-character region as a distinct "plot" and analyzes its properties.

At a high level, the program:
1. Parses the garden layout line by line
2. Identifies segments of plants on each line
3. Tracks contiguous regions across multiple lines
4. Calculates area and perimeter of each region
5. Outputs visualization and analysis results

## 2. Data Model: Fundamental Structure

The program's foundation is built on three main abstractions:

### PlotSegment: The Basic Building Block

```rust
pub(super) struct PlotSegment(char, Range<Seed>);
```

Insight: A `PlotSegment` represents a horizontal row of identical plants, storing:
- The plant type (character)
- A range of positions this plant spans in a single line

This simple abstraction allows us to:
- Check if a position is contained within the segment
- Determine if segments overlap (essential for region tracking)
- Calculate the segment's length

```rust
impl PlotSegment {
    pub(super) fn is_overlapping(&self, other: &Self) -> bool {
        self.start() < other.end() && self.end() > other.start()
    }

    pub(super) fn len(&self) -> Seed {
        self.1.end as Seed - self.1.start as Seed
    }
}
```

## 3. Line-by-Line Parsing

The first step is to convert text lines into collections of segments:

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

Insight: This function processes text line-by-line, identifying runs of the same character. For example, "RRRRIICCFF" becomes:
- ('R', 0..4)
- ('I', 4..6)
- ('C', 6..8)
- ('F', 8..10)

This establishes the horizontal layout of plant segments on each line.

## 4. Plot: Tracking Multi-Line Regions

The next level is organizing segments into coherent plots:

```rust
pub(super) struct Plot {
    rows: BTreeSet<(usize, PlotSegment)>
}
```

Insight: A plot is a collection of segments organized by row number. Using a `BTreeSet` keeps segments ordered, making traversal and calculation more efficient.

## 5. Garden Assembly: Connecting Segments Across Lines

The most complex part of the solution is determining which segments connect to form cohesive plots:

```rust
pub(super) fn parse_garden(input: &str) -> Garden {
    let mut get_new_plot_id = id_generator(0);
    let mut get_line_number = id_generator(0);

    let (mut plots, mut g_line) = input
        .lines()
        .fold((BTreeMap::<usize, Plot>::new(), LastGardenScanLine::default()),
              |(plots, g_line), input| {
                  process_line(input, plots, g_line, &mut get_new_plot_id, get_line_number())
        });

    // move plot segments remaining to the garden map under their respective plot ID
    push_segments(&mut plots, g_line.drain(), get_line_number());

    // return garden map
    Garden { plots }
}
```

Insight: This algorithm:
1. Processes each line sequentially, maintaining state from previous lines
2. Tracks which segments in the current line overlap with previous segments
3. Assigns IDs to track distinct plots across the entire garden
4. Merges plots when segments of the same plant type overlap vertically

The key insight is in `process_segment`:

```rust
fn process_segment(segment: &PlotSegment, plots: BTreeMap<usize, Plot>, g_line: LastGardenScanLine,
                  line: usize, mut get_plot_id: impl FnMut() -> usize)
    -> (BTreeMap<usize, Plot>, LastGardenScanLine, usize)
{
    // find active plots matching this segment
    // matching = (a) overlapping with && (b) have same plant type
    let matched = g_line.overlaps(segment);

    // if empty, then return a new plot ID for the segment
    if matched.is_empty() {
        return (plots, g_line, get_plot_id());
    }

    // otherwise, use the first matching plot ID as the master ID for consolidating all matched plots
    let (_, master_id, _) = g_line[ matched[0] ];

    // Process matches and handle merges
    // ...
}
```

This approach solves the tricky problem of region identification by:
1. Creating new plot IDs for unmatched segments
2. Reusing existing IDs when segments overlap with known plots
3. Merging plots when a segment bridges multiple existing plots

## 6. Area and Perimeter Calculation

With plots properly identified, we can calculate properties:

```rust
pub(super) fn area(self: &Plot) -> usize {
    self.rows.iter().map(|seg| seg.1.len() as usize).sum::<usize>()
}

pub(super) fn perimeter(&self) -> usize {
    let y_range = self.get_plot_y_range();

    self.north_perimeter_counter(y_range.clone()) +
        self.north_perimeter_counter(y_range.clone().rev()) +
        self.rows.len() * 2
}
```

Insight: Area is straightforward - the sum of all segment lengths. Perimeter is more complex:
1. Count north-facing perimeter edges by comparing each segment with the line above
2. Count south-facing perimeter edges using the same algorithm in reverse
3. Add the left and right edges (equal to the total number of segments × 2)

The most intricate part is calculating the north/south perimeter:

```rust
fn north_perimeter_counter(&self, range: impl Iterator<Item = usize>) -> usize  {
    let (west_bound, east_bound) = self.get_plot_bounding_segs();

    range.map(|y| {
        self.rows
            // for each segment in line `y`
            .range( (y, west_bound.clone()) ..= (y, east_bound.clone()) )
            .map(|(_, seg)| {
                // calculate perimeter on segment's north side
                // Sum( segment overlapping area against segment(s) above)
                seg.len() as usize - self.rows
                    .range( (y-1, west_bound.clone()) ..= (y-1, east_bound.clone()) )
                    .filter(|(_,nseg)| nseg.is_overlapping(seg) )
                    .map(|(_,nseg)| nseg.get_overlap(seg) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    })
    .sum::<usize>()
}
```

This approach:
1. Examines each segment on line y
2. Subtracts the total overlap with segments on the line above
3. The remaining value represents the non-overlapping perimeter

## 7. Visualization for Debugging

The solution includes visualization to help verify correctness:

```rust
impl Debug for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        // Rendering code...
    }
}
```

Insight: Debugging complex spatial algorithms is much easier with visual output. This makes patterns and errors immediately apparent.

## 8. Main Program Flow

The main program ties everything together:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").unwrap();
    let garden = Garden::parse_garden(&input);

    let total = garden
        .iter()
        .inspect(|(_, plot)| println!("{:?}", plot))
        .map(|(_,v)| (v.area(), v.perimeter()))
        .map(|(a,b)| {
            println!("area: {} * perimeter: {} = {}\n", a, b, a * b);
            a * b
        })
        .sum::<usize>();

    println!("{:?}", &garden);
    println!("Garden total cost : {total}");
}
```

This processes the input, calculates metrics for each plot, and outputs the total "cost" (area × perimeter).

## 9. Design Decisions and Trade-offs

1. **BTreeSet vs. Vector**: Using `BTreeSet` for plot storage enables efficient ordered access but adds complexity. The trade-off favors computational efficiency for larger gardens.

2. **Functional Style**: The code frequently uses iterators and functional patterns. This improves code clarity for operations like filtering and mapping but may introduce performance overhead compared to imperative approaches.

3. **Two-Pass Algorithm**: The solution processes lines first, then calculates properties. This separation of concerns improves maintainability but requires more memory to store intermediate state.

4. **Testing**: The comprehensive test suite validates both small components (segment overlap) and complete garden examples. This design choice slows initial development but prevents regressions and ensures correctness.

## 10. Conclusion

This solution demonstrates several important programming principles:

1. **Abstraction Hierarchy**: Breaking the problem into PlotSegment → Plot → Garden creates clean separation of concerns

2. **Incremental Complexity**: Starting with simple segments and building up to complete garden analysis makes the complex algorithm manageable

3. **Functional Programming**: Using iterators and transformations results in concise, expressive code

4. **Visualization Tools**: Creating visual debugging output significantly aids understanding and troubleshooting

The garden analysis program successfully identifies distinct plant plots in complex layouts and accurately calculates their properties, demonstrating a robust approach to 2D spatial analysis.
