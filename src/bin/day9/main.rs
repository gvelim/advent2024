mod diskmap;

use std::fmt::Debug;
use std::iter::repeat;
use std::time::Instant;
use itertools::Itertools;
use crate::diskmap::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();

    let t = Instant::now();
    let fs = FileSystem::read_diskmap(&diskmap).collect::<Vec<_>>();
    let comp = FileSystem::compress(&fs).collect::<Vec<_>>();
    let chksum = FileSystem::checksum(&comp);
    println!("Part 1: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6225730762521,chksum);

    let t = Instant::now();
    let dfg_dm = FileSystem::move_files(&mut diskmap);
    let dfg_fs = FileSystem::read_diskmap(dfg_dm).collect::<Vec<_>>();
    let chksum = FileSystem::checksum(&dfg_fs);
    println!("Part 2: Checksum {:?} - {:?}",chksum, t.elapsed());
    assert_eq!(6250605700557,chksum);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn read_diskmap(map: &DiskMap) -> impl Iterator<Item=(isize,u8)> {
        map.iter()
            .enumerate()
            .flat_map(move |(i, &c)| {
                repeat(
                    if i % 2 == 0 { (c.1,c.0 as u8) } else { (-1,c.0 as u8) }
                ).take(c.0 as usize)
            })
    }
    fn compress(fs: &[(isize,u8)]) -> impl Iterator<Item=(isize,u8)> {
        let mut citer = fs.iter().rev().enumerate().filter(|(_, c)| c.0.is_positive()).peekable();
        fs.iter()
            .enumerate()
            .filter_map(move |(i, &c)| {
                let &(ci, &cc) = citer.peek()?;
                if i >= fs.len() - ci { return None };
                if c.0.is_negative() { citer.next(); Some(cc) } else { Some(c) }
            })
    }
    fn checksum(comp: &[(isize,u8)]) -> usize {
        comp.iter()
            .enumerate()
            .filter(|(_, i)| i.0.is_positive())
            .map(|(i, c)| i * (c.0 as usize))
            .sum::<usize>()
    }
    fn move_files(dm: &mut DiskMap) -> &DiskMap {
        let files = dm.files().cloned().collect::<Vec<_>>();

        for file in files.iter().rev() {
            let Some(f_pos) = dm.iter().position(|e| e == file) else { continue };
            let space = dm.spaces().find_position(|space| space.0 >= file.0);
            if let Some((pos, space)) = space  {
                let s_pos = pos * 2 + 1;
                if space.0 == 0 || s_pos >= f_pos { continue; }
                dm.remove_file(f_pos).insert_file(s_pos, *file);
            }
        }
        dm
    }
    // fn to_string(fs: &[(isize, u8)]) -> String {
    //     fs.iter()
    //         .map(|&(i, _)|{
    //             if i == -1 {'.'} else { ((i % 10) as u8 + b'0') as char }
    //         })
    //         .collect::<String>()
    // }
}
