fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use nom::{
        bytes::complete::tag, character::complete::{
            alpha1,
            alphanumeric1
        },
        sequence::{
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
        let parse_button_dir = separated_pair(
            preceded(tag(" X+"), alphanumeric1),
            tag(","),
            preceded(tag(" Y+"), alphanumeric1),
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
