use std::{fmt::Debug, num::ParseIntError, str::FromStr};
use super::order::Page;
use super::OrderRules;

pub(crate) struct ManualUpdates {
    list: Vec<Page>,
}

impl ManualUpdates {
    pub fn make_validator(rules: &OrderRules) ->  impl Fn(&ManualUpdates) -> bool {
        |updates: &ManualUpdates| {
            let tmp = Self::sort_update(rules)(updates);
            tmp.entries()
                .zip(updates.entries())
                .all(|(a,b)| a == b)
        }
    }

    pub fn sort_update(rules: &OrderRules) ->  impl Fn(&ManualUpdates) -> ManualUpdates {
        use std::cmp;

        |updates: &ManualUpdates| {
            let mut list = updates.entries().cloned().collect::<Vec<_>>();
            list.sort_by(|&a,b| {
                rules
                    .pages_to_follow(a)
                    .map(|set|
                        if set.contains(b) { cmp::Ordering::Less } else { cmp::Ordering::Greater }
                    )
                    .unwrap_or(cmp::Ordering::Equal)
            });
            ManualUpdates { list }
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &Page>  {
        self.list.iter()
    }

    pub(crate) fn middle(&self) -> Page {
        self.list.get(self.list.len()/2).map_or(0_usize, |entry| *entry)
    }
}

impl FromStr for ManualUpdates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( ManualUpdates {
            list : s.split(',')
                .map(|numeric| numeric.parse::<usize>())
                .collect::<Result<Vec<_>,_>>()?
        })
    }
}

impl Debug for ManualUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Updates:")?;
        f.debug_list().entries(self.entries()).finish()?;
        Ok(())
    }
}
