use aoc::{
    get_input_2d,
    print::{pretty_print_2d, print_2d},
};
fn main() {
    let input = get_input_2d::<u8>();
    print_2d(&input);
    simulate(input.clone(), 100);
}
fn simulate(mut matrix: Vec<Vec<u8>>, steps: u16) {
    let matrix_size = matrix.len() * matrix[0].len();
    let mut flashes = 0;
    let mut all_flashed = -1;
    let mut step: u16 = 0;
    loop {
        if step >= steps && all_flashed != -1 {
            break;
        }
        // first give everyone +1 energy
        for line in matrix.iter_mut() {
            for n in line {
                *n += 1;
            }
        }
        // calculate flashes
        let mut flashed: Vec<(usize, usize)> = Vec::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let n = matrix[i][j];
                // octopus flashes
                if n > 9 && !flashed.contains(&(i, j)) {
                    flash(&mut matrix, &mut flashed, (i as i32, j as i32));
                }
            }
        }
        // reset flashed octopus' energy
        for (i, j) in flashed.iter() {
            matrix[*i][*j] = 0;
        }
        flashes += flashed.len();
        // check if every octopus flashed
        if flashed.len() == matrix_size {
            all_flashed = step as i32 + 1;
        }
        pretty_print_2d(&matrix, |n| *n == 0);
        step += 1;
    }
    println!("part 1: {}", &flashes);
    println!("part 2: {}", &all_flashed);

    fn flash(matrix: &mut Vec<Vec<u8>>, flashed: &mut Vec<(usize, usize)>, (i, j): (i32, i32)) {
        flashed.push((i as usize, j as usize));
        let directions: [(i32, i32); 8] = [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];
        for (ii, jj) in directions {
            if ii < 0 || ii >= matrix.len() as i32 || jj < 0 || jj >= matrix[0].len() as i32 {
                continue;
            }
            matrix[ii as usize][jj as usize] += 1;
            if !flashed.contains(&(ii as usize, jj as usize))
                && matrix[ii as usize][jj as usize] > 9
            {
                flash(matrix, flashed, (ii, jj));
            }
        }
    }
}
