use std::{
    fs,
    num::ParseIntError,
    rc::{self, Rc},
    str::FromStr,
};

fn main() {
    let input = fs::read_to_string("src/bin/day2/sample.txt").expect("File not found");
    let lists = input
        .lines()
        .map(|line| line.parse::<Report>().expect("Invalid list"))
        .collect::<Vec<Report>>();

    for ele in lists.iter() {
        println!("{:?} => {}", ele, ele.is_safe())
    }
}

#[derive(Debug)]
enum Direction {
    Asc,
    Desc,
}

#[derive(Debug)]
struct Report {
    levels: rc::Rc<[usize]>,
    dir: Direction,
}

impl Report {
    fn is_safe(&self) -> bool {
        self.levels.windows(2).all(|a| {
            let d = a[0].abs_diff(a[1]);
            (1..=3).contains(&d)
                && match self.dir {
                    Direction::Asc => a[0] < a[1],
                    Direction::Desc => a[0] > a[1],
                }
        })
    }
}

fn direction(levels: Rc<[usize]>) -> Result<Direction, String> {
    let mut iter = levels.iter();
    let first = iter.next().unwrap();
    for last in iter {
        match first.cmp(last) {
            std::cmp::Ordering::Less => return Ok(Direction::Asc),
            std::cmp::Ordering::Greater => return Ok(Direction::Desc),
            std::cmp::Ordering::Equal => (),
        };
    }
    Err("all numbers equal to first".to_string())
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>())
            .collect::<Result<rc::Rc<[usize]>, ParseIntError>>()?;
        let dir = direction(levels.clone()).expect("msg");
        Ok(Report { levels, dir })
    }
}
