use std::str::FromStr;
use crate::sequence;

pub type Id = isize;
pub type Count = usize;
pub type Entry = (Count,Id);

enum EntryPosition { Start, Middle, End }

#[derive(Debug)]
pub struct DiskMap(Vec<Entry>);
impl DiskMap {
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entry> {
        self.0.iter_mut()
    }
    fn spaces(&self) -> impl Iterator<Item=Entry> {
        let mut inc = sequence(0);
        self.0.iter().copied().filter(move |_| inc(1) % 2 != 0)
    }
    fn files(&self) -> impl Iterator<Item=Entry> {
        let mut inc = sequence(0);
        self.0.iter().copied().filter(move |_| inc(1) % 2 == 0)
    }
    fn insert_file(&mut self, idx: usize, value: Entry) -> &mut Self {
        if idx % 2 == 0 { return self }
        if self.0.get(idx).is_none() { return self };
        if self.0.get(idx).unwrap().0 < value.0 { return self }
        let space = self.0.remove(idx);
        [(0,-1), value, (space.0.abs_diff(value.0),-1)]
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| { self.0.insert(idx+i, v); });
        self
    }
    fn remove_file(&mut self, idx: usize) -> &mut Self {
        if idx % 2 != 0 { return self }
        match (
            idx.checked_sub(1).map(|idx| self.0.get(idx)),
            self.0.get(idx),
            self.0.get(idx + 1)
        ) {
            (Some(Some(a)), Some(b), Some(c)) => Some((a.0 + b.0 + c.0, idx - 1..=idx + 1, EntryPosition::Middle)),
            (Some(Some(_)), Some(_), None) => Some((0, idx - 1..=idx, EntryPosition::End)),
            (None, Some(_), Some(_)) => Some((0, idx..=idx + 1, EntryPosition::Start)),
            _ => None,
        }.inspect(|(sum, rng, pos)| {
            self.0.drain(rng.clone());
            if let EntryPosition::Middle = pos  {
                self.0.insert(*rng.start(), (*sum,-1));
            }
        });
        self
    }
}

impl FromStr for DiskMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seq = sequence(0);
        Ok(Self(s
            .bytes()
            .enumerate()
            .map(|(idx,num)|
                ((num - b'0') as usize, if idx % 2 == 0 { seq(1) } else { -1 })
            )
            .collect()
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_diskmap_parse() {
        let dm = "2333133121414131402".parse::<DiskMap>().unwrap();
        println!("{:?}", dm);
        println!("Space: {:?}", dm.spaces().collect::<Vec<Entry>>());
        println!("File: {:?}", dm.files().collect::<Vec<Entry>>());
    }

    #[test]
    fn test_diskmap_collapse() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        println!("{:?}",dm);
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1), (6, -1), (3, 3)]);
        assert_eq!(dm.remove_file(4).0, vec![(2, 0), (3, -1), (3, 1)]);
        assert_eq!(dm.remove_file(1).0, vec![(2, 0), (3, -1), (3, 1)]);
        assert_eq!(dm.remove_file(0).0, vec![(3, 1)]);
        println!("{:?}", dm);
    }

    #[test]
    fn test_diskmap_expand() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        assert_eq!(dm.insert_file(1, (2, 4)).0, vec![(2, 0), (0, -1), (2, 4), (1, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        assert_eq!(dm.insert_file(3, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        assert_eq!(dm.insert_file(2, (1, 5)).0, vec![(2, 0), (0, -1), (2, 4), (0, -1), (1, 5), (0, -1), (3, 1), (3, -1), (1, 2), (2, -1), (3, 3)]);
        println!("{:?}", dm );
    }
}