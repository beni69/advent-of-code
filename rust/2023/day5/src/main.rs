use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let mut lines = inp.trim().split("\n\n");
    let mut seeds = lines.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    for map in lines {
        // prepare map
        let transforms = map
            .lines()
            .skip(1)
            .map(|l| {
                let nums = l
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec();
                // (start, end, x)
                (nums[1], nums[1] + nums[2], nums[0] - nums[1])
            })
            .collect_vec();

        // apply transform
        for i in 0..seeds.len() {
            let tr = transforms
                .iter()
                .find(|(start, end, _)| *start <= seeds[i] && seeds[i] <= *end)
                .map_or(0, |(_, _, x)| *x);
            seeds[i] += tr;
        }
    }
    println!("{}", seeds.iter().min().unwrap());
    Ok(())
}
