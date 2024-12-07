use std::{collections::{HashMap, HashSet}, num::ParseIntError, str::FromStr};

pub type PageSet = HashSet<usize>;
pub type Page = usize;

#[derive(Debug)]
pub struct OrderRules {
    // Key: Page, Value: Set of Pages at lower priority
    rules: HashMap<Page,PageSet>
}

impl OrderRules {
    pub fn pages_to_follow(&self, p: Page) -> Option<&PageSet> {
        self.rules.get(&p)
    }
}

impl FromStr for OrderRules {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for l in s.lines() {
            let mut s = l.split('|');
            let x = s.next().expect("mising X").parse::<usize>()?;
            let y = s.next().expect("missing Y").parse::<usize>()?;
            rules
                .entry(x)
                .and_modify(|s: &mut PageSet| {s.insert(y);})
                .or_insert(HashSet::new())
                .insert(y);
        }
        Ok(OrderRules{rules})
    }
}
