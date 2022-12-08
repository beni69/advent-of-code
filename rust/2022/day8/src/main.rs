use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?
        .into_iter()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(i, _)| l[i..i + 1].parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut res1 = vec![vec![0; input.len()]; input.len()];
    for x in 0..input.len() {
        for y in 0..input.len() {
            // res1[x][y] = is_visible(&input, x, y);
            let v = input[x][y];
            let mut res = 4;
            for i in 0..y {
                if input[x][i] >= v {
                    res -= 1;
                    break;
                }
            }
            for i in y + 1..input.len() {
                if input[x][i] >= v {
                    res -= 1;
                    break;
                }
            }
            for i in 0..x {
                if input[i][y] >= v {
                    res -= 1;
                    break;
                }
            }
            for i in x + 1..input.len() {
                if input[i][y] >= v {
                    res -= 1;
                    break;
                }
            }
            res1[x][y] = res;
        }
    }
    println!(
        "part 1: {}",
        res1.iter()
            .map(|l| l.iter().filter(|n| **n > 0).count())
            .sum::<usize>()
    );

    let mut res2 = vec![vec![0; input.len()]; input.len()];
    for x in 0..input.len() {
        for y in 0..input.len() {
            // res2[x][y] = score(&input, x, y);
            let v = input[x][y];
            let mut res = 1;
            let mut tmp = 0;
            for i in (0..y).rev() {
                tmp += 1;
                if input[x][i] >= v {
                    break;
                }
            }
            res *= tmp;
            tmp = 0;
            for i in y + 1..input.len() {
                tmp += 1;
                if input[x][i] >= v {
                    break;
                }
            }
            res *= tmp;
            tmp = 0;
            for i in (0..x).rev() {
                tmp += 1;
                if input[i][y] >= v {
                    break;
                }
            }
            res *= tmp;
            tmp = 0;
            for i in x + 1..input.len() {
                tmp += 1;
                if input[i][y] >= v {
                    break;
                }
            }
            res2[x][y] = res * tmp;
        }
    }
    println!(
        "part 2: {}",
        res2.iter().map(|l| l.iter().max().unwrap()).max().unwrap()
    );

    Ok(())
}
