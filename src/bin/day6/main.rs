mod guard;

use crate::guard::*;
use std::{collections::{HashMap,HashSet}, rc::Rc};
use advent2024::location::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let lab = Rc::new(input.parse::<Lab>().expect("Field parse err"));

    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");

    let mut unique_locations  = Guard{lab: lab.clone(),pos,dir}.collect::<HashMap<Location,DirVector>>();
    unique_locations.insert(pos,dir);
    println!("Part 1: Guard visited {:?} unique locations", unique_locations.len());
    // assert_eq!(unique_locations.len(),5534);
    // assert_eq!(unique_locations.len(),41);

    let obs_count = Guard{lab:lab.clone(),pos,dir}
        .filter_map(|(l, d)|
            is_loop_detected(Guard{lab:lab.clone(),pos:l,dir:d})
                .then_some((l,d))
        )
        .filter_map(|(l,d)|
            l.move_relative(d)
                .filter(|&nl| nl != l )
        )
        .collect::<HashSet<_>>();

    println!("Part 2: There are {:?} loop obstacles", obs_count.len());
    // assert_eq!(obs_count.len(),2262);
    // assert_eq!(obs_count,6);

    // print_all(pos, &Guard{lab,pos,dir}, &unique_locations, Some(&obs_count));
}


fn print_all(start: Location, guard: &Guard, path: &HashMap<Location,DirVector>, obst: Option<&HashSet<Location>>) {
    println!();
    (0..guard.lab.height()).for_each(|y| {
        (0..guard.lab.width()).for_each(|x| {
            let loc = Location(x,y);
            if loc == guard.pos { print!("😀"); return; };
            if loc == start { print!("🚀"); return; };
            let c = match (guard.lab.get(loc), path.get(&loc), obst.map(|o| o.contains(&loc))) {
                (None, _, _) => unreachable!(),
                (_, _, Some(true)) => 'O',
                (_, Some(&d), _) => ddv(d),
                (Some(&c), _, _) => c,
            };
            print!("{c:2}");
        });
        println!();
    });
}

fn ddv(d:DirVector)-> char {
    match d { (1,0) => '→', (-1,0) => '←', (0,-1) => '↑', (0,1) => '↓', _ => unreachable!() }
}
