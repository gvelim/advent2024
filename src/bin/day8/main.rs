use std::{collections::HashMap, ops::RangeInclusive, str::FromStr, time::Instant};
use advent2024::{field::Field, location::Location};
use itertools::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day8/sample.txt").unwrap();
    let city = input.parse::<City>().expect("Failed to parse City");

    let t = Instant::now();
    let count = city.antinodes(1..=1).unique().count();
    println!("Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}",count, t.elapsed());

    let t = Instant::now();
    let count = city.antinodes(0..=10).unique().count();
    println!("Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}",count, t.elapsed());
}

#[derive(Debug,Clone, Copy)]
struct  Antenna(Location);

impl Antenna {
    pub fn antinodes(&self, rhs: Antenna, resonance: usize) -> [Option<Location>;2] {
        let (dxu,dyu) = self.0.distance(&rhs.0);
        let (dx,dy) = ((resonance * dxu) as isize, (resonance * dyu) as isize);
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

impl City {
    pub fn antinodes(&self, resonance:RangeInclusive<usize>) -> impl Iterator<Item = Location> {
        self.antennas
            .iter()
            .flat_map(move |(_, a)|
                a.iter()
                    .tuple_combinations()
                    .flat_map({
                        let res = resonance.clone();
                        move |(a,&b)|
                        res.clone()
                            .take_while(move |&r|{
                                let res = a.antinodes(b,r);
                                match (res[0],res[1]) {
                                    (None, None) => false,
                                    (None, Some(l)) if self.city.get(l).is_some() => true,
                                    (Some(l), None) if self.city.get(l).is_some() => true,
                                    (Some(a), Some(b)) if self.city.get(a).is_some() || self.city.get(b).is_some() => true,
                                    _ => false
                                }
                            })
                            .map(move |r| a.antinodes(b,r))
                    })
            )
            .flat_map(|an| an.into_iter())
            .filter_map(|loc|
                loc.filter(|&loc| self.city.get(loc).is_some())
            )
    }
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
        assert_eq!(a[0].antinodes(a[1],1), [Some(Location(6, 7)), Some(Location(3, 1))]);
        assert_eq!(a[0].antinodes(a[2],1), [Some(Location(12, 5)), Some(Location(0, 2))]);
        assert_eq!(a[1].antinodes(a[2],1), [Some(Location(11, 3)), Some(Location(2, 6))]);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day8/sample.txt").unwrap();
        let city = input.parse::<City>().expect("Failed to parse City");
        println!("{:?}",city.city);
        println!("{:?}",city.antennas);
    }
}
