use std::{collections::{HashMap, HashSet}, rc::Rc, str::FromStr};

use advent2024::{field::Field, location::*};

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").unwrap();
    let map = input.parse::<TopographicalMap>().unwrap();

    println!("{:?}", map);
    let sum = map.lowests()
        .filter_map(|start|
            TrailHead::default().search_path(&map, start, |d| d == 9)
        )
        .sum::<usize>();

    println!("{:?}", sum);
}

#[derive(Debug)]
struct TopographicalMap(Field<u8>);

impl TopographicalMap {
    fn lowests(&self) -> impl Iterator<Item = Location> {
        self.0.iter()
            .enumerate()
            .filter(|&(_,s)| *s == 0)
            .map(|(idx,_)| self.0.index_to_cartesian(idx))
    }
}

#[derive(Debug, Default)]
struct TrailHead {
    history: HashSet<Location>
}

impl TrailHead {
    fn search_path(&mut self, map: &TopographicalMap, loc: Location, comp: fn(u8)->bool) -> Option<usize> {
        let &val = map.0.get(loc)?;
        if comp(val) { return Some(1) }

        Some([(0,1),(0,-1),(1,0),(-1,0)]
            .into_iter()
            .filter_map(|dir| loc.move_relative(dir))
            .filter_map(|neighbor| {
                if self.history.contains(&neighbor) { return None };
                if map.0.get(neighbor).is_some_and(|&nv| nv != val+1) { return None }
                self.history.insert(neighbor);
                self.search_path(map, neighbor, comp)
            })
            .sum::<usize>()
        )
    }
}

impl FromStr for TopographicalMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TopographicalMap(
            s.parse::<Field<u8>>()?
        ))
    }
}
