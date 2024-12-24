use rayon::prelude::*;

fn main() {
    // let stones = vec![125, 17];
    let stones = vec![1, 24596, 0, 740994, 60, 803, 8918, 9405859];

    let blink_once =
        |stones: Vec<Stone>| stones
            .into_par_iter()
            .flat_map(|stone| stone.blink())
            .flatten();

    let stones = (0..25).fold(stones, |stones, _|{
        blink_once(stones).collect::<Vec<_>>()
    });
    println!("Part 1: {} stones after blinking 25 times",stones.len() );
    assert_eq!(203457, stones.len());

    let stones = (25..45).fold(stones, |stones, c|{
        let v = blink_once(stones).collect::<Vec<_>>();
        println!("{c} - {:?}",v.len());
        v
    });
    println!("Part 2: {} stones after blinking 75 times",stones.len() );
    // assert_eq!(203457, stones.len());
}

type Stone = u64;
trait Blink {
    fn blink(self) -> [Option<Stone>;2];
    fn is_even_digit(&self) -> bool;
}
impl Blink for Stone {
    fn blink(self) -> [Option<Stone>;2] {
        if self == 0 {
            [Some(1),None]
        } else if self.is_even_digit() {
            split_stone(self)
        } else {
            [Some(self * 2024),None]
        }
    }
    fn is_even_digit(&self) -> bool {
        self.ilog10() % 2 == 1
    }
}

fn split_stone(stone: Stone) -> [Option<Stone>;2] {
    let m = (10 as Stone).pow((stone.ilog10() + 1) / 2);
    [
        Some(stone / m),
        Some(stone % m)
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
