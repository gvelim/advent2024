
pub type Direction = (isize,isize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Location(pub usize, pub usize);

impl Location {
    // get a new location given current location + delta vector
    pub fn move_relative(&self, distance: Direction) -> Option<Location> {
        let x = self.0.checked_add_signed(distance.0);
        let y = self.1.checked_add_signed(distance.1);
        match (x, y) {
            (Some(x), Some(y)) => Some(Location(x, y)),
            _ => None
        }
    }
}
