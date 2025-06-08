use std::collections::HashMap;

pub type Stone = u64;

#[derive(Default)]
pub(crate) struct Blinker {
    cache: HashMap<(usize, Stone), usize>,
}

impl Blinker {
    pub(crate) fn count(&mut self, blink: usize, stone: Stone) -> usize {
        if blink == 0 {
            return 1;
        }
        if let Some(&ret) = self.cache.get(&(blink, stone)) {
            return ret;
        }
        let ret = match stone.blink() {
            BlinkResult::One(a) => self.count(blink - 1, a),
            BlinkResult::Two(a, b) => self.count(blink - 1, a) + self.count(blink - 1, b),
        };
        self.cache.insert((blink, stone), ret);
        ret
    }
}

trait Blink {
    fn blink(self) -> BlinkResult;
    fn has_even_digits(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
enum BlinkResult {
    One(Stone),
    Two(Stone, Stone),
}

impl Blink for Stone {
    fn blink(self) -> BlinkResult {
        if self == 0 {
            BlinkResult::One(1)
        } else if self.has_even_digits() {
            let m = (10 as Stone).pow(self.ilog10().div_ceil(2));
            BlinkResult::Two(self / m, self % m)
        } else {
            BlinkResult::One(self * 2024)
        }
    }
    fn has_even_digits(&self) -> bool {
        self.ilog10() % 2 == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blink_count() {
        let mut blinker = Blinker::default();
        assert_eq!(blinker.count(10, 0), 39);
    }

    #[test]
    fn test_blink() {
        assert_eq!(0.blink(), BlinkResult::One(1));
        assert_eq!(1.blink(), BlinkResult::One(2024));
        assert_eq!(22.blink(), BlinkResult::Two(2, 2));
        assert_eq!(3.blink(), BlinkResult::One(6072));
        assert_eq!(6072.blink(), BlinkResult::Two(60, 72));
        assert_eq!(60.blink(), BlinkResult::Two(6, 0));
        assert_eq!(1234.blink(), BlinkResult::Two(12, 34));
        assert_eq!(12345.blink(), BlinkResult::One(24986280));
        assert_eq!(123456.blink(), BlinkResult::Two(123, 456));
        assert_eq!(120006.blink(), BlinkResult::Two(120, 6));
        assert_eq!(120000.blink(), BlinkResult::Two(120, 0));
    }
}
