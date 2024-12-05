use std::{fmt::Debug, rc::Rc, str::FromStr};
use super::location::Location;

pub(crate) struct Field<T> where T: Copy + Clone {
    pub cells: Rc<[T]>,
    pub length: usize
}

impl<T> Field<T> where T: Copy + Clone {
    pub(crate) fn get_pos(&self, Location(x, y): Location) -> Option<T>
    {
        let index = y * self.length + x;
        if x <= self.length && index < self.cells.len() {
            Some(self.cells[index])
        } else {
            None
        }
    }
}

impl FromStr for Field<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            cells: s
                .lines()
                .map(|s| s.chars())
                .flatten()
                .collect::<Rc<[char]>>(),
            length: s.lines().next().unwrap().len()
        })
    }
}

impl<T> Debug for Field<T> where T: Copy + Clone + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (p, c) in self.cells.iter().enumerate() {
            if p % self.length == 0 { writeln!(f)? }
            write!(f, "{:?}", c)?;
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

        assert_eq!(field.get_pos(Location(9, 0)), Some('M'));
        assert_eq!(field.get_pos(Location(9, 1)), Some('A'));
        assert_eq!(field.get_pos(Location(0, 8)), Some('M'));
        assert_eq!(field.get_pos(Location(0, 9)), Some('M'));
        assert_eq!(field.get_pos(Location(9, 8)), Some('M'));
        assert_eq!(field.get_pos(Location(9, 9)), Some('X'));
        assert_eq!(field.get_pos(Location(10, 9)), None);
        assert_eq!(field.get_pos(Location(9, 10)), None);
    }
}
