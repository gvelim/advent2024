use std::{collections::HashMap, ops::RangeInclusive, str::FromStr, time::Instant};
use advent2024::{field::Field, location::Location};
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("src/bin/day8/input.txt").unwrap();
    let city = input.parse::<City>().expect("Failed to parse City");

    let t = Instant::now();
    let count = city.antinodes(1..=1).unique().count();
    println!("Part 1: {:?} unique locations within the bounds of the map contain an antinode - {:?}",count, t.elapsed());
    assert_eq!(247,count);

    let t = Instant::now();
    let count = city.antinodes(0..=100).unique().count();
    println!("Part 2: {:?} unique locations contain an antinode given the effects of resonant harmonics - {:?}",count, t.elapsed());
    assert_eq!(861,count);
}

#[derive(Debug,Clone, Copy)]
struct  Antenna(Location);

impl Antenna {
    pub fn antinodes(&self, rhs: Antenna, harmonics: usize) -> [Option<Location>;2] {
        let (dxu,dyu) = self.0.distance(&rhs.0);
        let (dx,dy) = ((harmonics * dxu) as isize, (harmonics * dyu) as isize);
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
    pub fn antinodes(&self, harmonics:RangeInclusive<usize>) -> impl Iterator<Item = Location> {
        self.antennas
            .values()
            .flat_map(move |antennas| antennas
                .iter()
                .tuple_combinations()
                .flat_map({
                    let res = harmonics.clone();
                    move |(a, b)| res
                        .clone()
                        .map(|harmonic| a.antinodes(*b, harmonic))
                        .take_while(|&antinodes| {
                            match (antinodes[0], antinodes[1]) {
                                (_, Some(l)) if self.city.get(l).is_some() => true,
                                (Some(l), _) if self.city.get(l).is_some() => true,
                                _ => false
                            }
                        })
                })
            )
            .flat_map(|antinodes| antinodes.into_iter())
            .filter_map(|location|
                location.filter(|&location| self.city.get(location).is_some())
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
