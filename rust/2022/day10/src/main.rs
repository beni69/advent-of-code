use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut cmds = input.iter();

    let mut x = 1;
    let mut done = -1;
    let mut cmd = Vec::new();
    let mut res = Vec::new();

    for i in 1..221 {
        // before cycle
        if done == -1 {
            // previous instruction finished, start a new one
            cmd = cmds
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            match cmd[0] {
                "noop" => done = i,
                "addx" => done = i + 1,
                _ => unreachable!("invalid instruction"),
            }
        }

        // during cycle
        if (i - 20) % 40 == 0 {
            res.push(x * i);
            eprintln!("{i}: {x} - {}", x * i);
        }

        // after cycle
        if done == i {
            match cmd[0] {
                "noop" => (),
                "addx" => x += cmd[1].parse::<i32>().unwrap(),
                _ => unreachable!("invalid instruction"),
            }
            done = -1;
        }
    }

    println!("part 1: {}", res.iter().sum::<i32>());

    Ok(())
}
