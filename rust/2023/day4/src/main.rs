use itertools::Itertools;
use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let cards = inp
        .trim()
        .lines()
        .map(|l| {
            l[l.find(':').unwrap() + 2..].split('|').map(|p| {
                p.split_ascii_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
            })
        })
        .collect_vec();

    let mut s1 = 0;
    let mut count = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        let winners = card.flatten().duplicates().count();
        // part 1
        if winners > 0 {
            s1 += 2u32.pow(winners as u32 - 1);
        }
        // part 2
        for j in i + 1..i + 1 + winners {
            count[j] += count[i];
        }
    }
    let s2: u32 = count.iter().sum();
    Ok(println!("{s1}\n{s2}"))
}
