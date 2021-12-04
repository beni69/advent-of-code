import { readFileSync } from "fs";

type Board = BoardNum[][];
interface BoardNum {
    value: number;
    picked: boolean;
}

const getWinningBoards = (boards: Board[]) => {
    const winning = Array(boards.length).fill(false);
    for (let k = 0; k < boards.length; k++) {
        const b = boards[k];
        // decide if a full line is picked
        for (let i = 0; i < b.length; i++) {
            const line = b[i];
            let allPicked = true;
            for (let j = 0; j < line.length; j++) {
                const item = line[j];
                !item.picked && (allPicked = false);
            }
            allPicked && (winning[k] = true);
        }

        // decide if a full column is picked
        for (let j = 0; j < b[0].length; j++) {
            let allPicked = true;
            const column = b.map(line => line[j]);
            for (let i = 0; i < column.length; i++) {
                const item = column[i];
                !item.picked && (allPicked = false);
            }
            allPicked && (winning[k] = true);
        }
    }
    return winning;
};

const pick = (boards: Board[], num: number) => {
    for (const b of boards) {
        for (let i = 0; i < b.length; i++) {
            const line = b[i];
            for (let j = 0; j < line.length; j++) {
                const item = line[j];
                if (item.value === num) {
                    item.picked = true;
                }
            }
        }
    }
};

const sumAllUnpicked = (b: Board) => {
    let sum = 0;
    for (const line of b) {
        for (const num of line) {
            num.picked || (sum += num.value);
        }
    }
    return sum;
};

const parseBoards = (input: string[]) => {
    let tmp: Board = [];
    const boards: Board[] = [];
    for (const line of input) {
        if (!line) {
            if (tmp?.length) boards.push(tmp);
            tmp = [];
            continue;
        }
        tmp.push(
            line
                .trim()
                .split(/\s+/)
                .map(n => ({ value: Number(n), picked: false }))
        );
    }
    boards.push(tmp);
    return boards;
};

const input = readFileSync("in.txt").toString().trim().split("\n"),
    nums = input.shift()!.split(",").map(Number);

const part1 = () => {
    const boards = parseBoards(input);
    for (const n of nums) {
        pick(boards, n);
        const winner = getWinningBoards(boards).indexOf(true);
        if (winner >= 0) return sumAllUnpicked(boards[winner]) * n;
    }
    return null;
};

const part2 = () => {
    const boards = parseBoards(input);
    let lastW = Array(boards.length).fill(false);
    for (const n of nums) {
        pick(boards, n);
        const winners = getWinningBoards(boards);
        if (winners.indexOf(false) >= 0) {
            lastW = winners;
            continue;
        }
        const loser = lastW.indexOf(false);
        if (loser === -1) throw new Error("no loser");
        return sumAllUnpicked(boards[loser]) * n;
    }
};

console.log(part1());
console.log(part2());
