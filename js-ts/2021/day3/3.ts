import { readFileSync } from "fs";

const input = readFileSync("ex.txt")
    .toString()
    .trim()
    .split("\n")
    .map(x => x.split(""));

const countNums = (input: string[][]) => {
    let num0 = Array(input[0].length).fill(0) as number[];
    input.forEach(x => x.forEach((y, i) => y === "0" && num0[i]++));
    return num0;
};

const part1 = () => {
    let gamma = "";
    const num0 = countNums(input);
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
    const num0 = countNums(input);
    console.log({ num0 });

    let nums = [...input];
    for (let i = 0; i < input.length; i++) {
        console.log({ nums, l: num0[i] > input.length / 2 ? "0" : "1" });

        if (nums.length == 1) break;
        nums = nums.filter(
            n => n[i] === `${num0[i] > input.length / 2 ? "0" : "1"}`
        );
    }
    // TODO: count most 0 or 1 from nums instead of input
    console.log(nums);
};

// console.log(input);
console.log(part1());
console.log(part2());
