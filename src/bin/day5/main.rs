mod update;
mod order;
mod entry;

use order::OrderRules;
use update::ManualUpdates;

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/input.txt").expect("msg");
    let mut s = input.split("\n\n");
    let rules_str = s.next().unwrap();
    let updates = s.next().unwrap()
        .lines()
        .map(|line| line.parse::<ManualUpdates>().expect("msg"));

    let rules = rules_str.parse::<OrderRules>().expect("msg");
    let validator = make_validator(&rules);

    let pass = updates
        // .inspect(|d| print!("{:?}",d))
        .filter(validator)
        .map(|updates| updates.middle())
        // .inspect(|s| println!("{s}"))
        .sum::<usize>();
    println!("Part 1: valid updates score: {pass}");
}


fn make_validator(rules: &OrderRules) ->  impl Fn(&ManualUpdates) -> bool {
    |updates: &ManualUpdates| {
        updates
            .entries()
            // is each page followed by the correct pages ?
            .all(|entry|{
                rules
                    .pages_to_follow(entry.page)
                    // .inspect(|p| print!("{:?} => {:?}, ",entry,p))
                    .map(|pages| {
                        // pages that MUST follow in the update
                        pages
                            .iter()
                            // current page position < following page(s) positions
                            .all(|&follow_page|
                                // page in the update list ?
                                updates.contains(follow_page)
                                    // .inspect(|p| print!("{:?},",p))
                                    .map(|following| following > entry)
                                    .unwrap_or(true)
                            )
                    })
                    // .inspect(|_| println!())
                    .unwrap_or(true)
            })
    }
}
