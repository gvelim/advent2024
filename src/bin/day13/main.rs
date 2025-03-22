mod parser;
mod machine;

use parser::parse_prize_clawmachine;


fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Failed to read input file");

    let runs = input.split("\n\n")
        .map(|run| parse_prize_clawmachine(run))
        .map(|res| res.map(|(_,res)| res))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| panic!("{:?}", e) )
        .unwrap();

    let sum = runs.iter()
        .filter_map(|run| {
            let (prize, machine) = run;
            print!("{:?} -> ", machine);
            println!("{:?} = ", prize);
            let res = machine.optimal_cost(*prize);
            if let Some((cost,paths)) = res.clone() {
                println!("Optimal Cost: {:?}", cost);
                println!("Optimal Path: {:?}", paths);
                println!()
            }
            res
        })
        .map(|(cost,_)| cost)
        .sum::<u32>();

    println!("Total Sum: {}", sum);
}
