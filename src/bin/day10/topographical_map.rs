use advent2024::{field::Field, location::Location};
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct TopographicalMap(Field<u8>);

impl TopographicalMap {
    #[inline]
    pub(crate) fn get(&self, loc: Location) -> Option<&u8> {
        self.0.get(loc)
    }

    pub(crate) fn lowests(&self) -> impl Iterator<Item = Location> {
        self.0
            .iter()
            .enumerate()
            .filter(|&(_, s)| *s == 0)
            .map(|(idx, _)| self.0.index_to_cartesian(idx))
    }
}

impl FromStr for TopographicalMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TopographicalMap(s.parse::<Field<u8>>()?))
    }
}
