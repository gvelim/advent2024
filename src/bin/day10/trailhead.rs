use std::collections::HashSet;
use advent2024::location::*;
use crate::topographical_map::TopographicalMap;

#[derive(Debug, Default)]
pub(crate) struct TrailHead {
    history: Option<HashSet<Location>>
}

impl TrailHead {

    pub(crate) fn new(history: bool) -> TrailHead {
        TrailHead { history: if history { Some(HashSet::new()) } else { None } }
    }

    pub(crate) fn search_path(&mut self, map: &TopographicalMap, loc: Location, is_found: fn(u8)->bool) -> Option<usize> {
        let &val = map.get(loc)?;
        if is_found(val) { return Some(1) }
        Some(
            [(0,1),(0,-1),(1,0),(-1,0)]
                .into_iter()
                .filter_map(|dir| loc.move_relative(dir))
                .filter_map(|neighbor| {
                    if self.history.as_ref().is_some_and(|h| h.contains(&neighbor)) { return None };
                    if map.get(neighbor).is_some_and(|&nv| nv != val+1) { return None }

                    self.history.as_mut().map(|h| h.insert(neighbor));
                    self.search_path(map, neighbor, is_found)
                })
                .sum::<usize>()
        )
    }
}
