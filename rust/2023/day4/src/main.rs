use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;

    let mut sum = 0;
    for line in inp.trim().lines() {
        let x = line[line.find(':').unwrap() + 2..]
            .split('|')
            .flat_map(|p| {
                p.split_ascii_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
            });

        let winners = x.duplicates().count();
        if winners > 0 {
            sum += 2u32.pow(winners as u32 - 1);
        }
    }
    println!("{sum}");

    let mut s2 = 0;
    let lines = inp
        .trim()
        .lines()
        .map(|l| {
            l[l.find(':').unwrap() + 2..]
                .split('|')
                .map(|p| {
                    p.split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_vec()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();

    eprintln!("{lines:?}");

    Ok(())
}
