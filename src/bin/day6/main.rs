use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use advent2024::field::Field;
use advent2024::location::*;

type Lab = Field<char>;

fn main() {
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let lab = Rc::new(input.parse::<Lab>().expect("Field parse err"));

    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");
    let mut unique_locations  = Guard{lab: lab.clone(),pos,dir}
        .map(|(l,_)| l)
        .collect::<HashSet<Location>>();
    unique_locations.insert(pos);
    println!("Part 1: Guard visited {:?} unique locations", unique_locations.len());
    // assert_eq!(unique_locations.len(),5534);
    // assert_eq!(unique_locations.len(),41);

    let mut history = HashMap::new();
    history.insert(pos, dir);
    let obs_count = Guard{lab:lab.clone(),pos,dir}
        .filter_map(|(l, d)| {
            if let Some(&h) = history.get(&l) {
                if turn_cw(d) == Some(h) { return Some((l,d)) }
            } else {
                let mut cur = l;
                while let Some(cl) = {
                    cur.move_relative(turn_cw(d).unwrap())
                        .filter(|&cl| lab.in_bounds(cl))
                } {
                    if let Some(&cd) = history.get(&cl) {
                        if turn_cw(d) == Some(cd) {
                            return Some((l,d))
                        }
                    }
                    cur = cl;
                }
            }
            history.entry(l).or_insert(d);
            None
        })
        .filter_map(|(l,d)| {
            l.move_relative(d)
                .filter(|&nl| nl != l )
        })
        .inspect(|l| println!("Obstacle {:?}",l))
        .collect::<HashSet<_>>();

    println!("Part 2: There are {:?} loop obstacles", obs_count.len());
    // assert_eq!(obs_count.len(),5534);
    // assert_eq!(obs_count,6);
}

#[derive(Debug)]
struct Guard {
    lab: Rc<Lab>,
    dir: DirVector,
    pos: Location
}

impl Iterator for Guard {
    type Item = (Location, DirVector);

    fn next(&mut self) -> Option<Self::Item> {
        self.pos
            .move_relative(self.dir)
            .and_then(|loc|
                self.lab.value_at(loc)
                    .and_then(|c|{
                        self.dir = match c {
                            &'#' => turn_cw(self.dir).unwrap(),
                            _ => self.dir
                        };
                        self.pos.move_relative(self.dir)
                    })
            )
            // .inspect(|l| println!("{l:?}"))
            .map(|pos| {
                self.pos = pos;
                (pos, self.dir)
            })
    }
}

fn find_guard(lab: &Lab, token: &[char]) -> Option<(Location, DirVector)> {
    lab
        .iter()
        .position(|c| token.contains(c))
        .map(|idx| {
            let y = idx / lab.height();
            let loc = Location(idx - y * lab.height(), y );
            (
                loc,
                lab.value_at(loc).map(|val|
                    match &val {'^' => (0,-1),'>' => (1,0),'v' => (0,1),'<' => (-1,0), _ => unreachable!()}
                ).unwrap()
            )
        })
}

#[test]
fn test_find_guard() -> Result<(),()> {
    let dt = [
        ("...\n.<.\n...",Some((Location(1,1), (-1_isize,0_isize)))),
        ("...\n^..\n...",Some((Location(0,1), (0,-1)))),
        ("...\n..>\n...",Some((Location(2,1), (1,0)))),
        ("...\n...\n.v.",Some((Location(1,2), (0,1)))),
        ("...\n...\n...",None)
    ];
    for (l, out) in dt.into_iter() {
        let lab = l.parse::<Lab>()?;
        assert_eq!(find_guard(&lab, &['^','>','v','<']), out, "{:#?}, {:#?}",lab, out);
    }
    Ok(())
}
