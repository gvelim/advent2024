mod machine;
mod parser;

use parser::parse_prize_clawmachine;

fn main() {
    let input =
        std::fs::read_to_string("src/bin/day13/input.txt").expect("Failed to read input file");

    let runs = input
        .split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res| res.map(|(_, res)| res))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| panic!("{e:?}"))
        .unwrap();

    let sum = runs
        .iter()
        .inspect(|(prize, machine)| print!("{machine:?} -> {prize:?} = "))
        .filter_map(|(prize, machine)| {
            if let Some((cost, paths)) = machine.optimal_cost(*prize) {
                println!("{cost}");
                println!("{:->5}Optimal Path: {:?}", ' ', paths);
                Some(cost)
            } else {
                println!("No Solution");
                None
            }
        })
        .sum::<u32>();

    println!("Total Sum: {sum}");
}
