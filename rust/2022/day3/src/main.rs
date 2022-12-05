use std::error::Error;

use lib::{input::get_input, year_day};

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(year_day!())?;
    let input = input.lines().collect::<Vec<_>>();

    {
        let input = input
            .clone()
            .iter()
            .map(|l| (l[0..l.len() / 2].to_string(), l[l.len() / 2..].to_string()))
            .collect::<Vec<_>>();

        let mut dupe = Vec::new();
        for (a, b) in input {
            dupe.append(&mut common_chars(&a, &b));
        }

        let mut res = Vec::new();
        for c in dupe {
            res.push(priority(c));
        }
        println!("part 1: {}", res.iter().sum::<u32>());
    }

    let mut iter = input.iter();
    let mut dupe = Vec::new();
    for _ in 0..input.len() / 3 {
        let a = common_chars(iter.next().unwrap(), iter.next().unwrap())
            .into_iter()
            .collect::<String>();
        dupe.append(&mut common_chars(&a, iter.next().unwrap()))
    }

    let mut res = Vec::new();
    for c in dupe {
        res.push(priority(c));
    }
    println!("part 2: {}", res.iter().sum::<u32>());

    Ok(())
}

fn common_chars(a: &str, b: &str) -> Vec<char> {
    let mut line = Vec::new();
    for c in a.chars() {
        if b.contains(c) && !line.contains(&c) {
            line.push(c);
        }
    }
    line
}
fn priority(c: char) -> u32 {
    c as u32 - if c.is_lowercase() { 96 } else { 38 }
}
