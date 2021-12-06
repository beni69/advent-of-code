import { readFileSync } from "fs";

const simulateDay = (fish: BigUint64Array) => {
    const length = fish.length;
    let newFish = new BigUint64Array(length);
    for (let i = 0; i < length; i++) {
        if (fish[i] > 0) newFish[i] = fish[i] - 1n;
        else {
            newFish[i] = 6n;
            // newFish[newFish.length] = BigInt(8);
            newFish = BigUint64Array.from([...newFish, 8n]);
        }
    }
    // console.log({ newFish });

    return newFish;
};

const input = readFileSync("in.txt").toString().trim().split(",").map(Number);

const part1 = () => {
    // const fish = [...input];
    // for (let i = 0; i < 80; i++) {
    //     simulateDay(fish);
    // }
    // return fish.length;
};

const part2 = () => {
    let fish = input.map(BigInt),
        bigfish = BigUint64Array.from(fish);

    for (let i = 0; i < 80; i++) {
        console.log(i, bigfish);

        bigfish = simulateDay(bigfish);
    }
    return bigfish.length;
};

// console.log(part1());
console.log(part2());

const x = new BigUint64Array();
console.log(x);
x[0] = 1n;
console.log(x);
