
pub type DirVector = (isize,isize);

pub fn turn_cw(dir: DirVector) -> Option<DirVector> {
    match dir {
        (1,0) => Some((0,1)),
        (0,1) => Some((-1,0)),
        (-1,0) => Some((0,-1)),
        (0,-1) => Some((1,0)),
        _ => None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(pub usize, pub usize);

impl Location {
    // get a new location given current location + delta vector
    pub fn move_relative(&self, distance: DirVector) -> Option<Location> {
        let x = self.0.checked_add_signed(distance.0);
        let y = self.1.checked_add_signed(distance.1);
        match (x, y) {
            (Some(x), Some(y)) => Some(Location(x, y)),
            _ => None
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
