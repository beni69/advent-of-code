use lib::{input::get_input_lines, year_day, Result};

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

    let start = input.iter().take(size_i);
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); size];
    for line in start {
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
    dbg!(&stacks);

    let instructions = input.iter().skip(size_i + 2).map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });
    // println!("{:?}", instructions.collect::<Vec<_>>());
    // !FIXME
    for step in instructions {
        for _ in 0..step[0] {
            let item = stacks[step[1] - 1].pop().unwrap();
            stacks[step[2] - 1].push(item);
        }
    }
    dbg!(&stacks);

    Ok(())
}
