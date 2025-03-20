use std::str::FromStr;
use advent2024::location::{reverse_dirvector, DirVector, Location};
use nom::{
    bytes::complete::{tag, take_till},
    character::{complete::alpha1, is_digit},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Crane {
    loc: Location
}

impl Crane {
    fn new(prize: Location) -> Self {
        Crane { loc: prize }
    }
    fn back_a_step(&mut self, button: &Button) -> Option<Location> {
        if let Some(loc) = self.loc.move_relative(reverse_dirvector(button.dir)) {
            self.loc = loc;
            Some(loc)
        } else {
            None
        }
    }
    fn optimal_cost(&mut self, buttons: &[Button]) -> Option<u32> {
        // per button substract target
        // if new target is (0,0) then return Some(Button.cost)
        // if new target is less than 0 retun None; path has no solution
        // recurse Min( passing (a) new target, (b) button )
        buttons.iter()
            .filter_map(|button| {
                if let Some(loc) = self.back_a_step(button) {
                    if loc.is_origin() {
                        Some(button.cost as u32)
                    } else {
                        self.optimal_cost(buttons)
                    }
                } else {
                    None
                }
            })
            .min()
    }
}


#[derive(Debug, PartialEq)]
struct Button {
    dir: DirVector,
    cost: u8
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
    fn test_crane_back_a_step() {
        let mut crane = Crane::new(Location(10, 10));
        let button = Button { dir: (2, 2), cost: 3 };
        assert_eq!(crane.back_a_step(&button), Some(Location(8, 8)));
        assert_eq!(crane.back_a_step(&button), Some(Location(6, 6)));
        assert_eq!(crane.back_a_step(&button), Some(Location(4, 4)));
        assert_eq!(crane.back_a_step(&button), Some(Location(2, 2)));
        assert_eq!(crane.back_a_step(&button), Some(Location(0, 0)));
        assert_eq!(crane.back_a_step(&button), None);
        assert_eq!(Crane::new(Location(1,2)).back_a_step(&button), None);
        assert_eq!(Crane::new(Location(2,1)).back_a_step(&button), None);
        assert_eq!(Crane::new(Location(1,1)).back_a_step(&button), None);
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
