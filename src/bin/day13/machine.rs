use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, str::FromStr};
use advent2024::location::{reverse_dirvector, DirVector, Location};


pub(crate) struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
}

impl ClawMachine {
    pub(crate) fn new(buttons: &[Button]) -> Self {
        ClawMachine { buttons: buttons.into(), cache: RefCell::new(HashMap::new()) }
    }

    pub(crate) fn optimal_cost(&self, prize: Location) -> Option<u32> {
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

impl Debug for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClawMachine")
            .field("buttons", &self.buttons)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub(crate) struct Button {
    dir: DirVector,
    cost: u32
}

impl Button {
    pub(crate) fn new(dir: DirVector, cost: u32) -> Self {
        Button { dir, cost }
    }
}

impl FromStr for Button {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use crate::parser::parse_button;

        match parse_button(input) {
            Ok((_, button)) => Ok(button),
            Err(_) => Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse_prize_clawmachine;

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
