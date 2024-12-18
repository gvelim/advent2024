use std::str::FromStr;
use crate::sequence;

#[derive(Debug)]
pub struct DiskMap(Vec<u8>);
impl DiskMap {
    pub fn iter(&self) -> impl Iterator<Item = &u8> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.0.iter_mut()
    }
    fn spaces(&self) -> impl Iterator<Item=u8> {
        let mut inc = sequence(0);
        self.0.iter().copied().filter(move |_| inc(1) % 2 != 0)
    }
    fn files(&self) -> impl Iterator<Item=u8> {
        let mut inc = sequence(0);
        self.0.iter().copied().filter(move |_| inc(1) % 2 == 0)
    }
    fn expand(&mut self, idx: usize, value: u8) -> &mut Self {
        if self.0.get(idx).is_none() { return self };
        if self.0.get(idx).unwrap() < &value { return self }
        let space = self.0.remove(idx);
        [0,value,space.abs_diff(value)].into_iter().enumerate().for_each(|(i, v)| {
            self.0.insert(idx+i, v);
        });
        self
    }
    fn collapse(&mut self, idx: usize) -> &mut Self {
        match (
            idx.checked_sub(1).map(|idx| self.0.get(idx)),
            self.0.get(idx),
            self.0.get(idx + 1)
        ) {
            (Some(Some(a)), Some(b), Some(c)) => Some((a + b + c, idx - 1..=idx + 1)),
            (Some(Some(a)), Some(b), None) => Some((a + b, idx - 1..=idx)),
            (None, Some(b), Some(c)) => Some((b + c, idx..=idx + 1)),
            _ => None,
        }.inspect(|(sum, rng)| {
            self.0.drain(rng.clone());
            self.0.insert(*rng.start(), *sum);
        });
        self
    }
}

impl FromStr for DiskMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.bytes().map(|c| c - b'0').collect()
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
        println!("Space: {:?}", dm.spaces().collect::<Vec<u8>>());
        println!("File: {:?}", dm.files().collect::<Vec<u8>>());
    }

    #[test]
    fn test_diskmap_collapse() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        assert_eq!(dm.collapse(5).0, vec![2, 3, 3, 3, 6]);
        assert_eq!(dm.collapse(4).0, vec![2, 3, 3, 9]);
        assert_eq!(dm.collapse(0).0, vec![5, 3, 9]);
        println!("{:?}", dm);
    }

    #[test]
    fn test_diskmap_expand() {
        let mut dm = "2333123".parse::<DiskMap>().unwrap();
        assert_eq!(dm.expand(1, 2).0, vec![2, 0, 2, 1, 3, 3, 1, 2, 3]);
        assert_eq!(dm.expand(3, 1).0, vec![2, 0, 2, 0, 1, 0, 3, 3, 1, 2, 3]);
        assert_eq!(dm.expand(1, 1).0, vec![2,0,2,0,1,0,3,3,1,2,3]);
        println!("{:?}", dm );
    }
}