use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, str::FromStr};
use advent2024::location::{reverse_dirvector, DirVector, Location};

type PushCombinations = Vec<(u32,u32)>;

pub(crate) struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
    trail: RefCell<HashMap<u32, u32>>,
    paths: RefCell<Vec<PushCombinations>>,
}

impl ClawMachine {
    pub(crate) fn new(buttons: &[Button]) -> Self {
        ClawMachine {
            buttons: buttons.into(),
            cache: RefCell::new(HashMap::new()),
            trail: RefCell::new(HashMap::default()),
            paths: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn optimal_cost(&self, prize: Location) -> Option<(u32, Vec<PushCombinations> )> {
        self._optimal_cost(prize)
            .map(|c| {
                let paths = self.paths.borrow().clone();
                (c, paths)
            })
    }

    fn _optimal_cost(&self, prize: Location) -> Option<u32> {
        if let Some(val) = self.cache.borrow().get(&prize) {
            return *val;
        }
        if prize.is_origin() {
            self.paths.borrow_mut().push(
                self.trail.borrow()
                    .iter()
                    .map(|(x,y)| (*x,*y))
                    .collect::<Vec<_>>()
            );
            return Some(0)
        }

        self.buttons
            .iter()
            .filter_map(|button| {
                let cost = prize
                    .move_relative( reverse_dirvector(button.dir) )
                    .and_then(|new_prize| {
                        self.trail.borrow_mut().entry(button.cost).and_modify(|c| *c += 1).or_insert(1);
                        let cost = self._optimal_cost(new_prize).map(|c| c + button.cost);
                        self.trail.borrow_mut().entry(button.cost).and_modify(|c| *c -= 1);
                        cost
                    });
                self.cache.borrow_mut().insert(prize,cost);
                cost
            })
            .min()
    }
}

impl Debug for ClawMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClawMachine:")?;
        f.debug_list()
            .entries( self.buttons.iter() )
            .finish()?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub(crate) struct Button {
    dir: DirVector,
    cost: u32
}

impl Button {
    pub(crate) fn new(dir: DirVector, cost: u32) -> Self {
        Button { dir, cost }
    }
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button {}: X+{},Y+{}", self.cost, self.dir.0, self.dir.1)
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
            clawmachine.optimal_cost(prize).unwrap().0,
            280
        );
    }
}
