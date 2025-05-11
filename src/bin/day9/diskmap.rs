use std::{fmt::Debug, str::FromStr};

pub type Id = i16;
pub type Count = u8;
pub type Entry = (Count,Id);

#[derive(Clone)]
pub struct DiskMap(Vec<Entry>);

impl DiskMap {
    pub fn spaces(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1.is_negative())
    }

    pub(crate) fn files(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1 != -1)
    }

    fn insert_file(&mut self, idx: usize, file: Entry) -> &mut Self {
        if idx % 2 == 0 { return self }
        if self.0.get(idx).is_none() { return self };
        if self.0.get(idx).unwrap().0 < file.0 { return self }
        let space = self.0.remove(idx);
        self.0.splice(idx..idx, [(0,-1), file, (space.0.abs_diff(file.0),-1)]);
        self
    }

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

    pub fn expand_diskmap(&self) -> impl Iterator<Item=Entry> {
        self.0.iter()
            .flat_map(move |&(count, id)| {
                (0..count).map(move |_| (count, id))
            })
    }

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

    pub fn checksum(&self) -> usize {
        self.expand_diskmap()
            .enumerate()
            .map(|(idx, (_,id))| if id.is_negative() {0} else {idx * id as usize})
            .sum::<usize>()
    }
}

fn sequence(mut start: isize) -> impl FnMut(isize) -> isize {
    move |inc| { let ret = start; start += inc; ret }
}

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_checksum() {
        let dm = "2333133121414131402".parse::<DiskMap>().unwrap();
        println!("{dm:?}");
        assert_eq!(dm.checksum(), 4116);
    }
    #[test]
    fn test_diskmap_parse() {
        let dm = "2333133121414131402".parse::<DiskMap>().unwrap();
        println!("{dm:?}");
        println!("Space: {:?}", dm.spaces().collect::<Vec<_>>());
        println!("File: {:?}", dm.files().collect::<Vec<_>>());
    }

    #[test]
    fn test_diskmap_compress() {
        let mut dm = "2333133121414131402".parse::<DiskMap>().unwrap();
        println!("{dm:?}");
        println!("Space: {:?}", dm.compress());
        println!("Checksum: {:?}", dm.checksum());
        assert_eq!(1928,dm.checksum());
    }

    #[test]
    fn test_diskmap_move_file() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("\n{dm:?}");
        assert_eq!(dm.move_file(4,1).0, vec![(2, 0), (0,-1), (1, 2), (2, -1), (3, 1), (6, -1), (3, 3)]);
        println!("{dm:?}");
        assert_eq!(dm.move_file(6,3).0, vec![(2, 0), (0,-1), (1, 2), (0,-1),(2, 3),(0,-1),(3, 1), (6, -1), (1, 3)]);
        println!("{dm:?}");
        assert_eq!(dm.move_file(8,7).0, vec![(2, 0), (0,-1), (1, 2), (0,-1),(2, 3),(0,-1),(3, 1), (0,-1),(1,3)]);
        println!("{dm:?}");
    }

    #[test]
    fn test_diskmap_remove_file() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("\n{dm:?}");
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1), (6, -1), (3, 3)]);
        println!("{dm:?}");
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1)]);
        println!("{dm:?}");
        assert_eq!(dm.remove_file(1).0, vec![(2, 0), (3, -1), (3, 1)]);
        println!("{dm:?}");
        assert_eq!(dm.remove_file(0).0, vec![(3, 1)]);
        println!("{dm:?}");
    }

    #[test]
    fn test_diskmap_insert_file() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("\n{dm:?}");
        assert_eq!(dm.insert_file(1, (2, 4)).0, vec![(2, 0), (0, -1), (2, 4), (1, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{dm:?}");
        assert_eq!(dm.insert_file(3, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{dm:?}");
        assert_eq!(dm.insert_file(2, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{dm:?}");
    }
}
