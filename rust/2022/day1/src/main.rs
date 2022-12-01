use lib::{input::get_input, year_day};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut inp = get_input(year_day!())?
        .split("\n\n")
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
    inp.sort_unstable();
    inp.reverse();

    println!(
        "part 1: {}\npart 2: {}",
        inp[0],
        inp.iter().take(3).sum::<u32>()
    );
    Ok(())
}
