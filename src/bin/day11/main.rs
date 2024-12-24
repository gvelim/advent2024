
fn main() {

}

type Stone = usize;

trait Blink {
    fn blink(self) -> [Option<Stone>;2];
}

impl Blink for Stone {
    fn blink(self) -> [Option<Stone>;2] {
        if self == 0 {
            [Some(1),None]
        } else if self % 2 == 0 {
            split_stone(self)
        } else {
            [Some(self * 2024),None]
        }
    }
}

fn split_stone(stone: Stone) -> [Option<Stone>;2] {
    let s = stone.to_string();
    if s.len() % 2 != 0 {
        return [None,None]
    }
    [
        s[0..s.len()/2].parse::<Stone>().ok(),
        s[s.len()/2..].parse::<Stone>().ok()
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_split_stone() {
        assert_eq!(split_stone(1234), [Some(12),Some(34)]);
        assert_eq!(split_stone(12345), [None,None]);
        assert_eq!(split_stone(123456), [Some(123),Some(456)]);
        assert_eq!(split_stone(120056), [Some(120),Some(56)]);
    }

    #[test]
    fn test_blink() {
        assert_eq!(0.blink(), [Some(1),None]);
        assert_eq!(1.blink(), [Some(2024),None]);
        assert_eq!(22.blink(), [Some(2),Some(2)]);
        assert_eq!(3.blink(), [Some(6072),None]);
        assert_eq!(6072.blink(), [Some(60),Some(72)]);
        assert_eq!(60.blink(), [Some(6),Some(0)]);
    }
}
