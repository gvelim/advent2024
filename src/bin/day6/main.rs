use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use advent2024::field::Field;
use advent2024::location::*;

type Lab = Field<char>;

fn main() {
    let input = std::fs::read_to_string("src/bin/day6/sample.txt").expect("msg");
    let lab = Rc::new(input.parse::<Lab>().expect("Field parse err"));

    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");

    let mut unique_locations  = Guard{lab: lab.clone(),pos,dir}.collect::<HashMap<Location,DirVector>>();
    unique_locations.insert(pos,dir);
    println!("Part 1: Guard visited {:?} unique locations", unique_locations.len());
    // assert_eq!(unique_locations.len(),5534);
    // assert_eq!(unique_locations.len(),41);

    let mut history = HashMap::new();
    history.insert(pos, dir);
    let obs_count = Guard{lab:lab.clone(),pos,dir}
        .filter_map(|(l, d)| {
            history.entry(l).or_insert(d);
            // send a Guard off to your right; the Guard will either exit the lab or hit a loop condition
            Guard{lab:lab.clone(),pos:l,dir:turn_cw(d)}
                // carry on until we've entered a loop, that is, we are back where we've started
                .take_while(|&(nl,nd)| nl != l && nd != d )
                // obstacle position conditions
                .filter(|&(nl,nd)|
                    // we've been here before and...
                    if let Some(&before) = history.get(&nl) {
                        match lab.peek(nl, nd) {
                            // ... going in a clock-wise direction while we are facing a #
                            Some(&'#') if before == turn_cw(nd) => true,
                            // ... going in the same direction vector
                            Some(_) if before == nd => true,
                            _ => false,
                        }
                    } else { false }
                )
                .next()
                .map(|_| (l,d))
        })
        .filter_map(|(l,d)|
            l.move_relative(d)
                .filter(|&nl| nl != l )
        )
        .collect::<HashSet<_>>();

    println!("Part 2: There are {:?} loop obstacles", obs_count.len());
    // assert_eq!(obs_count.len(),5534);
    // assert_eq!(obs_count,6);

    print_all(&lab, &unique_locations, &obs_count);
}

fn print_all(lab:&Lab, path: &HashMap<Location,DirVector>, obst: &HashSet<Location>) {
    (0..lab.height()).for_each(|y| {
        (0..lab.width()).for_each(|x| {
            let loc = Location(x,y);
            let c = match (lab.get(loc), path.get(&loc), obst.contains(&loc)) {
                (None, _, _) => unreachable!(),
                (_, _, true) => 'O',
                (_, Some(&d), _) => ddv(d),
                (Some(&c), _, _) => c,
            };
            print!("{c:2}");
        });
        println!();
    });
}

fn ddv(d:DirVector)-> char {
    match d {
        (1,0) => '→',
        (-1,0) => '←',
        (0,-1) => '↑',
        (0,1) => '↓',
        _ => unreachable!()
    }
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
                self.lab.get(loc)
                    .and_then(|c|{
                        self.dir = match c {
                            &'#' => turn_cw(self.dir),
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
        .inspect(|p| println!("pos: {:?}, {:?}",p, lab))
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
