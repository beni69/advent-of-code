use std::error::Error;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string("./test")?;
    let inp: Vec<_> = file.trim().split("\n").collect();
    // println!("{inp:#?}");

    let mut sol = 0u32;
    for line in inp {
        let mut i = 0usize;
        let mut c1: Option<char> = None;
        let mut c2: Option<char> = None;
        while i < line.len() {
            let c = line.chars().skip(i).next().unwrap();

            if c.is_numeric() {
                c2 = Some(c);
                if c1.is_none() {
                    c1 = c2.clone();
                }
            } else {
                for (j, d) in DIGITS.iter().enumerate() {
                    if d.len() + i > line.len() {
                        continue;
                    }
                    // dbg!(&line[i..i + d.len()], d);
                    if **d == line[i..i + d.len()] {
                        dbg!(d);
                        c2 = Some((j + 1).to_string().chars().next().unwrap());
                        if c1.is_none() {
                            c1 = c2;
                        }
                        i += d.len() - 1;
                    }
                }
            }

            i += 1;
        }
        dbg!(&c1, &c2);
        match (c1, c2) {
            (Some(c1), Some(c2)) => sol += format!("{c1}{c2}").parse::<u32>().unwrap(),
            _ => unreachable!(),
        }
    }

    println!("{sol}");

    Ok(())
}
