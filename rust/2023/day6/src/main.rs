use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;

    let mut nums = inp.trim().lines().map(|l| {
        l[l.find(":").unwrap() + 1..]
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
    });
    let n2 = nums.clone();
    let races = nums.next().unwrap().zip(nums.next().unwrap());

    let sol: usize = races
        .map(|(time, rec)| (1..time).filter(|i| (time - i) * i > rec).count())
        .product();
    println!("{sol}");

    let (time, rec) = n2
        .map(|it| {
            String::from_iter(it.map(|x| x.to_string()))
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    let c = (1..time).filter(|i| (time - i) * i > rec).count();
    Ok(println!("{c}"))
}
