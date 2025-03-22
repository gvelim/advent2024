mod parser;

use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};
use advent2024::location::{reverse_dirvector, DirVector, Location};
use parser::parse_prize_clawmachine;


fn main() {
    let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Failed to read input file");

    let runs = input.split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res|
            res.map(|(_,res)| res)
        )
        .collect::<Result<Vec<_>, _>>();

    runs.iter().for_each(|run| println!("{:?}",run));
}

#[derive(Debug)]
struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
}

impl ClawMachine {
    fn new(buttons: &[Button]) -> Self {
        ClawMachine { buttons: buttons.into(), cache: RefCell::new(HashMap::new()) }
    }

    fn optimal_cost(&self, prize: Location) -> Option<u32> {
        if let Some(val) = self.cache.borrow().get(&prize) {
            return *val;
        }
        if prize.is_origin() {
            return Some(0)
        }

        self.buttons
            .iter()
            .filter_map(|button| {
                let cost = prize
                    .move_relative( reverse_dirvector(button.dir) )
                    .and_then(|new_prize|
                        self.optimal_cost(new_prize).map(|c| c + button.cost)
                    );
                self.cache.borrow_mut().insert(prize,cost);
                cost
            })
            .min()
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Button {
    dir: DirVector,
    cost: u32
}

impl FromStr for Button {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use parser::parse_button;

        match parse_button(input) {
            Ok((_, button)) => Ok(button),
            Err(_) => Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::parse_prize_clawmachine;

    #[test]
    fn test_optimal_cost() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Failed to read file");
        let mut input = input.split("\n\n");
        let run = input.next().unwrap();

        let (_,(prize,clawmachine)) = parse_prize_clawmachine(run).unwrap();

        assert_eq!(
            clawmachine.optimal_cost(prize),
            Some(280)
        );
    }
}
