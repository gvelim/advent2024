use std::{collections::{HashMap, HashSet}, num::ParseIntError, str::FromStr};

fn main() {
    let input = std::fs::read_to_string("src/bin/day5/sample.txt").expect("msg");
    let mut s = input.split("\n\n");
    let rules = s.next().unwrap();
    let lists = s.next().unwrap();

    for ele in lists.lines() {
        println!("{:?}", ele.parse::<Update>())
    }

    println!("{:?}", rules.parse::<OrderRules>())
}

#[derive(Debug)]
struct OrderRules {
    rules: HashMap<usize,HashSet<usize>>
}

impl FromStr for OrderRules {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for l in s.lines() {
            let mut s = l.split('|');
            let x = s.next().unwrap().parse::<usize>()?;
            let y = s.next().unwrap().parse::<usize>()?;
            rules
                .entry(x)
                .and_modify(|s: &mut HashSet<usize>| {s.insert(y);})
                .or_insert(HashSet::new())
                .insert(y);
        }
        Ok(OrderRules{rules})
    }
}

#[derive(Debug, PartialEq)]
struct Update {
    list: HashMap<usize,usize>
}

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Update {
            list: s
                .split(',')
                .enumerate()
                .map(|(i,numeric)|
                    numeric.parse::<usize>().map(|num| (num,i))
                )
                .collect::<Result<HashMap<usize,usize>,ParseIntError>>()?
        })
    }
}

#[test]
fn test_parse_update() {
    assert_eq!(
        "75,47,61,53,29".parse::<Update>().unwrap(),
        Update { list: HashMap::from([(75,0),(47,1),(61,2),(53,3),(29,4)]) }
    );
}
