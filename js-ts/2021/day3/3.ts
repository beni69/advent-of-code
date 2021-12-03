import { readFileSync } from "fs";

const input = readFileSync("ex.txt")
    .toString()
    .trim()
    .split("\n")
    .map(x => x.split(""));

const countNums = () => {
    let num0 = Array(input[0].length).fill(0) as number[];
    input.forEach(x => x.forEach((y, i) => y === "0" && num0[i]++));
    return num0;
};

const part1 = () => {
    let gamma = "";
    const num0 = countNums();
    input[0].forEach((x, i) =>
        num0[i] > input.length / 2 ? (gamma += "0") : (gamma += "1")
    );
    const flipNum = (n: number) => Number(!n);
    const epsilon = gamma
        .split("")
        .map(x => flipNum(parseInt(x)))
        .join("");

    const gammaDec = parseInt(gamma, 2),
        epsilonDec = parseInt(epsilon, 2);

    return gammaDec * epsilonDec;
};

const part2 = () => {
    const num0 = countNums();
    // const first = input.map(x => x[0]);
    const o2Char = num0[0] > input.length / 2 ? "1" : "0";
    // const one = input.filter(x => x[0] === "1"),
    //     zero = input.filter(x => x[0] === "0");
    let nums = input;
    input[0].forEach(
        (_, i) => (nums = nums.filter(x => x[0] === `${num0[i]}`))
    );
    // TODO: stop when only one number remains
    console.log(nums);
};

// console.log(input);
console.log(part1());
console.log(part2());
