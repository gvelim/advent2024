use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, str::FromStr};
use advent2024::location::{reverse_dirvector, DirVector, Location};
use nom::error::Error;

type ButtonCombinations = Vec<(u32,u32)>;

pub(crate) struct ClawMachine {
    buttons: Rc<[Button]>,
    cache: RefCell<HashMap<Location, Option<u32>>>,
    click_trail: RefCell<HashMap<u32, u32>>,
    combos: RefCell<Vec<ButtonCombinations>>,
}

impl ClawMachine {
    pub(crate) fn new(buttons: &[Button]) -> Self {
        ClawMachine {
            buttons: buttons.into(),
            cache: RefCell::new(HashMap::new()),
            click_trail: RefCell::new(HashMap::default()),
            combos: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn _calculate_cost(&self, prize: Location) -> Option<(u32, Vec<ButtonCombinations>)>{
        let [a, b] = self.buttons[..] else { panic!("Ops! argument with more than 2 buttons ?")};
        let (a_x,a_y) = a.dir;
        let (b_x,b_y) = b.dir;
        let (p_x, p_y) = (prize.0 as isize, prize.1 as isize);

        // 2 equations with 2 unknowns
        // Ax*An + Bx*Bn = Px
        // Ay*An + By*Bn = Py
        // where An and Bn are the unknown number of button clicks to reach the goal
        let b_count = (a_x*p_y - a_y*p_x)/(a_x*b_y - a_y*b_x);
        let a_count = (p_x - b_count*b_x)/a_x;

        // since the equation isn't polynomial there is only a single solution
        // hence the cost will be optimal always
        (a_count*a_x + b_count*b_x == p_x && a_count*a_y + b_count*b_y == p_y)
            .then_some({
                let (a_count, b_count) = (a_count as u32, b_count as u32);
                let cost = a_count* a.cost + b_count*b.cost;
                (cost, vec![vec![(a_count,b_count)]])
            })
    }

    // return the optimal cost and the button press combinations
    pub(crate) fn optimal_cost(&self, prize: Location) -> Option<(u32, Vec<ButtonCombinations>)> {
        self._optimal_cost(prize)
            .map(|c| {
                let paths = self.combos.borrow().clone();
                (c, paths)
            })
    }

    // calculates the optimal cost and updates the internal paths
    fn _optimal_cost(&self, prize: Location) -> Option<u32> {
        if let Some(val) = self.cache.borrow().get(&prize) {
            return *val;
        }
        // have we hit the (0,0) prize ?
        if prize.is_origin() {
            // store the button press combinations up to this point
            self.combos
                .borrow_mut()
                .push(
                    // extract from active trail the (button cost, counter) tuples
                    self.click_trail
                        .borrow()
                        .iter()
                        .map(|(x,y)| (*x,*y))
                        .collect::<Vec<_>>()
                );
            // return cost 0 as the initial condition for prize (0,0)
            return Some(0)
        }

        // Calculate the minimum cost by exploring all button options first
        let min_cost = self.buttons
            .iter()
            .filter_map(|button| {
                // calculate origin of current prize given the button press
                // prize - button press = origin_prize
                prize
                    .move_relative( reverse_dirvector(button.dir) )
                    .and_then(|origin_prize| {
                        // increment button press count by one; use button cost as key
                        self.click_trail.borrow_mut().entry(button.cost).and_modify(|c| *c += 1).or_insert(1);

                        // Recursively find the cost from the origin_prize
                        // cost for current prize = cost for origin prize + button cost
                        let cost = self._optimal_cost(origin_prize).map(|c| c + button.cost);

                        // depress button; reduce counter by one (backtrack)
                        self.click_trail.borrow_mut().entry(button.cost).and_modify(|c| *c -= 1);

                        // Return the calculated cost for this path (if any)
                        cost
                    })
            })
            // capture the minimum cost from all possible preceding steps (button presses)
            .min();

        // Cache the calculated minimum cost for the current prize position
        // This happens *after* the minimum is found across all paths
        self.cache.borrow_mut().insert(prize, min_cost);

        // Return the minimum cost found
        min_cost
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
    type Err = nom::Err<Error<String>>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use crate::parser::parse_button;

        match parse_button(input) {
            Ok((_, button)) => Ok(button),
            Err(err) => Err(err.to_owned())
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
        assert_eq!(
            clawmachine._calculate_cost(prize).unwrap().0,
            280
        );
    }
}
