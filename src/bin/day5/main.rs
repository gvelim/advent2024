mod update;
mod order;

use order::OrderRules;
use update::Update;

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/sample.txt").expect("msg");
    let mut s = input.split("\n\n");
    let rules_str = s.next().unwrap();
    let lists_str = s.next().unwrap();

    let rules = rules_str.parse::<OrderRules>().expect("msg");
    let printer = printer(&rules);

    let pass = lists_str.lines()
        .map(|line| line.parse::<Update>().expect("msg"))
        .take(1)
        .inspect(|d| println!("{:?}",d))
        .all(printer);
    println!("{:?}", if pass {"Pass"} else {"Fail"});
}


fn printer(order: &OrderRules) ->  impl Fn(Update) -> bool {
    |update: Update| update.is_page_order_valid(order)
}
