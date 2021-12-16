use aoc::get_input_string;

fn main() {
    let input = get_input_string()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap() as u8))
        .fold(String::new(), |acc, x| acc + &x);
    let mut chars = input.chars();
    let len = input.len();
    let mut i = 0usize;
    let mut versions = 0u16;
    loop {
        if i >= len {
            break;
        }
        let mut ver = String::new();
        ver.push(chars.next().unwrap());
        ver.push(chars.next().unwrap());
        ver.push(chars.next().unwrap());
        i += 3;
        dbg!(&ver);
        versions += u16::from_str_radix(&ver, 2).unwrap();
        dbg!(&versions);

        break;
    }
}
