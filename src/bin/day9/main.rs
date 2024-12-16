use std::iter::repeat;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/input.txt").unwrap();
    let diskmap = input.lines().next().unwrap();
    // let diskmap = "2333133121414131402";
    //
    let fs = FileSystem::read_diskmap(diskmap).collect::<Vec<_>>();
    let comp = FileSystem::compress(&fs).collect::<Vec<_>>();
    let chksum = FileSystem::checksum(&comp);
    println!("Part 1: Checksum {:?}",chksum);
    assert_eq!(6225730762521,chksum);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn read_diskmap(map: &str) -> impl Iterator<Item = isize> {
        let mut inc = Incr(0);
        map.char_indices()
            .flat_map(move |(i, c)| {
                repeat(
                    if i % 2 == 0 { inc.next().unwrap() } else { -1 }
                ).take((c as u8 - b'0') as usize)
            })
    }
    fn compress(fs: &[isize]) -> impl Iterator<Item = isize> {
        let mut citer = fs.iter().rev().enumerate().filter(|(_, c)| c.is_positive()).peekable();
        fs.iter()
            .enumerate()
            .filter_map(move |(i, &c)|{
                let &(ci, &cc) = citer.peek()?;
                if i >= fs.len()-ci { return None };
                if c < 0isize { citer.next(); Some(cc) } else { Some(c) }
            })
    }
    fn checksum(comp: &[isize]) -> usize {
        comp.iter()
            .enumerate()
            .filter(|(_,i)| i.is_positive())
            .map(|(i, c)| i * (*c as usize))
            .sum::<usize>()
    }
}

struct Incr(isize);
impl Iterator for Incr {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let r = Some(self.0);
        self.0 += 1;
        r
    }
}
