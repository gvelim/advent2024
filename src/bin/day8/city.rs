use crate::antenna::Antenna;
use advent2024::{field::Field, location::Location};
use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

pub(crate) struct City {
    city: Field<char>,
    antennas: HashMap<char, Vec<Antenna>>,
}

impl City {
    pub fn antinodes(&self, harmonics: RangeInclusive<usize>) -> impl Iterator<Item = Location> {
        self.antennas
            .values()
            .flat_map(move |antennas| {
                antennas.iter().tuple_combinations().flat_map({
                    let h = harmonics.clone();
                    move |(a, b)| {
                        a.antinodes(*b, h.clone()).take_while(|&antinodes| {
                            match (antinodes[0], antinodes[1]) {
                                (_, Some(l)) if self.city.get(l).is_some() => true,
                                (Some(l), _) if self.city.get(l).is_some() => true,
                                _ => false,
                            }
                        })
                    }
                })
            })
            .flat_map(|antinodes| antinodes.into_iter())
            .filter_map(|location| location.filter(|&location| self.city.get(location).is_some()))
    }
}

impl FromStr for City {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let city = s.parse::<Field<char>>()?;
        let antennas: HashMap<char, Vec<Antenna>> = city
            .iter()
            .enumerate()
            .filter(|&(_, c)| c.ne(&'.'))
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
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day8/sample.txt").unwrap();
        let city = input.parse::<City>().expect("Failed to parse City");
        println!("{:?}", city.city);
        println!("{:?}", city.antennas);
    }
}
