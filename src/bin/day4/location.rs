use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Location(pub usize, pub usize);

impl Location {
    pub fn move_by(&self, dir: (isize,isize)) -> Option<Location> {
        let x = self.0.checked_add_signed(dir.0);
        let y = self.1.checked_add_signed(dir.1);
        match (x, y) {
            (Some(x), Some(y)) => Some(Location(x, y)),
            _ => None
        }
    }
}

impl Sub for Location
{
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 && self.1 >= rhs.1 {
            Some(Location(self.0.sub(rhs.0), self.1.sub(rhs.1)))
        } else {
            None
        }
    }
}

impl Add for Location
{
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        Some(Location(self.0.add(rhs.0), self.1.add(rhs.1) ))
    }
}
