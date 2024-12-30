use std::collections::HashSet;
use advent2024::location::*;
use crate::topographical_map::TopographicalMap;

#[derive(Debug, Default)]
pub(crate) struct TrailHead {
    history: Option<HashSet<Location>>
}

impl TrailHead {

    pub(crate) fn unique_trails() -> TrailHead {
        TrailHead { history: None }
    }

    pub(crate) fn trail_heads() -> TrailHead {
        TrailHead { history: Some(HashSet::new()) }
    }

    pub(crate) fn count_trails(
        &mut self,
        map: &TopographicalMap,
        loc: Location,
        is_found: impl Fn(u8) -> bool + Copy
    ) -> Option<usize>
    {
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
                    self.count_trails(map, neighbor, is_found)
                })
                .sum::<usize>()
        )
    }
}
