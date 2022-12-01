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

pub fn get_input_2d<T: FromStr>() -> Vec<Vec<T>> {
    let f = get_input_string();
    let mut v: Vec<Vec<T>> = Vec::new();
    for line in f.lines() {
        let mut vv: Vec<T> = Vec::new();
        for s in line.trim().chars() {
            let res = s.to_string().parse::<T>();
            if res.is_err() {
                panic!("Failed to parse {}", s);
            }
            vv.push(res.ok().unwrap())
        }
        v.push(vv);
    }
    v
}

pub mod print {
    use std::fmt::Display;

    pub fn print_2d<T>(matrix: &Vec<Vec<T>>)
    where
        T: Display,
    {
        let mut s = String::new();
        for line in matrix {
            for item in line {
                s.push_str(&item.to_string())
            }
            s.push('\n');
        }
        println!("{}", &s);
    }
    pub fn pretty_print_2d<T, F>(matrix: &Vec<Vec<T>>, check: F)
    where
        T: Display,
        F: Fn(&T) -> bool,
    {
        let mut s = String::new();
        for line in matrix {
            for item in line {
                if check(item) {
                    s.push_str(&format!("\x1b[1;36m{}\x1b[0m", item));
                } else {
                    s.push_str(&item.to_string())
                }
            }
            s.push('\n');
        }
        println!("{}", &s);
    }
}
