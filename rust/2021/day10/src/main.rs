use aoc::get_input_lines;
fn main() {
    let input = get_input_lines();
    let open = ['(', '[', '{', '<'];
    let close = [')', ']', '}', '>'];
    let mut score_1 = 0;
    let mut scores: Vec<u64> = Vec::new();
    for i in 0..input.len() {
        let line = &input[i];
        // contains the open/close indexes
        let mut stack: Vec<usize> = Vec::new();
        let mut corrupted = false;
        let chars: Vec<char> = line.chars().collect();
        for j in 0..line.len() {
            let c = chars[j];
            if open.contains(&c) {
                stack.push(open.iter().position(|x| x == &c).unwrap());
            } else if close.contains(&c) {
                if stack.is_empty() {
                    panic!("stack empty at {:?}", j);
                }
                let last = stack.pop().unwrap();
                let correct = close[last];

                if c != correct {
                    corrupted = true;
                    score_1 += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    println!("expected \"{}\" but found \"{}\"", &close[last], &c);
                }
            }
            // dbg!(&stack);
        }
        // part 2
        if !(stack.is_empty() || corrupted) {
            println!("incomplete line at {:?}", &i);
            let mut score: u64 = 0;
            for j in (0..stack.len()).rev() {
                let c = close[stack[j]];
                score *= 5;
                score += stack[j] as u64 + 1;
            }
            scores.push(score);
        }
    }
    scores.sort();
    let mid = (scores.len() as f64 / 2 as f64).floor() as usize;
    let score_2 = scores[mid];
    println!("part 1: {}", &score_1);
    println!("part 2: {}", &score_2);
}
