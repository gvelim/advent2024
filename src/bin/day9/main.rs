mod diskmap;

use std::fmt::Debug;
use std::time::Instant;
use crate::diskmap::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();

    let t = Instant::now();
    let fs = diskmap.expand_diskmap().collect::<Vec<Entry>>();
    let comp = FileSystem::compress(&fs).collect::<Vec<Entry>>();
    let chksum = FileSystem::checksum(&comp);
    println!("Part 1: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6225730762521,chksum);

    let t = Instant::now();
    let chksum = diskmap.move_files().checksum();
    println!("Part 2: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6250605700557,chksum);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn compress(fs: &[Entry]) -> impl Iterator<Item=Entry> {
        let mut citer = fs.iter()
            .rev()
            .enumerate()
            .filter(|(_, c)| c.1.is_positive())
            .peekable();

        fs.iter()
            .enumerate()
            .filter_map(move |(i, &c)| {
                let &(ci, &cc) = citer.peek()?;
                if i >= fs.len() - ci { return None };
                if c.1.is_negative() { citer.next(); Some(cc) } else { Some(c) }
            })
    }

    fn checksum(comp: &[Entry]) -> usize {
        comp.iter()
            .enumerate()
            .map(|(idx, &(_,id))| if id.is_negative() {0} else {idx * id as usize})
            .sum::<usize>()
    }
}
