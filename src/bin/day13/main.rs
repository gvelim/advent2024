mod parser;
mod machine;

use parser::parse_prize_clawmachine;


fn main() {
    let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Failed to read input file");

    let runs = input.split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res|
            res.map(|(_,res)| res)
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    runs.iter()
        .for_each(|run| {
            let (prize, machine) = run;
            print!("{:?} -> ", machine);
            print!("{:?} = ", prize);
            println!("{:?}", machine.optimal_cost(*prize));
        });
}
