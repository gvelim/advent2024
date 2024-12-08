use std::collections::HashSet;
use std::rc::Rc;

use advent2024::field::Field;
use advent2024::location::*;

type Lab = Field<char>;

fn main() {
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let lab = Rc::new(input.parse::<Lab>().expect("Field parse err"));

    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");
    let mut path  = Guard{lab,pos,dir}.collect::<HashSet<Location>>();
    path.insert(pos);
    println!("Part 1: Guard visited {:?} unique locations", path.len());
    assert_eq!(path.len(),5534)
}

#[derive(Debug)]
struct Guard {
    lab: Rc<Lab>,
    dir: DirVector,
    pos: Location
}

impl Iterator for Guard {
    type Item = Location;

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
            .inspect(|&pos| self.pos = pos)
    }
}

fn find_guard(lab: &Lab, guard: &[char]) -> Option<(Location, DirVector)> {
    lab
        .iter()
        .position(|c| guard.contains(c))
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
        assert_eq!(find_guard(&lab,&['^','>','v','<']), out, "{:#?}, {:#?}",lab, out);
    }
    Ok(())
}
