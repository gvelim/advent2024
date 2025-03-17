fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use nom::{
        bytes::complete::tag,
        character::complete::alpha1,
        combinator::map, sequence::{
            preceded,
            separated_pair
        }
    };

    #[test]
    fn test_parse() {
        let button = "Button A: X+10, Y+10";

        let parse_button_id = preceded(
            tag::<_,_,()>("Button "),
            alpha1
        );
        let parse_button_dir = map(
            separated_pair(
            preceded(tag(" X+"), nom::character::complete::u32),
            tag(","),
            preceded(tag(" Y+"), nom::character::complete::u32)
            ), |(a,b)| (a,b)
        );

        println!("Parsed button: {:?}",
            separated_pair(
                parse_button_id,
                tag(":"),
                parse_button_dir
            )(button)
        );
    }
}
