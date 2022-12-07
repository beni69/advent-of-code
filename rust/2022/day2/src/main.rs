use lib::{input::*, year_day};
use std::{cmp::Ordering, error::Error, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let inp = get_input(year_day!())?;
    let inp: Vec<Vec<_>> = inp
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    {
        let p1: Vec<(Instruction, Instruction)> = inp
            .iter()
            .map(|l| (l[0].parse().unwrap(), l[1].parse().unwrap()))
            .collect();
        let mut score = 0;
        for (left, right) in p1 {
            score += match right.partial_cmp(&left).unwrap() {
                Ordering::Greater => 6,
                Ordering::Equal => 3,
                Ordering::Less => 0,
            } + right as u32
                + 1;
        }
        println!("part 1: {score}");
    }

    let p2 = inp
        .iter()
        .map(|l| {
            let i: Instruction = l[0].parse().unwrap();
            cmp_to_score(&i, l[1])
        })
        .sum::<u32>();

    println!("part 2: {p2:?}");

    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Rock,
    Paper,
    Scissors,
}
impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("parse failed: {s}").into()),
        }
    }
}
impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if match self {
            Instruction::Rock => other == &Instruction::Scissors,
            Instruction::Paper => other == &Instruction::Rock,
            Instruction::Scissors => other == &Instruction::Paper,
        } {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Less)
    }
}

fn find_cmp(other: &Instruction, cmp: Ordering) -> Instruction {
    for i in [Instruction::Rock, Instruction::Paper, Instruction::Scissors] {
        if i.partial_cmp(other).unwrap() == cmp {
            return i;
        }
    }
    unreachable!()
}
fn cmp_to_score(other: &Instruction, cmp: &str) -> u32 {
    match cmp {
        "Y" => (*other as u32 + 1) + 3,
        "X" => find_cmp(other, Ordering::Less) as u32 + 1,
        "Z" => find_cmp(other, Ordering::Greater) as u32 + 1 + 6,
        _ => unreachable!("{cmp}"),
    }
}
