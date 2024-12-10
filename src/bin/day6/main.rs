mod guard;

use crate::guard::*;
use std::{collections::HashMap, time::Instant};
use advent2024::location::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day6/input.txt").expect("msg");
    let mut lab = input.parse::<Lab>().expect("Field parse err");

    let (pos,dir) = find_guard(&lab, &['^','>','v','<']).expect("there is no Lab Guard !!");

    let t = Instant::now();
    let mut unique_locations  = Guard{lab:&lab,pos,dir}.collect::<HashMap<_,_>>();
    unique_locations.insert(pos,dir);
    println!("Part 1: Guard visited {:?} unique locations - {:?}", unique_locations.len(), t.elapsed());
    assert_eq!(unique_locations.len(),5534);

    let t = Instant::now();
    let obs_count = unique_locations.into_iter()
        .filter(|&(l, _)| {
            lab.get_mut(l).map(|c| *c = '#');
            let is_loop = is_loop_detected(Guard{lab:&lab,pos,dir});
            lab.get_mut(l).map(|c| *c = '.');
            is_loop
        })
        .count();

    println!("Part 2: There are {:?} loop obstacles - {:?}", obs_count, t.elapsed());
    assert_eq!(obs_count,2262);
    // assert_eq!(obs_count,6);
}

fn print_all(start: Location, guard: &Guard, path: &HashMap<Location,DirVector>, obst: Option<&Vec<Location>>) {
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
