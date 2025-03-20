use std::ops::Sub;

pub type DirVector = (isize,isize);

pub fn turn_cw(d: DirVector) -> DirVector {
    Direction::from(d).turn_cw().to_cartesian()
}

pub fn turn_ccw(d: DirVector) -> DirVector {
    Direction::from(d).turn_ccw().to_cartesian()
}

pub fn dirvector_to_char(d:DirVector)-> char {
    Direction::from(d).into()
}

pub fn reverse_dirvector(d: DirVector) -> DirVector {
    (-d.0,-d.1)
}

pub enum Direction {
  Left, Down, Right, Up
}

impl Direction {
    pub fn to_cartesian(&self) -> DirVector {
        match self {
            Direction::Left => (-1,0),
            Direction::Down => (0,1),
            Direction::Right => (1,0),
            Direction::Up => (0,-1),
        }
    }
    pub fn turn_cw(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
    pub fn turn_ccw(&self) -> Direction {
        match self {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
        }
    }
}

impl From<Direction> for char{
    fn from(val: Direction) -> Self {
        match val.to_cartesian() { (1,0) => '→', (-1,0) => '←', (0,-1) => '↑', (0,1) => '↓', _ => unreachable!() }
    }
}

impl From<DirVector> for Direction {
    fn from(value: (isize,isize)) -> Self {
        match value {
            (-1,0) => Direction::Left ,
             (0,1) => Direction::Down,
             (1,0) => Direction::Right,
             (0,-1) => Direction::Up ,
             _ => panic!("{value:?} is not any of (-1,0) (0,-1) (1,0) (0,1)")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location(pub usize, pub usize);

impl Location {
    // get a new location given current location + delta vector
    pub fn move_relative(&self, distance: DirVector) -> Option<Location> {
        let x = self.0.checked_add_signed(distance.0);
        let y = self.1.checked_add_signed(distance.1);
        x.zip(y).map(|(x,y)| Location(x,y))
    }

    pub fn next(&self, distance: Direction) -> Option<Location> {
        self.move_relative(distance.to_cartesian())
    }

    pub fn distance(&self, loc: &Location) -> (usize,usize) {
        (self.0.abs_diff(loc.0), self.1.abs_diff(loc.1))
    }

    pub fn is_origin(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

impl Sub for Location {
    type Output = Option<Location>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.0.checked_sub(rhs.0), self.1.checked_sub(rhs.1)) {
            (Some(x), Some(y)) => Some(Location(x,y)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_location() {
        assert_eq!(Location(1,1).move_relative((0,0)), Some(Location(1,1)));
        assert_eq!(Location(1,1).move_relative((1,1)), Some(Location(2,2)));
        assert_eq!(Location(1,1).move_relative((-1,0)), Some(Location(0,1)));
        assert_eq!(Location(1,1).move_relative((0,-1)), Some(Location(1,0)));
        assert_eq!(Location(1,1).move_relative((0,-2)), None);
        assert_eq!(Location(1,1).move_relative((-2,0)), None);
        assert_eq!(Location(1,1).move_relative((0,isize::MAX)), Some(Location(1,9223372036854775808_usize)));
        assert_eq!(Location(1,1).move_relative((isize::MAX,0)), Some(Location(9223372036854775808_usize,1)));
    }
}
