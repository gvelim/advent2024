use std::rc::Rc;

use advent2024::field::Field;
use advent2024::location::*;

pub type Lab = Field<char>;

#[derive(Debug)]
pub(crate) struct Guard<'a> {
    pub lab: &'a Lab,
    pub dir: DirVector,
    pub pos: Location
}

impl Iterator for Guard<'_> {
    type Item = (Location, DirVector);

    fn next(&mut self) -> Option<Self::Item> {
        // turn until you find a way fwd
        while let Some(&'#') = self.lab.peek(self.pos, self.dir) {
            self.dir = turn_cw(self.dir);
        }
        // move next position as long as it is within bounds
        self.pos.move_relative(self.dir)
            .filter(|&p| self.lab.within_bounds(p))
            .map(|pos| {
                self.pos = pos;
                (pos, self.dir)
            })
    }
}

pub fn is_loop_detected(mut guard: Guard) -> bool {
    use std::collections::HashMap;

    let mut history = HashMap::new();
    let (pos,dir) = (guard.pos, guard.dir);

    // register starting position
    history.entry(pos).or_insert(dir);
    // carry on until we fall off the lab
    // or we step onto a position already visited in the same direction
    !guard.all(|(nl,nd)| {
        let found = history.get(&nl).is_some_and(|&pd| nd == pd);
        history.entry(nl).or_insert(nd);
        !found
    })
}

pub fn find_guard(lab: &Lab, token: &[char]) -> Option<(Location, DirVector)> {
    lab
        .iter()
        .position(|c| token.contains(c))
        .map(|idx| {
            let loc = lab.index_to_cartesian(idx);
            (
                loc,
                lab.get(loc)
                    .map(|val|
                        match &val {'^' => (0,-1),'>' => (1,0),'v' => (0,1),'<' => (-1,0), _ => unreachable!()}
                    )
                    .unwrap()
            )
        })
}


#[test]
fn test_find_guard() -> Result<(),()> {
    let dt = [
        ("...\n.<.\n...\n...",Some((Location(1,1), (-1_isize,0_isize)))),
        ("...\n^..\n...\n...",Some((Location(0,1), (0,-1)))),
        ("...\n..>\n...\n...",Some((Location(2,1), (1,0)))),
        ("...\n...\n.v.\n...",Some((Location(1,2), (0,1)))),
        ("...\n...\n...\n.^.",Some((Location(1,3), (0,-1)))),
        ("...\n...\n...\n...",None)
    ];
    for (l, out) in dt.into_iter() {
        let lab = l.parse::<Lab>()?;
        assert_eq!(find_guard(&lab, &['^','>','v','<']), out, "{:#?}, {:#?}",lab, out);
    }
    Ok(())
}
