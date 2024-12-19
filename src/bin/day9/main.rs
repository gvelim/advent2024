mod diskmap;

use std::fmt::Debug;
use std::time::Instant;
use crate::diskmap::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();

    let t = Instant::now();
    let fs = FileSystem::read_diskmap(&diskmap).collect::<Vec<Entry>>();
    let comp = FileSystem::compress(&fs).collect::<Vec<Entry>>();
    let chksum = FileSystem::checksum(&comp);
    println!("Part 1: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6225730762521,chksum);

    let t = Instant::now();
    let dfg_dm = FileSystem::move_files(&mut diskmap);
    let dfg_fs = FileSystem::read_diskmap(dfg_dm).collect::<Vec<Entry>>();
    let chksum = FileSystem::checksum(&dfg_fs);
    println!("Part 2: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6250605700557,chksum);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn read_diskmap(map: &DiskMap) -> impl Iterator<Item=Entry> {
        map.iter()
            .enumerate()
            .flat_map(move |(i, &c)| {
                (0..c.0).map(move |_| (c.0, if i % 2 == 0 {c.1} else {-1}))
            })
    }
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
            .filter(|(_, i)| i.1.is_positive())
            .map(|(i, c)| i * c.1 as usize)
            .sum::<usize>()
    }
    fn move_files(dm: &mut DiskMap) -> &DiskMap {
        let files = dm.files().cloned().collect::<Vec<Entry>>();

        for file in files.iter().rev() {
            let Some(f_pos) = dm.iter().position(|e| e == file) else { continue };
            let Some(s_pos) = dm.spaces().position(|space| space.0 >= file.0) else { continue };
            if s_pos*2+1 > f_pos { continue }
            dm
                .remove_file(f_pos)
                .insert_file(s_pos*2+1, *file);
        }
        dm
    }
}
