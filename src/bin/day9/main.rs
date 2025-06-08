mod diskmap;

use crate::diskmap::*;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();

    let t = Instant::now();
    let chksum = diskmap.clone().compress().checksum();
    println!("Part 1: Checksum {:?} - {:?}", chksum, t.elapsed());
    assert_eq!(6225730762521, chksum);

    let t = Instant::now();
    let chksum = diskmap.defragment().checksum();
    println!("Part 2: Checksum {:?} - {:?}", chksum, t.elapsed());
    assert_eq!(6250605700557, chksum);
}
