use std::{collections::HashMap, fmt::Debug, num::ParseIntError, str::FromStr};
use crate::order::Page;
use super::entry::PrintEntry;

pub(crate) struct ManualUpdates {
    // store (Key: Number, Val: {Page,Index})
    // we'll then use the rules to validate Xi < Yi
    list: HashMap<Page,PrintEntry>,
    middle: Page
}

impl ManualUpdates {
    pub fn entries(&self) -> impl Iterator<Item = &PrintEntry>  {
        self.list.values()
    }
    pub fn contains(&self, p: Page) -> Option<&PrintEntry> {
        self.list.get(&p)
    }
    pub(crate) fn middle(&self) -> Page {
        self.middle
    }
}

impl FromStr for ManualUpdates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s
            .split(',')
            .enumerate()
            .map(|(pos,numeric)|
                numeric.parse::<usize>().map(|page|
                    (page, PrintEntry{page,pos})
                )
            )
            .collect::<Result<Vec<_>,_>>()?;

        let middle = list.get( list.len()/2 )
            .map_or(0_usize, |entry| entry.0);

        Ok( ManualUpdates {
            list: HashMap::from_iter(list),
            middle
        })
    }
}

impl Debug for ManualUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Updates")?;
        f.debug_map().entries(
            self.entries().map(|d| (d.page, d.pos))
        ).finish()?;
        Ok(())
    }
}
