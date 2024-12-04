use std::{fs, num::ParseIntError, rc, str::FromStr, time};

fn main() {
    let input = fs::read_to_string("src/bin/day2/input.txt").expect("File not found");
    let lists = input
        .lines()
        .map(|line| line.parse::<Report>().expect("Invalid list"))
        .collect::<Vec<Report>>();

    let t = time::Instant::now();
    let count = lists.iter().filter(|r| r.is_safe()).count();
    println!("Part 1: {} = {:?}", count, t.elapsed());
    assert_eq!(count, 407);

    let t = time::Instant::now();
    let count = lists.iter().filter(|r| r.is_safe_dumpen()).count();
    println!("Part 2: {} - {:?}", count, t.elapsed());
    assert_eq!(count, 459);
}

#[derive(Debug)]
struct Report {
    levels: rc::Rc<[usize]>,
}

impl Report {
    fn validate(r: &[usize]) -> bool {
        let dir = r[0] < r[1];
        r.windows(2).all(|a| {
            (1..=3).contains(&(a[0].abs_diff(a[1])))
                && match dir {
                    true => a[0] < a[1],
                    false => a[0] > a[1],
                }
        })
    }

    fn is_safe(&self) -> bool {
        Report::validate(&self.levels)
    }

    fn is_safe_dumpen(&self) -> bool {
        (0..self.levels.len()).any(|p| {
            let mut levels = self.levels.to_vec();
            levels.remove(p);
            Report::validate(&levels)
        })
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            levels: s
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>())
                .collect::<Result<rc::Rc<[usize]>, ParseIntError>>()?
        })
    }
}
