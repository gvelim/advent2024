use std::rc::Rc;

use advent2024::{location::*, field::Field};

fn main() {
    todo!()
}

type TopographicalMap = Field<u8>;

struct Trail {
    map: Rc<TopographicalMap>,
    pos: Location
}

impl Iterator for Trail {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
