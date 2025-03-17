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

    #[derive(Debug)]
    struct Button {
        dir: (u32,u32),
        cost: u8
    }

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
            map(
                separated_pair(
                    parse_button_id,
                    tag(":"),
                    parse_button_dir
                ),
                |(id, dir)| Button { dir, cost: if id == "A" { 3 } else { 1 }}
            )(button)
        );
    }
}
