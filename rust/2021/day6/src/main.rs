use aoc::get_input_string;

type Fish = [u128; 9];

fn main() {
    let input_string = get_input_string();
    // println!("{:?}", &inp);
    let mut input = Vec::new();
    for num in input_string.split(",") {
        input.push(num.parse::<u128>().unwrap());
    }
    let input: Fish = transform(input);
    println!("{:?}", &input);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn transform(nums: Vec<u128>) -> Fish {
    let mut x = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for n in nums {
        x[n as usize] += 1;
    }
    x
}

fn simulate_day(fish: &mut Fish) {
    let tmp_fish = fish.clone();
    // let len = fish.len();
    // for i in 0..len {
    //     if fish[i] > 0 {
    //         fish[i] -= 1;
    //     } else {
    //         fish[i] = 6;
    //         // fish.push(8);
    //     }
    // }
    // for i in 0..fish.len() - 1 {
    // if i == 0 {
    //     fish[6] = fish[0];
    //     fish[8] = fish[0];
    //     fish[0] = 0;
    // }
    // }
    for i in (0..fish.len() - 1).rev() {
        dbg!(&i);
        if i == 6 || i == 8 {
            fish[i] = tmp_fish[0];
        } else {
            fish[i] = tmp_fish[i + 1];
        }
    }
    // fish[0] = 1000000;
}

fn count_fish(fish: &Fish) -> u128 {
    fish.iter().sum()
}

fn part1(nums: &Fish) -> u128 {
    let mut fish = nums.clone();
    for _ in 0..18 {
        simulate_day(&mut fish);
        dbg!(&fish);
    }

    count_fish(&fish)
}

fn part2(nums: &Fish) -> usize {
    let mut fish = nums.clone();
    // for _i in 0..256 {
    //     dbg!(_i);
    //     simulate_day(&mut fish);
    // }
    fish.len()
}
