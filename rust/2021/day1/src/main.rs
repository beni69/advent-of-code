use aoc::get_input_lines_int;

fn main() {
    let input = get_input_lines_int();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(nums: &Vec<i32>) -> i32 {
    let mut sol = 0;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            sol += 1;
        }
    }
    sol
}

fn part2(nums: &Vec<i32>) -> i32 {
    let mut sol = 0;
    for i in 0..nums.len() - 3 {
        if nums[i + 3] > nums[i] {
            sol += 1;
        }
    }
    sol
}
