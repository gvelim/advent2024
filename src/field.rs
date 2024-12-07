use std::{fmt::Debug, rc::Rc, str::FromStr};
use super::location::Location;

pub struct Field<T> {
    cells: Rc<[Rc<[T]>]>
}

impl<T> Field<T> {
    pub fn value_at(&self, Location(x, y): Location) -> Option<&T>
    {
        // use build-in bounds checker of the two arrays
        // out of bounds will result to None
        self.cells
            .get(y)
            .and_then(|w| w.get(x))
    }
    pub fn width(&self) -> usize { self.cells.first().map(|v| v.len()).unwrap_or(0) }
    pub fn height(&self) -> usize { self.cells.len() }
}

impl FromStr for Field<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            cells: s
                .lines()
                .map(|s| s.chars().collect::<Rc<[char]>>())
                .collect::<Rc<[_]>>(),
        })
    }
}

impl<T> Debug for Field<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (p, c) in self.cells.iter().enumerate() {
            if p % self.width() == 0 { writeln!(f)? }
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

        assert_eq!(field.value_at(Location(9, 0)), Some(&'M'));
        assert_eq!(field.value_at(Location(9, 1)), Some(&'A'));
        assert_eq!(field.value_at(Location(0, 8)), Some(&'M'));
        assert_eq!(field.value_at(Location(0, 9)), Some(&'M'));
        assert_eq!(field.value_at(Location(9, 8)), Some(&'M'));
        assert_eq!(field.value_at(Location(9, 9)), Some(&'X'));
        assert_eq!(field.value_at(Location(10, 9)), None);
        assert_eq!(field.value_at(Location(9, 10)), None);
    }
}
