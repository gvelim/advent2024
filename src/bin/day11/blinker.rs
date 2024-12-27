use std::collections::HashMap;

pub type Stone = u64;

pub(crate) struct Blinker {
    cache: HashMap<(usize,Stone),usize>
}

impl Default for Blinker {
    fn default() -> Self {
        Blinker {
            cache: HashMap::new()
        }
    }
}

impl Blinker {
    pub(crate) fn count(&mut self, blink: usize, stone: Stone) -> usize {
        if blink == 0 { return 1 }
        if let Some(&ret) =  self.cache.get(&(blink,stone)) { return ret }
        let ret = match stone.blink() {
            [Some(a), None] => self.count(blink-1, a),
            [Some(a), Some(b)] =>
                self.count(blink-1, a)
                + self.count(blink-1, b),
            _ => unreachable!()
        };
        self.cache.insert((blink,stone), ret);
        ret
    }
}

fn split_stone(stone: Stone) -> [Option<Stone>;2] {
    let m = (10 as Stone).pow((stone.ilog10() + 1) / 2);
    [Some(stone / m), Some(stone % m)]
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blink_count() {
        let mut blinker = Blinker::default();
        assert_eq!(blinker.count(10, 0),39);
    }

    #[test]
    fn test_split_stone() {
        assert_eq!(split_stone(1234), [Some(12),Some(34)]);
        assert_eq!(split_stone(12345), [Some(123), Some(45)]);
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
