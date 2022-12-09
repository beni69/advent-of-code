use lib::{input::get_input_lines, year_day, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let moves = input
        .iter()
        .map(|l| {
            let mut s = l.split_ascii_whitespace();
            (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut knots = [(0i32, 0i32); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for (m, n) in moves.iter() {
        for _ in 0..*n {
            match *m {
                "U" => knots[0].0 += 1,
                "D" => knots[0].0 -= 1,
                "R" => knots[0].1 += 1,
                "L" => knots[0].1 -= 1,
                _ => unreachable!("invalid move"),
            }
            for i in 0..knots.len() - 1 {
                let h = knots[i];
                let mut t = knots[i + 1];

                // tail too far, needs to move
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

                    if i == 8 {
                        visited.insert(t);
                    }

                    knots[i] = h;
                    knots[i + 1] = t;
                }
            }
        }
    }

    println!("{}", visited.len());

    Ok(())
}

// !WARNING: this is for experiment, it will panic when used with the real input, might fix it later
// fn pretty_print(knots: &[(i32, i32)]) {
//     let mut m = vec![vec!['.'; 6]; 5];
//     m[0][0] = 's';
//     for (i, (x, y)) in knots.into_iter().enumerate().rev() {
//         m[*x as usize][*y as usize] = if i == 0 {
//             'H'
//         } else {
//             char::from_digit(i as u32, 10).unwrap()
//         };
//     }

//     let res = m
//         .iter()
//         .rev()
//         .map(|l| l.iter().collect::<String>())
//         .collect::<Vec<_>>()
//         .join("\n");
//     eprintln!("==================\n{res}\n");
// }
