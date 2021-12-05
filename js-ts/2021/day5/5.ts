import { appendFileSync, readFileSync, writeFileSync } from "fs";

interface Line {
    x1: number;
    y1: number;
    x2: number;
    y2: number;
}
class Line {
    constructor(
        public x1: number,
        public y1: number,
        public x2: number,
        public y2: number
    ) {}
    public straight() {
        return this.x1 === this.x2 || this.y1 === this.y2;
    }
    public forEach(cb: (x: number, y: number) => any) {
        if (this.straight()) {
            const X = [Math.min(this.x1, this.x2), Math.max(this.x1, this.x2)];
            const Y = [Math.min(this.y1, this.y2), Math.max(this.y1, this.y2)];
            for (let x = X[0]; x <= X[1]; x++) {
                for (let y = Y[0]; y <= Y[1]; y++) {
                    cb(x, y);
                }
            }
        } else {
            const bx = this.x1 < this.x2,
                by = this.y1 < this.y2,
                nx = bx ? 1 : -1,
                ny = by ? 1 : -1,
                cx = bx
                    ? (x: number) => x <= this.x2
                    : (x: number) => x >= this.x2,
                cy = by
                    ? (y: number) => y <= this.y2
                    : (y: number) => y >= this.y2;
            let x = this.x1,
                y = this.y1;
            while (cx(x) && cy(y)) {
                cb(x, y);
                x += nx;
                y += ny;
            }
        }
    }
}
type Map = number[][];

const parseLines = (input: string[]): Line[] =>
    input.map(line => {
        const l = line
            .split(/\s|->|,/g)
            .filter(Boolean)
            .map(Number) as [number, number, number, number];
        return new Line(...l);
    });
const getMapSize = (lines: Line[]) =>
    lines.reduce<[number, number]>(
        ([maxX, maxY], line) => [
            Math.max(maxX, line.x1 + 1, line.x2 + 1),
            Math.max(maxY, line.y1 + 1, line.y2 + 1),
        ],
        [0, 0]
    );
const linesToMap = (lines: Line[], [sizeX, sizeY]: [number, number]): Map => {
    let out: number[][] = Array(sizeY)
        .fill(0)
        .map(_ => Array(sizeX).fill(0));

    for (const line of lines) {
        line.forEach((x, y) => {
            out[y][x]++;
        });
    }
    return out;
};
const mapToString = (map: Map) => {
    return map
        .map(row => row.join(""))
        .join("\n")
        .replace(/0/g, ".");
};

const input = readFileSync("in.txt").toString().trim().split("\n");
console.log(input);

const part1 = () => {
    // only worry about straight lines
    const lines = parseLines(input).filter(line => line.straight());
    const mapSize = getMapSize(lines);
    const map = linesToMap(lines, mapSize);
    writeFileSync("out.txt", mapToString(map));
    return map.reduce((sum, row) => sum + row.filter(x => x > 1).length, 0);
};

const part2 = () => {
    const lines = parseLines(input);
    const mapSize = getMapSize(lines);
    const map = linesToMap(lines, mapSize);
    appendFileSync("out.txt", Array(10).fill("\n").join("") + mapToString(map));
    return map.reduce((sum, row) => sum + row.filter(x => x > 1).length, 0);
};

console.log(part1());
console.log(part2());
