# Advent of Code 2024 - Day 9: Disk Map

## Problem Overview

The challenge for Day 9 involves managing a disk map, which is a representation of files and spaces on a disk. The disk map is represented as a sequence of entries, where each entry consists of a count and an ID. The count represents the size of the file or space, and the ID represents the file identifier or a special value for spaces.

The goal is to implement various operations on this disk map, such as inserting, moving, and removing files, as well as compressing and defragmenting the disk. Additionally, we need to compute a checksum for the disk map.

## Intuition

The disk map is represented as
* a vector of `Entries`
* each entry is a `tuple`,
* where each tuple contains a `count` and an `ID`.
* The ID is `negative` for spaces and `non-negative` for files.

The operations on the disk map involve
* manipulating this vector to achieve the desired state.
* while retaining the sequence structure File,Space,File,Space, ....

The key operations are:
1. **Insert File**: Insert a file at a specific index.
2. **Move File**: Move a file from one index to another.
3. **Remove File**: Remove a file from the disk map.
4. **Compress**: Move all files to the beginning of the disk map, leaving spaces at the end.
5. **Defragment**: Move files to fill the largest available spaces.

## Solution Steps

### Step 1: Define the Data Structures

We start by defining the data structures for the disk map. The `DiskMap` struct contains a vector of entries, where each entry is a tuple of count and ID.

```rust
pub type Id = i16;
pub type Count = u8;
pub type Entry = (Count, Id);

#[derive(Clone)]
pub struct DiskMap(Vec<Entry>);
```

### Step 2: Implement Basic Operations

#### Insert File

The `insert_file` method inserts a file at a specific index. It checks if the index is valid and if the space at the index is large enough to accommodate the file. If so, it splits the space and inserts the file.

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

#### Move File

The `move_file` method moves a file from one index to another. It checks if the source and destination indices are valid and if the destination space is large enough to accommodate the file.

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

#### Remove File

The `remove_file` method removes a file from the disk map. It merges the adjacent spaces if necessary.

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

### Step 3: Implement Compression and Defragmentation

#### Compress

The `compress` method moves all files to the beginning of the disk map, leaving spaces at the end.

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

#### Defragment

The `defragment` method moves files to fill the largest available spaces.

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

### Step 4: Compute Checksum

The `checksum` method computes the checksum of the disk map by iterating over the expanded disk map and summing the product of the index and the file ID.

```rust
pub fn checksum(&self) -> usize {
    self.expand_diskmap()
        .enumerate()
        .map(|(idx, (_,id))| if id.is_negative() {0} else {idx * id as usize})
        .sum::<usize>()
}
```

### Step 5: Main Function

The main function reads the input, parses the disk map, and performs the compression and defragmentation operations. It then computes and prints the checksums.

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();

    let t = Instant::now();
    let chksum = diskmap.clone().compress().checksum();
    println!("Part 1: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6225730762521,chksum);

    let t = Instant::now();
    let chksum = diskmap.defragment().checksum();
    println!("Part 2: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6250605700557,chksum);
}
```
