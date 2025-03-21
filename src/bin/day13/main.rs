use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};
use advent2024::location::{reverse_dirvector, DirVector, Location};
use nom::{
    bytes::complete::{tag, take_till},
    character::{complete::alpha1, is_digit},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = std::fs::read_to_string("sample.txt").expect("Failed to read input file");


}

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
    type Err = nom::Err<()>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match map(
            separated_pair(
                map(
                    preceded(tag("Button "), alpha1),
                    |id| if id == "A" { 3 } else { 1 }
                ),
                tag(":"),
                parse_numbers_pair
            ),
            |(cost, (dx,dy))| Button { dir: (dx as isize, dy as isize), cost}
        )(input) {
            Ok((_, button)) => Ok(button),
            Err(err) => Err(err)
        }
    }
}


fn parse_prize(input: &str) -> Result<Location, nom::Err<()>> {
    match preceded(tag("Prize:"), parse_numbers_pair)(input) {
        Ok((_, (x,y))) => Ok(Location(x as usize, y as usize)),
        Err(err) => Err(err)
    }
}

fn parse_numbers_pair(input: &str) -> IResult<&str, (u32,u32), ()> {
    separated_pair(
        preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32),
        tag(","),
        preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32)
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_optimal_cost() {
        let buttons = [
            "Button A: X+94, Y+34".parse::<Button>().unwrap(),
            "Button B: X+22, Y+67".parse::<Button>().unwrap()
        ];
        let prize = parse_prize("Prize: X=8400, Y=5400").unwrap();
        assert_eq!(
            ClawMachine::new(&buttons).optimal_cost(prize),
            Some(280)
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!("Button A: X+10, Y+10".parse::<Button>(), Ok(Button { dir: (10, 10), cost: 3 }));
        assert_eq!("Button A:X+10,Y+10".parse::<Button>(), Ok(Button { dir: (10, 10), cost: 3 }));
        assert!("ButtonA:X+10,Y+10".parse::<Button>().is_err());
        assert_eq!(parse_prize("Prize: X=8400, Y=5400"),Ok(Location(8400, 5400)));
        assert_eq!(parse_prize("Prize:X=8400,Y=5400"),Ok(Location(8400, 5400)));
        assert!(parse_prize("X=8400, Y=5400").is_err());
    }
}
