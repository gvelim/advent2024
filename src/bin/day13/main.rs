fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use nom::{bytes::complete::{tag, take_till}, character::{complete::alpha1, is_digit}, combinator::map, sequence::{
        preceded,
        separated_pair
    }, IResult};

    #[derive(Debug)]
    struct Button {
        dir: (u32,u32),
        cost: u8
    }

    #[test]
    fn test_parse() {
        let button = "Button A: X+10, Y+10";
        let prize = "Prize: X=8400, Y=5400";

        let parse_button_cost = map(
            preceded(tag::<_,_,()>("Button "), alpha1),
            |id| if id == "A" { 3 } else { 1 }
        );
        let parse_button_dir = separated_pair(
            preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32),
            tag(","),
            preceded(take_till(|c| is_digit(c as u8)), nom::character::complete::u32)
        );

        println!("Parsed button: {:?}",
            map(
                separated_pair(parse_button_cost, tag(":"), parse_button_dir),
                |(cost, dir)| Button { dir, cost}
            )(button)
        );
    }
}
