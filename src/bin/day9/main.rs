mod diskmap;

use std::fmt::Debug;
use std::iter::repeat;
use itertools::Itertools;
use crate::diskmap::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/sample.txt").unwrap();
    let mut diskmap = input.lines().next().unwrap().parse::<DiskMap>().unwrap();
    // let diskmap = "2333133121414131402";
    //
    let fs = FileSystem::read_diskmap(&diskmap).collect::<Vec<_>>();
    let comp = FileSystem::compress(&fs).collect::<Vec<_>>();
    let chksum = FileSystem::checksum(&comp);
    println!("Part 1: Checksum {:?}",chksum);
    // assert_eq!(6225730762521,chksum);
    FileSystem::move_files(&mut diskmap);
    // let chksum = FileSystem::checksum(&comp);
    println!("Part 2: Checksum {:?}, {:?}",0, diskmap);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn read_diskmap(map: &DiskMap) -> impl Iterator<Item=(isize,u8)> {
        let mut id_gen = sequence(0);
        map.iter()
            .enumerate()
            .flat_map(move |(i, &c)| {
                repeat(
                    if i % 2 == 0 { (id_gen(1),c.0 as u8) } else { (-1,c.0 as u8) }
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
    fn move_files(dm: &mut DiskMap) {
        let files = dm.files().collect::<Vec<_>>();
        let mut offset = 0;

        for file in files.iter().rev() {
            let f_pos = (file.1 * 2) as usize + offset;
            print!("File to move: {:?}", (file,f_pos));
            // find space
            let space = dm.iter().filter(|e| e.1.eq(&-1)).find_position(|space| space.0 >= file.0);
            if let Some((pos, space)) = space  {
                let s_pos = pos * 2 + 1;
                if space.0 == 0 || s_pos >= f_pos { continue; }
                print!(" into: {:?}", (s_pos, space));
                dm.remove_file(f_pos).insert_file(s_pos, *file);
                print!(", remove at {} & insert at {}", f_pos, s_pos);
                offset += 2;
            }
            println!("\n\t{:?}",dm);
        }
    }
}

fn sequence(mut start: isize) -> impl FnMut(isize) -> isize {
    move |inc| { let ret = start; start += inc; ret }
}