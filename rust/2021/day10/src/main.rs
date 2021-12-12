use aoc::get_input_lines;
fn main() {
    let input = get_input_lines();
    let open = ["(", "[", "{", "<"];
    let close = [")", "]", "}", ">"];
    let stack: Vec<&str> = Vec::new();
    for line in input {
        //dbg!(line);
        let chars: Vec<char> = line.chars().collect();
        for i in 0..line.len() {}
    }
}
