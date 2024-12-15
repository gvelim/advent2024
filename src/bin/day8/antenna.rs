use std::ops::RangeInclusive;

use advent2024::location::Location;

#[derive(Debug,Clone, Copy)]
pub(crate) struct Antenna(pub Location);

impl Antenna {
    pub fn antinode_pair(&self, rhs: Antenna, harmonics: usize) -> [Option<Location>;2] {
        let (dxu,dyu) = self.0.distance(&rhs.0);
        let (dx,dy) = ((harmonics * dxu) as isize, (harmonics * dyu) as isize);
        match (self.0.0 >= rhs.0.0, self.0.1 >= rhs.0.1) {
            (true, true) => [rhs.0.move_relative((-dx,-dy)), self.0.move_relative((dx,dy))],
            (true, false) => [rhs.0.move_relative((-dx,dy)), self.0.move_relative((dx,-dy))],
            (false, true) => [rhs.0.move_relative((dx,-dy)), self.0.move_relative((-dx,dy))],
            (false, false) => [rhs.0.move_relative((dx,dy)), self.0.move_relative((-dx,-dy))],
        }
    }

    pub fn antinodes(&self, rhs: Antenna, harmonics: RangeInclusive<usize>) -> impl Iterator<Item =[Option<Location>;2]> {
        harmonics
            .map(move |harmonics| self.antinode_pair(rhs, harmonics))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_antinode() {
        let a = [
            Antenna(Location(4,3)),
            Antenna(Location(5,5)),
            Antenna(Location(8,4))
        ];
        assert_eq!(a[0].antinode_pair(a[1],1), [Some(Location(6, 7)), Some(Location(3, 1))]);
        assert_eq!(a[0].antinode_pair(a[2],1), [Some(Location(12, 5)), Some(Location(0, 2))]);
        assert_eq!(a[1].antinode_pair(a[2],1), [Some(Location(11, 3)), Some(Location(2, 6))]);
    }
}
