import { readFileSync } from "fs";

const input = readFileSync("in.txt")
    .toString()
    .trim()
    .split("\n")
    .map(x => [x.split(" ")[0], parseInt(x.split(" ")[1])]) as [
    string,
    number
][];

const part1 = () => {
    let x = 0,
        y = 0;
    input.map(inp => {
        switch (inp[0]) {
            case "forward":
                x += inp[1];
                break;
            case "up":
                y -= inp[1];
                break;
            case "down":
                y += inp[1];
                break;
        }
    });
    console.log({ x, y });
    return x * y;
};

const part2 = () => {
    let x = 0,
        y = 0,
        aim = 0;
    input.map(inp => {
        switch (inp[0]) {
            case "forward":
                x += inp[1];
                y += inp[1] * aim;
                break;
            case "up":
                aim -= inp[1];
                break;
            case "down":
                aim += inp[1];
                break;
        }
    });
    console.log({ x, y, aim });
    return x * y;
};

console.log(part1());
console.log(part2());
