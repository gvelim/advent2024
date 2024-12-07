use std::{fmt::Debug, num::ParseIntError, str::FromStr};
use crate::order::Page;

pub(crate) struct ManualUpdates {
    // store (Key: Number, Val: {Page,Index})
    // we'll then use the rules to validate Xi < Yi
    pub list: Vec<Page>,
}

impl ManualUpdates {
    pub fn entries(&self) -> impl Iterator<Item = &Page>  {
        self.list.iter()
    }
    pub fn contains(&self, p: Page) -> Option<usize> {
        self.list.iter().position(|&e|p == e)
    }
    pub(crate) fn middle(&self) -> Page {
        self.list.get(self.list.len()/2).map_or(0_usize, |entry| *entry)
    }
}

impl FromStr for ManualUpdates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s
            .split(',')
            .map(|numeric|
                numeric.parse::<usize>()
            )
            .collect::<Result<Vec<_>,_>>()?;

        Ok( ManualUpdates {list})
    }
}

impl Debug for ManualUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Updates")?;
        f.debug_list().entries(
            self.entries()
        ).finish()?;
        Ok(())
    }
}
