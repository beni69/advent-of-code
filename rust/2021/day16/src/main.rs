use aoc::get_input_string;
use std::{iter::Peekable, str::Chars};
fn main() {
    let input = get_input_string()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap() as u8))
        .fold(String::new(), |acc, x| acc + &x);
    let mut chars = input.chars().peekable();
    let len = input.len();
    let mut i = 0usize; // will keep track of our position
    let mut versions = 0u16;
    dbg!(&input, &len);

    let res = consume(&mut chars, &mut versions);
    i += res.0;
    let value = res.1;

    // consume remaining 0s
    let mut p = 0;
    loop {
        let next = chars.peek();
        if next.is_none() || next.unwrap() != &'0' {
            break;
        }
        chars.next();
        p += 1;
    }
    i += p;
    eprintln!("removed {} trailing 0s", &p);
    println!("part 1: {}", &versions);
    println!("part 2: {}", &value);
    assert_eq!(i, len); // safety check
}
fn consume(chars: &mut Peekable<Chars>, versions: &mut u16) -> (usize, u64) {
    let i = 6; // we will consume 6 bits

    // first 3 bits -> packet version
    let mut ver = String::new();
    ver.push(chars.next().unwrap());
    ver.push(chars.next().unwrap());
    ver.push(chars.next().unwrap());
    dbg!(&ver);
    *versions += u16::from_str_radix(&ver, 2).unwrap();
    // next 3 bits -> packet type
    let mut type_id = String::new();
    type_id.push(chars.next().unwrap());
    type_id.push(chars.next().unwrap());
    type_id.push(chars.next().unwrap());
    let type_id = u8::from_str_radix(&type_id, 2).unwrap();
    dbg!(&type_id);

    // next step depends on packet type
    let res = match type_id {
        // type 4: literal value
        4 => consume_literal(chars),
        // any other type: operator packet (has nested packets)
        _ => consume_operator(chars, versions, &type_id),
    };
    (i + res.0, res.1)
}
fn consume_literal(chars: &mut Peekable<Chars>) -> (usize, u64) {
    let mut i = 0;
    let mut value = String::new();
    // read 5 bits at a time (four bits for the value)
    loop {
        let x = chars.next().unwrap();
        eprintln!("reading 5 bits with a leading {}", &x);
        value.push(chars.next().unwrap());
        value.push(chars.next().unwrap());
        value.push(chars.next().unwrap());
        value.push(chars.next().unwrap());
        i += 5;
        if x == '0' {
            break;
        }
    }
    let value = u64::from_str_radix(&value, 2).unwrap();
    dbg!(&value);
    (i, value)
}
fn consume_operator(chars: &mut Peekable<Chars>, versions: &mut u16, type_id: &u8) -> (usize, u64) {
    let mut v: Vec<u64> = Vec::new();
    let mut i = 1; // we already consumed the first bit
    let len_type = chars.next().unwrap();
    dbg!(&len_type);
    // get all sub-packets
    match len_type {
        // next 15 bits -> length of nested packets (in bits)
        '0' => {
            let mut l = String::new();
            let mut j = 0;
            while j < 15 {
                l.push(chars.next().unwrap());
                j += 1;
            }
            i += 15; // this will always consume 15 bits
            let l = usize::from_str_radix(&l, 2).unwrap();
            eprintln!("length of nested packets: {} bits", &l);
            j = 0;
            while j < l {
                let res = consume(chars, versions);
                j += res.0;
                v.push(res.1);
            }
            i += l;
        }
        // next 11 bits -> length of nested packets (in packets)
        '1' => {
            let mut l = String::new();
            let mut j = 0;
            while j < 11 {
                l.push(chars.next().unwrap());
                j += 1;
            }
            i += 11; // this will always consume 11 bits
            let l = usize::from_str_radix(&l, 2).unwrap();
            eprintln!("length of nested packets: {} packets", &l);
            j = 0;
            while j < l {
                let res = consume(chars, versions);
                i += res.0;
                v.push(res.1);
                j += 1;
            }
        }
        _ => unreachable!(),
    }
    // do the math
    let value = match type_id {
        0 => v.iter().sum(),
        1 => v.iter().product(),
        2 => *v.iter().min().unwrap(),
        3 => *v.iter().max().unwrap(),
        5 => (v[0] > v[1]) as u64,
        6 => (v[0] < v[1]) as u64,
        7 => (v[0] == v[1]) as u64,
        _ => unreachable!(),
    };
    (i, value)
}
