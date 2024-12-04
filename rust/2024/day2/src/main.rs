use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let inp = inp
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let s1 = inp
        .iter()
        // check for ascending or descending order
        .filter(|l| l.iter().sorted().eq(l.iter()) || l.iter().sorted().rev().eq(l.iter()))
        .filter(|l| {
            l.windows(2).all(|x| {
                let d = x[0].abs_diff(x[1]);
                d >= 1 && d <= 3
            })
        })
        .count();

    println!("{s1}");

    Ok(())
}
