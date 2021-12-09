use aoc::get_input_lines;
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
    //dbg!(&matrix);
    let low_points: Vec<i32> = Vec::new();

    for _i in 0..matrix.len() {
        let i = _i as i32;
        //println!("line {}", i);
        let line = &matrix[_i];
        for _j in 0..line.len() {
            let j = _j as i32;
            let n = &matrix[_i][_j];
            println!("{}.{}: {}", &i, &j, &n);
            //TODO: check num
            if exists(&matrix, i - 1, j) && &matrix[_i][_j] > n {
                panic!("one above is bigger");
            }
        }
    }
}

// TODO: get an item at the specified index. silently fail if not found
fn get(matrix: &Vec<Vec<i32>>, i: i32, j: i32) -> Option<&i32> {
    if i < 0 || i >= matrix.len() as i32 || j < 0 {
        return None;
    }
    let line = &matrix[i as usize];
    if j >= line.len() as i32 {
        return None;
    }
    Some(&line[j as usize])
}
fn exists(matrix: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
    i >= 0 && j >= 0 && i < matrix.len() as i32 && j < matrix[0].len() as i32
}
