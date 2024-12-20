use std::{fmt::Debug, str::FromStr};

pub type Id = i16;
pub type Count = u8;
pub type Entry = (Count,Id);

pub struct DiskMap(Vec<Entry>);

impl DiskMap {
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.0.iter()
    }

    pub fn spaces(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1 == -1)
    }

    pub(crate) fn files(&self) -> impl Iterator<Item=&Entry> {
        self.0.iter().filter(|e| e.1 != -1)
    }

    pub(crate) fn insert_file(&mut self, idx: usize, value: Entry) -> &mut Self {
        if idx % 2 == 0 { return self }
        if self.0.get(idx).is_none() { return self };
        if self.0.get(idx).unwrap().0 < value.0 { return self }
        let space = self.0.remove(idx);
        self.0.splice(idx..idx, [(0,-1), value, (space.0.abs_diff(value.0),-1)]);
        self
    }

    pub(crate) fn remove_file(&mut self, idx: usize) -> &mut Self {
        if idx % 2 != 0 { return self }
        match (
            idx.checked_sub(1).map(|idx| self.0.get(idx)),
            self.0.get(idx),
            self.0.get(idx + 1)
        ) {
            (Some(Some(a)), Some(b), Some(c)) => Some((a.0 + b.0 + c.0, idx - 1..=idx + 1)),
            (Some(Some(_)), Some(_), None) => Some((Count::MAX, idx-1..=idx)),
            (None, Some(_), Some(_)) => Some((Count::MAX, idx..=idx+1)),
            _ => None,
        }.inspect(|(sum, rng)| {
            self.0.drain(rng.clone());
            if sum < &Count::MAX {
                self.0.insert(*rng.start(), (*sum,-1));
            }
        });
        self
    }

    pub fn expand_diskmap(&self) -> impl Iterator<Item=Entry> {
        self.iter()
            .flat_map(move |&(count, id)| {
                (0..count).map(move |_| (count, id))
            })
    }

    pub fn move_files(&mut self) -> &DiskMap {
        let files = self.files().cloned().collect::<Vec<Entry>>();

        for file in files.iter().rev() {
            let Some(f_pos) = self.iter().position(|e| e == file) else { continue };
            let Some(s_pos) = self.spaces().position(|space| space.0 >= file.0) else { continue };
            if s_pos*2+1 > f_pos { continue }
            self
                .remove_file(f_pos)
                .insert_file(s_pos*2+1, *file);
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
        println!("{:?}",dm);
        assert_eq!(dm.checksum(), 4116);
    }
    #[test]
    fn test_diskmap_parse() {
        let dm = "2333133121414131402".parse::<DiskMap>().unwrap();
        println!("{:?}", dm);
        println!("Space: {:?}", dm.spaces().collect::<Vec<_>>());
        println!("File: {:?}", dm.files().collect::<Vec<_>>());
    }

    #[test]
    fn test_diskmap_collapse() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("{:?}",dm);
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1), (6, -1), (3, 3)]);
        println!("{:?}", dm );
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1)]);
        println!("{:?}", dm );
        assert_eq!(dm.remove_file(1).0, vec![(2, 0), (3, -1), (3, 1)]);
        println!("{:?}", dm );
        assert_eq!(dm.remove_file(0).0, vec![(3, 1)]);
        println!("{:?}", dm);
    }

    #[test]
    fn test_diskmap_expand() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("{:?}", dm );
        assert_eq!(dm.insert_file(1, (2, 4)).0, vec![(2, 0), (0, -1), (2, 4), (1, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{:?}", dm );
        assert_eq!(dm.insert_file(3, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{:?}", dm );
        assert_eq!(dm.insert_file(2, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{:?}", dm );
    }
}
