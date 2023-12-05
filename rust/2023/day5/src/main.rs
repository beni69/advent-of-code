use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let mut lines = inp.trim().split("\n\n");
    let mut seeds = lines.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let s2 = seeds.clone();
    let l2 = lines.clone();

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

    let mut seeds = s2
        .into_iter()
        .tuples::<(_, _)>()
        // (start, len) -> (start, end)
        .map(|(start, len)| (start, start + len - 1))
        .collect_vec();
    for map in l2 {
        for s in &seeds {
            eprintln!("{s:?}");
        }
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

        let s2 = seeds.clone();
        let mut rm = Vec::new(); // prevent index shift
        dbg!(s2.len());
        for i in 0..s2.len() {
            for tr in &transforms {
                let start = i64::max(s2[i].0, tr.0);
                let end = i64::min(s2[i].1, tr.1);

                // no overlap
                if end < start {
                    continue;
                }

                rm.push(i);
                seeds.push((start + tr.2, end + tr.2));
                if s2[i].0 < start {
                    seeds.push((s2[i].0, start - 1));
                }
                if s2[i].1 > end {
                    seeds.push((end + 1, s2[i].1));
                }

                eprintln!("\n===== {tr:?}");
                for s in &seeds {
                    eprintln!("{s:?}");
                }
                eprintln!("=====\n");

                break;
            }
        }
        eprintln!("rm: {rm:?}");
        rm.into_iter().rev().for_each(|i| {
            seeds.remove(i);
        });
        dbg!(seeds.len());
    }

    for s in &seeds {
        eprintln!("{s:?}");
    }

    let min = seeds
        .iter()
        .map(|(start, _)| start)
        .filter(|x| **x != 0)
        .min()
        .unwrap();
    println!("{min}");

    Ok(())
}
