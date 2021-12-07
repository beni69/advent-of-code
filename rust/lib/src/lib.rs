use std::{env::args, fs::canonicalize, path::Path, str::FromStr};

pub fn get_input_string() -> String {
    let args = args().collect::<Vec<String>>();
    let p = if args.len() > 1 && Path::new(&args[1]).exists() {
        canonicalize(&args[1]).expect("Could not find file")
    } else {
        canonicalize(Path::new("in.txt")).expect("in.txt not found")
    };

    println!("reading input from {:?}", &p);

    let f = std::fs::read_to_string(p).expect("failed to read input file");
    f.trim().to_string()
}

pub fn get_input_lines() -> Vec<String> {
    let f = get_input_string();

    let mut lines: Vec<String> = Vec::new();

    for line in f.lines() {
        lines.push(line.to_string());
    }

    lines
}

pub fn get_input_lines_int() -> Vec<i32> {
    let lines = get_input_lines();
    let mut nums: Vec<i32> = Vec::new();
    for line in lines {
        nums.push(line.parse::<i32>().unwrap());
    }
    nums
}

pub fn get_input_split<T: FromStr>(split: &str) -> Vec<T> {
    let f = get_input_string();
    let mut v: Vec<T> = Vec::new();
    for s in f.split(split) {
        let res = s.parse::<T>();
        if res.is_err() {
            panic!("Failed to parse {}", s);
        }
        v.push(res.ok().unwrap());
    }
    v
}
