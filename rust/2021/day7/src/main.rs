use aoc::get_input_split;
fn main() {
    let input = get_input_split::<i32>(",");
    let mut sol1: Vec<i32> = Vec::new();
    let mut sol2: Vec<i32> = Vec::new();
    for i in 0..input.iter().max().unwrap() + 1 {
        let mut fuel1 = 0;
        let mut fuel2 = 0;
        for n in input.iter() {
            let f = (i - n).abs();
            fuel1 += f;
            fuel2 += f * (f + 1) / 2;
        }
        sol1.push(fuel1);
        sol2.push(fuel2);
    }
    println!("part 1: {}", sol1.iter().min().unwrap());
    println!("part 2: {}", sol2.iter().min().unwrap());
    main_short();
}

// same as main() but only using one `;`
fn main_short() {
    let input = get_input_split::<i32>(",");
    println!(
        "part (1, 2): {:?}",
        (0..input.iter().max().unwrap() + 1)
            .map(|i| input.iter().fold((0, 0) as (i32, i32), |acc, n| (
                acc.0 + (i - n).abs(),
                acc.1 + (i - n).abs() * ((i - n).abs() + 1) / 2
            )))
            .fold((i32::MAX, i32::MAX) as (i32, i32), |acc, n| (
                acc.0.min(n.0),
                acc.1.min(n.1)
            ))
    )
}
