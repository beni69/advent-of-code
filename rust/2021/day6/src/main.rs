use aoc::get_input_string;

type Fish = [u128; 9];

fn main() {
    let input_string = get_input_string();
    let mut input = Vec::new();
    for num in input_string.split(",") {
        input.push(num.parse::<u128>().unwrap());
    }
    let input: Fish = transform(input);
    println!("input: {:?}", &input);
    println!("part 1: {}", simulate_n_days(&input, 80));
    println!("part 2: {}", simulate_n_days(&input, 256));
}

fn transform(nums: Vec<u128>) -> Fish {
    let mut x = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for n in nums {
        x[n as usize] += 1;
    }
    x
}

fn simulate_day(fish: &mut Fish) {
    let new = fish[0];
    for i in 0..fish.len() - 1 {
        fish[i] = fish[i + 1];
    }
    fish[8] = new;
    fish[6] += new;
}

fn count_fish(fish: &Fish) -> u128 {
    fish.iter().sum()
}

fn simulate_n_days(nums: &Fish, n: i32) -> u128 {
    let mut fish = nums.clone();
    for _ in 0..n {
        simulate_day(&mut fish);
    }
    count_fish(&fish)
}

// shortened version
fn _main_short() {
    let in_str = get_input_string();
    let mut fish: Fish = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for s in in_str.split(",") {
        fish[s.parse::<usize>().expect("invalid input file")] += 1;
    }
    for _ in 0..256 {
        let new = fish[0];
        for i in 0..fish.len() - 1 {
            fish[i] = fish[i + 1];
        }
        fish[8] = new;
        fish[6] += new;
    }
    let sum: u128 = fish.iter().sum();
    println!("{}", sum);
}
