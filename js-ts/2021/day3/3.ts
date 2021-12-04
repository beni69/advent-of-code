import { readFileSync } from "fs";

const input = readFileSync("in.txt")
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
        if (nums.length == 1) break;

        const b = countNums(nums)[i] > nums.length / 2 ? "0" : "1";
        console.log({ nums, b });

        nums = nums.filter(n => n[i] === b);
    }
    const o2 = parseInt(nums[0].join(""), 2);
    nums = [...input];
    for (let i = 0; i < input.length; i++) {
        if (nums.length == 1) break;

        const b = countNums(nums)[i] <= nums.length / 2 ? "0" : "1";
        console.log({ nums, b });

        nums = nums.filter(n => n[i] === b);
    }
    const co2 = parseInt(nums[0].join(""), 2);
    console.log({ o2, co2 });
    return o2 * co2;
};

console.log(part1());
console.log(part2());
