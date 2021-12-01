import { readFileSync } from "fs";
import { join } from "path";

const input = readFileSync("in.txt")
    .toString()
    .trim()
    .split("\n")
    .map(x => parseInt(x));

const part1 = () => {
    let sol = 0;
    for (let i = 1; i < input.length; i++) {
        if (input[i] > input[i - 1]) sol++;
    }
    return sol;
};

const part2 = () => {
    const letters: number[] = [];
    for (let i = 1; i < input.length - 1; i++) {
        letters.push(input[i - 1] + input[i] + input[i + 1]);
    }

    let sol = 0;
    for (let i = 1; i < letters.length; i++) {
        if (letters[i] > letters[i - 1]) sol++;
    }

    return sol;
};

console.log("part 1", part1());
console.log("part 2", part2());
