use aoc::get_input_string;
fn main() {
    let in_str = get_input_string();
    let mut input: Vec<i32> = Vec::new();
    for s in in_str.split(",") {
        input.push(s.parse::<i32>().expect("invalid input file"));
    }
    println!("{:?}", &input);
}
