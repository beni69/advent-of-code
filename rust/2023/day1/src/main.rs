use lib::{input::get_input, year_day, Result};
const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn check(line: &str, i: usize) -> Option<u32> {
    let c = line.chars().skip(i).next().unwrap();
    if c.is_numeric() {
        return c.to_digit(10);
    }
    for (j, d) in DIGITS.iter().enumerate() {
        if d.len() + i > line.len() {
            continue;
        }
        if **d == line[i..i + d.len()] {
            return Some(j as u32 + 1);
        }
    }
    None
}
fn main() -> Result<()> {
    let inp = get_input(year_day!())?;

    let mut sol = 0u32;
    for line in inp.trim().split("\n") {
        let mut i = 0;
        let mut c1: Option<u32> = None;
        while i < line.len() && c1.is_none() {
            c1 = check(line, i);
            i += 1;
        }
        i = line.len() - 1;
        let mut c2: Option<u32> = None;
        while c2.is_none() {
            c2 = check(line, i);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        if let (Some(c1), Some(c2)) = (c1, c2) {
            sol += c1 * 10 + c2;
        } else {
            unreachable!()
        }
    }
    println!("{sol}");
    Ok(())
}
