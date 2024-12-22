use std::{fmt::{Debug, Display}, str::FromStr};
use crate::location::DirVector;

use super::location::Location;

pub struct Field<T> {
    cells: Vec<Vec<T>>
}

impl<T> Field<T> {
    pub fn get(&self, Location(x, y): Location) -> Option<&T>
    {
        // use build-in bounds checker of the two arrays
        // out of bounds will result to None
        self.cells
            .get(y)
            .and_then(|w| w.get(x))
    }
    pub fn get_mut(&mut self, Location(x,y): Location) -> Option<&mut T> {
        self.cells.get_mut(y)
            .and_then(|w| w.get_mut(x))
    }
    pub fn peek(&self, l: Location, d: DirVector) -> Option<&T> {
        l.move_relative(d).and_then(|l| self.get(l))
    }
    pub fn peek_mut(&mut self, l: Location, d: DirVector) -> Option<&mut T> {
        l.move_relative(d).and_then(|l| self.get_mut(l))
    }
    pub fn within_bounds(&self, l: Location) -> bool {
        l.0 < self.width() && l.1 < self.height()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cells
            .iter()
            .flat_map(|c| c.iter())
    }
    pub fn width(&self) -> usize { self.cells.first().map(|v| v.len()).unwrap_or(0) }
    pub fn height(&self) -> usize { self.cells.len() }
    pub fn index_to_cartesian(&self, idx: usize) -> Location {
        Location(idx % self.width(), idx / self.width() )
    }
    pub fn new(w: usize, h: usize, d: T) -> Field<T> where T: Clone {
        Field {
            cells: vec![vec![d;w];h]
        }
    }
}

impl FromStr for Field<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            cells: s
                .lines()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>(),
        })
    }
}

impl FromStr for Field<u8> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            cells: s
                .lines()
                .map(|s| s.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
                .collect::<Vec<_>>(),
        })
    }
}

impl<T> Debug for Field<T> where T: Debug + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"W:{},H{}",self.width(),self.height())?;
        for c in self.cells.iter() {
            writeln!(f)?;
            for c in c.iter() { write!(f,"{:2}", c)? };
        }
        writeln!(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pos() {
        let input = std::fs::read_to_string("src/bin/day4/sample.txt").expect("File not found");
        let field = input.parse::<Field<char>>().expect("Doesn't error");

        assert_eq!(field.get(Location(9, 0)), Some(&'M'));
        assert_eq!(field.get(Location(9, 1)), Some(&'A'));
        assert_eq!(field.get(Location(0, 8)), Some(&'M'));
        assert_eq!(field.get(Location(0, 9)), Some(&'M'));
        assert_eq!(field.get(Location(9, 8)), Some(&'M'));
        assert_eq!(field.get(Location(9, 9)), Some(&'X'));
        assert_eq!(field.get(Location(10, 9)), None);
        assert_eq!(field.get(Location(9, 10)), None);
    }
}
