use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::order::{OrderRules, Page};

pub type Position = usize;

#[derive(Debug, PartialEq)]
pub(crate) struct Update {
    // store (Key: Number, Val: Index)
    // we'll then use the rules to validate Xi < Yi
    list: HashMap<Page,Position>
}

impl Update {
    pub fn is_page_order_valid(&self, rules: &OrderRules) -> bool {
        self.list
            .iter()
            // is each page followed by the correct pages ?
            .all(|(&page, &pos)|{
                rules
                    .pages_to_follow(page)
                    .inspect(|p| print!("{:?} => {:?}, ",(page,pos),p))
                    .map(|pages| {
                        // pages that MUST follow in the update
                        pages
                            .iter()
                            // current page position < following page(s) positions
                            .all(|&follow_page|
                                // page in the update list ?
                                self.contains(follow_page)
                                    .map(|f_pos| f_pos > pos)
                                    .unwrap_or(true)
                            )
                    })
                    .inspect(|_| println!())
                    .unwrap_or(true)
            })
    }
    fn contains(&self, p: Page) -> Option<Position> {
        self.list.get(&p).cloned()
    }
}

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Update {
            list: s
                .split(',')
                .enumerate()
                .map(|(i,numeric)|
                    numeric.parse::<usize>().map(|num| (num,i))
                )
                .collect::<Result<_,_>>()?
        })
    }
}

#[test]
fn test_parse_update() {
    assert_eq!(
        "75,47,61,53,29".parse::<Update>().unwrap(),
        Update { list: HashMap::from([(75,0),(47,1),(61,2),(53,3),(29,4)]) }
    );
}
