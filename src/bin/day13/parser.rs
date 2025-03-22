use advent2024::location::Location;
use nom::{
    bytes::complete::{tag, take_till},
    character::{complete::alpha1, is_digit},
    combinator::map,
    sequence::{preceded, separated_pair, terminated},
    IResult
};

use crate::machine::{Button, ClawMachine};

// expects three lines in the form of
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
pub(super) fn parse_prize_clawmachine(input: &str) -> IResult<&str, (Location, ClawMachine)> {

    let (input, button_a) = terminated(parse_button, tag("\n"))(input)?;
    let (input, button_b) = terminated(parse_button, tag("\n"))(input)?;
    let (input, prize) = parse_prize(input)?;

    Ok((input, (prize, ClawMachine::new(&[button_a, button_b]))))
}

// expects "Prize: X=8400, Y=5400"
pub(super) fn parse_prize(input: &str) -> IResult<&str, Location> {
    map(
        preceded(tag("Prize:"), parse_numbers_pair),
        |(x,y)| Location(x as usize, y as usize)
    )(input)
}

// expects "Button A: X+94, Y+34"
pub(super) fn parse_button(input: &str) -> IResult<&str, Button> {
    map(
        separated_pair(
            map(
                preceded(tag("Button "), alpha1),
                |id| if id == "A" { 3 } else { 1 }
            ),
            tag(":"),
            parse_numbers_pair
        ),
        |(cost, (x,y))| Button::new((x as isize, y as isize), cost)
    )(input)
}

// expects " X+94, Y+34"
pub(super) fn parse_numbers_pair(input: &str) -> IResult<&str, (u32,u32)> {
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
    fn test_parse_claw_engine() {
        let input = "Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400";

        println!("{:?}", parse_prize_clawmachine(input));
    }

    #[test]
    fn test_parse_button() {
        assert_eq!("Button A: X+10, Y+10".parse::<Button>(), Ok(Button::new((10, 10),3)));
        assert_eq!("Button A:X+10,Y+10".parse::<Button>(), Ok(Button::new((10, 10),3)));
        assert!("ButtonA:X+10,Y+10".parse::<Button>().is_err());
        assert_eq!(parse_prize("Prize: X=8400, Y=5400"),Ok(("",Location(8400, 5400))));
        assert_eq!(parse_prize("Prize:X=8400,Y=5400"),Ok(("",Location(8400, 5400))));
        assert!(parse_prize("X=8400, Y=5400").is_err());
    }
}
