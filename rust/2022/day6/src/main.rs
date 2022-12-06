use lib::{input::get_input, year_day, Result};

fn main() -> Result<()> {
    let input = get_input(year_day!())?;

    let r1 = find(&input, 4);
    println!("part 1: {}, ({})", r1.0, r1.1);

    let r2 = find(&input, 14);
    println!("part 2: {}, ({})", r2.0, r2.1);

    Ok(())
}
fn find<'a>(input: &'a str, n: usize) -> (usize, &'a str) {
    for i in 0..input.len() - n {
        let slice = &input[i..i + n];
        let mut chars = slice.chars().collect::<Vec<_>>();
        chars.sort_unstable();
        chars.dedup();
        if chars.len() == n {
            return (i + n, slice);
        }
    }
    unreachable!("failed to find")
}
