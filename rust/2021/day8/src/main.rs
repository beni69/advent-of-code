use aoc::get_input_lines;
// 1: 2
// 4: 4
// 7: 3
// 8: 7
fn main() {
    let input = get_input_lines();
    let in_split = input
        .iter()
        .map(|l| {
            let split = l.split("|").collect::<Vec<&str>>();
            (split[0].trim(), split[1].trim())
        })
        .collect::<Vec<(&str, &str)>>();

    {
        let input = in_split
            .iter()
            .map(|x| x.1.to_string())
            .collect::<Vec<String>>();

        let mut sum = 0;
        for line in input.iter() {
            for s in line.split_ascii_whitespace() {
                match s.len() {
                    2 | 4 | 3 | 7 => sum += 1,
                    _ => {}
                }
            }
        }
        println!("part 1: {}", sum);
    }

    // WIP
    // let mut patterns=input.iter();
    // let mut nums = Vec::new();
    // for line in input.iter() {
    //     let mut digits = String::new();

    //     let one = input
    // }
    // println!("part 2: {}", nums.iter().sum::<i32>());
}
