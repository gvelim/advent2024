mod blinker;

use std::time::Instant;
use blinker::{Blinker, Stone};

fn main() {
    let stones = vec![1 as Stone, 24596, 0, 740994, 60, 803, 8918, 9405859];
    let mut blinker = Blinker::default();

    let mut blink_counter = |stones: &[Stone], blinks: usize| {
        stones
            .iter()
            .map(|&stone| blinker.count(blinks, stone))
            .sum::<usize>()
    };

    let t = Instant::now();
    let count = blink_counter(&stones, 25);
    println!("Part 1: {count} stones after blinking 25 times - {:?}",t.elapsed() );
    assert_eq!(203457, count);

    let t = Instant::now();
    let count = blink_counter(&stones, 75);
    println!("Part 2: {count} stones after blinking 75 times - {:?}",t.elapsed() );
    assert_eq!(241394363462435, count);
}
