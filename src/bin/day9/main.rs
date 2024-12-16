use std::iter::repeat;

fn main() {
    let input = std::fs::read_to_string("src/bin/day9/sample.txt").unwrap();
    let diskmap = input.lines().next().unwrap();
    // let diskmap = "2333133121414131402".to_string();
    let fs = FileSystem::read_diskmap(diskmap).collect::<String>();

    println!("00...111...2...333.44.5555.6666.777.888899\n{:?}",fs);

    let comp = FileSystem::compress(&fs).collect::<String>();
    let chksum = FileSystem::checksum(&comp);

    println!("{:?} = {:?}",comp, chksum);
}

#[derive(Debug)]
struct FileSystem;

impl FileSystem {
    fn read_diskmap(map: &str) -> impl Iterator<Item = char> {
        let mut inc = Incr(0);
        map.char_indices()
            .flat_map(move |(i, c)| {
                repeat(
                    if i % 2 == 0 { (inc.next().unwrap() + 48) as char } else {'.'}
                ).take((c as u8 - 48) as usize)
            })
    }
    fn compress(fs: &str) -> impl Iterator<Item = char> {
        let mut citer = fs.chars().rev().enumerate().filter(|(_, c)| c != &'.').peekable();
        fs.char_indices()
            .filter_map(move |(i, c)|{
                let &(ci, cc) = citer.peek()?;
                if !(i < fs.len()-ci) { return None };
                if c == '.' { citer.next(); Some(cc) } else { Some(c) }
            })
    }
    fn checksum(comp: &str) -> usize {
        comp.char_indices()
            .map(|(i, c)| i * (c as u8 - 48) as usize)
            .sum::<usize>()
    }
}

struct Incr(u8);
impl Iterator for Incr {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let r = Some(self.0);
        self.0 += 1;
        r
    }
}
