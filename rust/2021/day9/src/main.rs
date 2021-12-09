use aoc::get_input_lines;
use std::collections::HashSet;
fn main() {
    let input = get_input_lines();
    let matrix: Vec<Vec<i32>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    // part 1
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    let mut risk_levels: Vec<i32> = Vec::new();
    for _i in 0..matrix.len() {
        let i = _i as i32;
        let line = &matrix[_i];
        for _j in 0..line.len() {
            let j = _j as i32;
            let n = &matrix[_i][_j];
            if (!exists(&matrix, i - 1, j) || &matrix[_i - 1][_j] > n)
                && (!exists(&matrix, i + 1, j) || &matrix[_i + 1][_j] > n)
                && (!exists(&matrix, i, j - 1) || &matrix[_i][_j - 1] > n)
                && (!exists(&matrix, i, j + 1) || &matrix[_i][_j + 1] > n)
            {
                low_points.push((_i, _j));
                risk_levels.push(n + 1);
            }
        }
    }
    println!("part 1: {}", risk_levels.iter().sum::<i32>());
    // part 2
    let mut basins: Vec<Vec<(usize, usize)>> = Vec::new();
    for (i, j) in low_points {
        let basin = check_around(&matrix, i, j, true);
        basins.push(basin);
    }
    basins.iter().for_each(|b| _pretty_print_basin(&matrix, b));
    // sort basins by size
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let biggest3 = &basins[0..3]
        .iter()
        .map(|b| b.len())
        .fold(1, |acc, b| acc * b);
    println!("part 2: {}", biggest3);
}
fn exists(matrix: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    i >= 0 && j >= 0 && i < matrix.len() as i32 && j < matrix[0].len() as i32
}
fn check_around(matrix: &Vec<Vec<i32>>, i: usize, j: usize, dedup: bool) -> Vec<(usize, usize)> {
    let n = matrix[i][j];
    if n == 9 {
        return Vec::new();
    }
    let mut basin: Vec<(usize, usize)> = Vec::new();
    basin.push((i, j));
    // check all four directions
    if exists(matrix, i as i32 - 1, j as i32)
        && matrix[i - 1][j] > n
        && !basin.contains(&(i - 1, j))
    {
        basin.append(&mut check_around(matrix, i - 1, j, false));
    }
    if exists(matrix, i as i32 + 1, j as i32)
        && matrix[i + 1][j] > n
        && !basin.contains(&(i + 1, j))
    {
        basin.append(&mut check_around(matrix, i + 1, j, false));
    }
    if exists(matrix, i as i32, j as i32 - 1)
        && matrix[i][j - 1] > n
        && !basin.contains(&(i, j - 1))
    {
        basin.append(&mut check_around(matrix, i, j - 1, false));
    }
    if exists(matrix, i as i32, j as i32 + 1)
        && matrix[i][j + 1] > n
        && !basin.contains(&(i, j + 1))
    {
        basin.append(&mut check_around(matrix, i, j + 1, false));
    }
    // remove duplicates
    if dedup {
        let set: HashSet<(usize, usize)> = basin.drain(..).collect();
        basin.extend(set.into_iter());
    }
    basin
}
fn _pretty_print_basin(matrix: &Vec<Vec<i32>>, basin: &Vec<(usize, usize)>) {
    let mut s = String::new();
    for i in 0..matrix.len() {
        let line = &matrix[i];
        for j in 0..line.len() {
            let n = &line[j];
            if basin.contains(&(i, j)) {
                s.push_str(&format!("\x1b[1;36m{}\x1b[0m", n));
            } else {
                s.push_str(&n.to_string());
            }
        }
        s.push_str("\n");
    }
    println!("{}", s);
}
