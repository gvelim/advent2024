use std::iter::repeat;

fn main() {
    let diskmap = "2333133121414131402";
    let fs = FileSystem::read_diskmap(diskmap).collect::<String>();

    println!("00...111...2...333.44.5555.6666.777.888899\n{:?}",fs);

    let mut citer = fs.chars().rev().filter(|c| c != &'.');
    let comp = FileSystem::read_diskmap(diskmap)
        .filter_map(|c|
            if c == '.' { citer.next() } else { Some(c) }
        )
        .collect::<String>();

    println!("{:?}",comp);
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
