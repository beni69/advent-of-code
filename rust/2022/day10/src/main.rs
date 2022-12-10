use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut cmds = input.iter();

    let mut x = 1;
    let mut done = -1;
    let mut cmd = Vec::new();
    let mut res = Vec::new();
    let mut screen: Vec<char> = vec!['.'; 240];

    for i in 1..241 {
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
            // eprintln!("{i}: {x} - {}", x * i);
        }
        // part 2
        let y = (i - 1) % 40;
        if x >= y - 1 && x <= y + 1 {
            screen[(i - 1) as usize] = '#';
        }

        // after cycle
        if done == i {
            if cmd[0] == "addx" {
                x += cmd[1].parse::<i32>().unwrap();
            }
            done = -1;
        }
    }

    println!("part 1: {}", res.iter().sum::<i32>());

    println!("part 2:\n{}", render(&screen).trim());

    Ok(())
}
fn render(screen: &Vec<char>) -> String {
    let mut s = String::with_capacity(246);
    for (i, c) in screen.iter().enumerate() {
        s.push(*c);
        if (i + 1) % 40 == 0 {
            s.push('\n');
        }
    }
    assert_eq!(s.len(), 246);
    s
}
