use std::collections::HashSet;

use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let moves = input
        .iter()
        .map(|l| {
            let mut s = l.split_ascii_whitespace();
            (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(t);

    pretty_print(h, t);
    for (m, n) in moves.iter() {
        for _ in 0..*n {
            match *m {
                "U" => h.0 += 1,
                "D" => h.0 -= 1,
                "R" => h.1 += 1,
                "L" => h.1 -= 1,
                _ => unreachable!("invalid move"),
            }
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                // tail too far, needs to move
                if h.0 > t.0 {
                    t.0 += 1
                } else if h.0 < t.0 {
                    t.0 -= 1
                }
                if h.1 > t.1 {
                    t.1 += 1
                } else if h.1 < t.1 {
                    t.1 -= 1
                }

                visited.insert(t);
            }
            // pretty_print(h, t);
        }
    }

    println!("{}", visited.len());

    Ok(())
}

fn pretty_print(h: (i32, i32), t: (i32, i32)) {
    let mut m = vec![vec!['.'; 6]; 5];
    m[0][0] = 's';
    m[t.0 as usize][t.1 as usize] = 'T';
    m[h.0 as usize][h.1 as usize] = 'H';
    let res = m
        .iter()
        .rev()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    eprintln!("==================\n{res}\n");
}
