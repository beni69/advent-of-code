use std::fs::canonicalize;
use std::path::Path;

pub fn get_input_string() -> String {
    let mut p = canonicalize(Path::new(".")).unwrap();
    p.push("in.txt");

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
