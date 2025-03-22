# Disk Fragmentation Simulation: An Educational Deep Dive

This documentation provides an educational overview of the disk fragmentation simulation program, explaining its design principles, implementation details, and the reasoning behind important decisions. The program models disk space allocation, file operations, and optimization techniques for managing disk space.

## Table of Contents

1. [Solution Intuition](#solution-intuition)
2. [Core Data Structures](#core-data-structures)
3. [Disk Map Representation](#disk-map-representation)
4. [File Operations](#file-operations)
5. [Space Management](#space-management)
6. [Optimization Algorithms](#optimization-algorithms)
7. [Visualization and Debugging](#visualization-and-debugging)
8. [Testing Strategy](#testing-strategy)
9. [Performance Considerations](#performance-considerations)

## Solution Intuition

The problem requires modeling a disk storage system with files and free space, then implementing operations like compressing and defragmenting the disk to optimize file storage. The key insight is representing the disk as alternating segments of files and free space, where each segment has a size and an identifier. This approach makes it easier to model operations like moving files and merging adjacent free spaces.

The solution uses a run-length encoding approach where we don't store every single block but rather store segments (runs) of the same type - either files or free space - along with their sizes. This provides a compact representation that's efficient to manipulate.

## Core Data Structures

The program defines three fundamental types:

```rust
pub type Id = i16;        // Identifies files with positive values, spaces with negative
pub type Count = u8;      // Size of a segment
pub type Entry = (Count,Id); // Pairs size with identifier
```

**Insight**: The choice of `i16` for `Id` creates a clear distinction between files (positive IDs) and spaces (negative IDs, typically -1). This makes filtering and processing segments by type straightforward.

**Reason**: Using signed integers for IDs simplifies type checking with a single comparison rather than maintaining a separate flag for file vs. space.

```rust
#[derive(Clone)]
pub struct DiskMap(Vec<Entry>);
```

**Insight**: Encapsulating the vector of entries in a struct allows for adding specialized methods that maintain the invariants of the disk map.

**Reason**: This design follows the principle of data abstraction, ensuring that the internal representation can change without affecting client code.

## Disk Map Representation

The `DiskMap` structure represents a disk as a sequence of file and space segments. Each segment has a size and an ID.

```rust
impl DiskMap {
    pub fn spaces(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1.is_negative())
    }

    pub(crate) fn files(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1 != -1)
    }
}
```

**Insight**: Using iterators with filters provides a clean, functional way to work with specific segment types.

**Reason**: This approach reduces code duplication and follows Rust's iterator pattern, which is both memory efficient and expressive.

### Parsing and Creating DiskMaps

```rust
impl FromStr for DiskMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seq = sequence(0);
        Ok(Self(s
            .bytes()
            .enumerate()
            .map(|(idx,num)|
                ((num - b'0') as Count, if idx % 2 == 0 { seq(1) } else { -1 } as Id)
            )
            .collect()
        ))
    }
}
```

**Insight**: The input format alternates between sizes and identifiers. The `sequence` helper function generates incrementing IDs for files.

**Reason**: This parsing approach converts a string like "2333" into a sequence of (size, id) pairs, where even positions define file sizes and odd positions define space sizes.

```rust
fn sequence(mut start: isize) -> impl FnMut(isize) -> isize {
    move |inc| { let ret = start; start += inc; ret }
}
```

**Insight**: This closure-based counter generator creates a stateful function that returns incrementing values.

**Reason**: This is a functional approach to maintaining state, providing a clean way to generate sequential IDs without global variables.

## File Operations

### Inserting Files

```rust
fn insert_file(&mut self, idx: usize, file: Entry) -> &mut Self {
    if idx % 2 == 0 { return self }
    if self.0.get(idx).is_none() { return self };
    if self.0.get(idx).unwrap().0 < file.0 { return self }
    let space = self.0.remove(idx);
    self.0.splice(idx..idx, [(0,-1), file, (space.0.abs_diff(file.0),-1)]);
    self
}
```

**Insight**: When inserting a file, we first validate that the target location is a space segment with sufficient size. Then we split that space into three parts: a zero-sized space marker, the new file, and the remaining free space.

**Reason**: This design preserves the alternating structure of files and spaces. The zero-sized space maintains the pattern even when a file is inserted at the beginning of a space segment.

### Moving Files

```rust
fn move_file(&mut self, src: usize, dst: usize) -> &mut Self {
    if src % 2 != 0 || dst % 2 == 0 { return self }
    if self.0.get(src).is_none() || self.0.get(dst).is_none() { return self }

    let file = self.0[src];
    if self.0[dst].0 >= self.0[src].0 {
        self.insert_file(dst, file)
            .remove_file(src+2);
    } else {
        self.0[src].0 -= self.0[dst].0;
        self.insert_file(dst, (self.0[dst].0, file.1 ));
    }
    self
}
```

**Insight**: Moving a file involves checking if the destination space has enough room. If it does, we insert the whole file and remove it from the source. If not, we split the file, moving as much as will fit.

**Reason**: This approach handles both complete and partial file moves, ensuring efficient use of available space while maintaining the disk structure.

### Removing Files

```rust
fn remove_file(&mut self, idx: usize) -> &mut Self {
    if idx % 2 != 0 { return self }
    match (
        idx.checked_sub(1).and_then(|idx| self.0.get(idx)),
        self.0.get(idx),
        self.0.get(idx + 1)
    ) {
        (Some(a), Some(b), Some(c)) => Some((a.0+b.0+c.0, idx-1..=idx+1)),
        (Some(_), Some(_), None) => Some((Count::MAX, idx-1..=idx)),
        (None, Some(_), Some(_)) => Some((Count::MAX, idx..=idx+1)),
        _ => None,
    }
    .map(|(sum, rng)| {
        self.0.drain(rng);
        if sum < Count::MAX {
            self.0.insert(idx-1, (sum,-1));
        }
        Some(())
    });
    self
}
```

**Insight**: When removing a file, we need to merge the adjacent space segments. This pattern matching approach handles various edge cases like files at the beginning or end of the disk.

**Reason**: Merging adjacent spaces is crucial for defragmentation. The careful handling of edge cases ensures the disk structure remains valid regardless of which file is removed.

## Space Management

### Expanding DiskMap

```rust
pub fn expand_diskmap(&self) -> impl Iterator<Item=Entry> {
    self.0.iter()
        .flat_map(move |&(count, id)| {
            (0..count).map(move |_| (count, id))
        })
}
```

**Insight**: This method expands the run-length encoding into individual units, useful for visualization and checksum calculation.

**Reason**: While the compact representation is efficient for manipulation, some operations need to work with the expanded view. This iterator-based approach avoids allocating a full copy of the expanded data.

### Checksum Calculation

```rust
pub fn checksum(&self) -> usize {
    self.expand_diskmap()
        .enumerate()
        .map(|(idx, (_,id))| if id.is_negative() {0} else {idx * id as usize})
        .sum::<usize>()
}
```

**Insight**: The checksum multiplies each file's ID by its position in the expanded view and sums these values, ignoring space segments.

**Reason**: This provides a way to validate the disk state after operations. The position-weighting ensures the checksum changes if files move, even if the overall content remains the same.

## Optimization Algorithms

### Compressing the Disk

```rust
pub fn compress(&mut self) -> &DiskMap {
    let mut s_pos = 1;
    while s_pos < self.0.len() - 1 {
        if self.0[s_pos].0 > 0 {
            self.move_file(self.0.len() - 1, s_pos);
        }
        s_pos += 2;
    }
    self
}
```

**Insight**: Compression moves files from the end of the disk to the first available space that can fit them, creating contiguous free space at the end.

**Reason**: This approach simulates how an operating system might consolidate files at the beginning of a disk to maximize contiguous free space, which makes allocating large files easier.

### Defragmenting the Disk

```rust
pub fn defragment(&mut self) -> &DiskMap {
    let files = self.files().cloned().collect::<std::rc::Rc<[Entry]>>();
    let len = self.0.len() - 1 ;

    for file in files.iter().rev() {
        let Some(f_pos) = self.0.iter().rev().position(|e| e == file) else { continue };
        let Some(s_pos) = self.spaces().position(|space| space.0 >= file.0) else { continue };
        if s_pos*2+1 >= len - f_pos { continue }
        self.move_file(len-f_pos, s_pos*2+1);
    }
    self
}
```

**Insight**: Defragmentation iterates through files from right to left, moving each file to the first available space that can fit it, as long as that move would bring the file closer to the beginning of the disk.

**Reason**: This optimizes not just for contiguous free space but also for file locality, placing files as close to the beginning as possible while maintaining their order relative to each other.

## Visualization and Debugging

```rust
impl Debug for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self
            .expand_diskmap()
            .map(|(_,i)| if i == -1 {'.'} else { ((i % 10) as u8 + b'0') as char })
        {
            write!(f,"{c}")?
        };
        Ok(())
    }
}
```

**Insight**: The Debug implementation provides a visual representation of the disk, showing spaces as dots and files as their ID (modulo 10).

**Reason**: Visual representation is crucial for understanding disk layout, especially during development and debugging. This approach makes it easy to validate the effects of operations.

## Testing Strategy

The code includes comprehensive unit tests that verify each operation independently and in combination:

```rust
#[test]
fn test_diskmap_move_file() {
    let mut dm = "2333123".parse::<DiskMap>().unwrap();
    println!("\n{:?}",dm);
    assert_eq!(dm.move_file(4,1).0, vec![(2, 0), (0,-1), (1, 2), (2, -1), (3, 1), (6, -1), (3, 3)]);
    // Additional test steps...
}
```

**Insight**: Tests use sample input strings to create disk maps, then verify that operations produce the expected results.

**Reason**: This approach tests not just individual operations but also their integration, ensuring that the system works as a whole.

## Performance Considerations

The main program demonstrates how to measure performance:

```rust
let t = Instant::now();
let chksum = diskmap.clone().compress().checksum();
println!("Part 1: Checksum {:?} - {:?}",chksum, t.elapsed());
```

**Insight**: By measuring operation time, we can identify potential bottlenecks and optimize accordingly.

**Reason**: Performance matters, especially for operations that might be applied to large disks. The timing information helps balance algorithm complexity against execution time.

---

This documentation has walked through the key components of the disk fragmentation simulation, explaining the design principles, implementation details, and reasoning behind important decisions. The program demonstrates effective use of Rust's type system, iterators, and pattern matching to create a clean, maintainable simulation of disk operations.
