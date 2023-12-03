use itertools::Itertools;
use lib::{input::get_input, year_day, Result};
use std::iter;

fn nearby_coords(matrix: &Vec<Vec<char>>, i: usize, j: usize, len: usize) -> Vec<(usize, usize)> {
    let it = usize::saturating_sub(j, 1)..usize::min(j + len + 1, matrix[i].len());
    let mut nearby = Vec::new();
    // felette sor
    if i > 0 {
        nearby.append(&mut iter::repeat(i - 1).zip(it.clone()).collect());
    }
    // alatta sor
    if i + 1 < matrix.len() {
        nearby.append(&mut iter::repeat(i + 1).zip(it).collect());
    }
    // elotte
    if j > 0 {
        nearby.push((i, j - 1));
    }
    // utana
    if j + len < matrix[i].len() {
        nearby.push((i, j + len));
    }
    nearby
}

fn main() -> Result<()> {
    let inp = get_input(year_day!())?;
    let matrix: Vec<Vec<char>> = inp.trim().lines().map(|l| l.chars().collect()).collect();

    let mut numbers = Vec::new();
    let mut lines = matrix.iter().enumerate();
    // find coords of continuous digits
    while let Some((i, line)) = lines.next() {
        let mut chars = line.iter().enumerate();
        while let Some((j, char)) = chars.next() {
            if char.is_ascii_digit() {
                let mut len = 1usize;
                while let Some(true) = chars.next().map(|c| c.1.is_ascii_digit()) {
                    len += 1;
                }

                numbers.push((i, j, len));
            }
        }
    }

    let mut s1 = 0;
    for (i, j, len) in numbers {
        let n: u32 = String::from_iter(&matrix[i][j..j + len]).parse().unwrap();

        let nearby = nearby_coords(&matrix, i, j, len);

        let valid = nearby
            .into_iter()
            .map(|(i, j)| matrix[i][j])
            .any(|c| c != '.' && !c.is_ascii_digit()); // symbol=nem '.' vagy szám

        if valid {
            s1 += n;
        }
    }
    println!("{s1}");

    let gears = matrix
        .iter()
        .enumerate()
        // find stars
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(j, c)| if *c == '*' { Some((i, j)) } else { None })
        })
        .map(|(i, j)| {
            // go around them
            let gear = nearby_coords(&matrix, i, j, 1)
                .into_iter()
                .filter(|(i, j)| matrix[*i][*j].is_ascii_digit())
                // put nearby digits into numbers
                .map(|(i, mut j)| {
                    let mut len = 1;

                    // walk forward
                    let mut it = matrix[i].iter().skip(j + 1);
                    while let Some(true) = it.next().map(|c| c.is_ascii_digit()) {
                        len += 1;
                    }

                    // walk back
                    let mut rev = matrix[i].iter().rev().skip(matrix[i].len() - j);
                    while let Some(true) = rev.next().map(|c| c.is_ascii_digit()) {
                        j -= 1;
                        len += 1;
                    }
                    (i, j, len)
                })
                .unique()
                .map(|(i, j, len)| String::from_iter(&matrix[i][j..j + len]).parse().unwrap())
                .collect_vec();

            if gear.len() == 2 {
                gear.iter().product::<u32>()
            } else {
                0
            }
        })
        .sum::<u32>();
    Ok(println!("{gears}"))
}
