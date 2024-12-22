use std::{collections::HashSet, str::FromStr};
use advent2024::{field::Field, location::*};

#[derive(Debug)]
pub(crate) struct TopographicalMap(Field<u8>);

impl TopographicalMap {
    pub(crate) fn lowests(&self) -> impl Iterator<Item = Location> {
        self.0.iter()
            .enumerate()
            .filter(|&(_,s)| *s == 0)
            .map(|(idx,_)| self.0.index_to_cartesian(idx))
    }
}

#[derive(Debug, Default)]
pub(crate) struct TrailHead {
    history: Option<HashSet<Location>>
}

impl TrailHead {

    pub(crate) fn new(history: bool) -> TrailHead {
        TrailHead { history: if history { Some(HashSet::new()) } else { None } }
    }

    pub(crate) fn search_path(&mut self, map: &TopographicalMap, loc: Location, comp: fn(u8)->bool) -> Option<usize> {
        let &val = map.0.get(loc)?;
        if comp(val) { return Some(1) }

        Some([(0,1),(0,-1),(1,0),(-1,0)]
            .into_iter()
            .filter_map(|dir| loc.move_relative(dir))
            .filter_map(|neighbor| {
                if self.history.as_ref().is_some_and(|h| h.contains(&neighbor)) { return None };
                if map.0.get(neighbor).is_some_and(|&nv| nv != val+1) { return None }
                self.history.as_mut().map(|h| h.insert(neighbor));
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
