use lib::{input::get_input_lines, year_day, Result};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let (size_i, size) = input
        .iter()
        .enumerate()
        .find(|l| l.1.trim().starts_with('1'))
        .unwrap();
    let size = size
        .trim()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()?;
    println!("size: {size}");

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); size];
    for line in input.iter().take(size_i).rev() {
        let mut chars = line.chars();
        for i in 0..size {
            let item = [
                chars.next().unwrap_or(' '),
                chars.next().unwrap_or(' '),
                chars.next().unwrap_or(' '),
            ];
            chars.next(); // skip space
            if item[0] == '[' && item[2] == ']' {
                stacks[i].push(item[1]);
            }
        }
    }

    let instructions = input.iter().skip(size_i + 2).map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut stacks2 = stacks.clone();
    for step in instructions.clone() {
        for _ in 0..step[0] {
            let item = stacks[step[1] - 1].pop().unwrap();
            stacks[step[2] - 1].push(item);
        }
    }

    // part 2
    for step in instructions {
        let mut res = VecDeque::new();
        for _ in 0..step[0] {
            let item = stacks2[step[1] - 1].pop().unwrap();
            res.push_front(item);
        }
        stacks2[step[2] - 1].append(&mut res.make_contiguous().to_vec());
    }

    let res = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    let res2 = stacks2
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    println!("part 1: {res}\npart 2: {res2}");

    Ok(())
}
