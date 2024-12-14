use std::{collections::HashMap, isize, str::FromStr};
use advent2024::{field::Field, location::Location};

fn main() {
    todo!();
}

#[derive(Debug,Clone, Copy)]
struct  Antenna(Location);
impl Antenna {
    pub fn antinodes(&self, rhs: Antenna) -> [Option<Location>;2] {
        let (dxu,dyu) = self.0.distance(&rhs.0);
        let (dx,dy) =( dxu as isize, dyu as isize);
        match (self.0.0 >= rhs.0.0, self.0.1 >= rhs.0.1) {
            (true, true) => [rhs.0.move_relative((-dx,-dy)), self.0.move_relative((dx,dy))],
            (true, false) => [rhs.0.move_relative((-dx,dy)), self.0.move_relative((dx,-dy))],
            (false, true) => [rhs.0.move_relative((dx,-dy)), self.0.move_relative((-dx,dy))],
            (false, false) => [rhs.0.move_relative((dx,dy)), self.0.move_relative((-dx,-dy))],
        }
    }
}

struct City {
    city: Field<char>,
    antennas: HashMap<char,Vec<Antenna>>
}

impl FromStr for City {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let city = s.parse::<Field<char>>()?;
        let antennas: HashMap<char,Vec<Antenna>> = city.iter()
            .enumerate()
            .filter(|&(_,c)| c.ne(&'.'))
            .fold(HashMap::new(), |mut map, (i, &c)| {
                let loc = city.index_to_cartesian(i);
                map.entry(c)
                    .and_modify(|antennas| antennas.push(Antenna(loc)))
                    .or_insert(vec![Antenna(loc)]);
                map
            });
        Ok(City { city, antennas })
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
        assert_eq!(a[0].antinodes(a[1]), [Some(Location(6, 7)), Some(Location(3, 1))]);
        assert_eq!(a[0].antinodes(a[2]), [Some(Location(12, 5)), Some(Location(0, 2))]);
        assert_eq!(a[1].antinodes(a[2]), [Some(Location(11, 3)), Some(Location(2, 6))]);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day8/sample.txt").unwrap();
        let city = input.parse::<City>().expect("Failed to parse City");
        println!("{:?}",city.city);
        println!("{:?}",city.antennas);
    }
}
