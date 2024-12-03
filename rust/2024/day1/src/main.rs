use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let inp: Vec<(i32, i32)> = inp
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for (x, y) in inp {
        v1.push(x);
        v2.push(y);
    }

    v1.sort_unstable();
    v2.sort_unstable();

    let s1: u32 = v1.iter().zip(v2.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
    dbg!(s1);

    let occ = v2.iter().counts();
    dbg!(&occ);
    let s2: i32 = v1
        .iter()
        .map(|x| x * (*occ.get(x).unwrap_or(&0) as i32))
        .sum();
    dbg!(s2);

    Ok(())
}
