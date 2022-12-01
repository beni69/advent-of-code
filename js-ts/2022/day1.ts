export const parse = (input: string) =>
    input
        .trim()
        .split("\n\n")
        .map(l => l.split("\n").map(x => parseInt(x)))
        .map(x => x.reduce((a, b) => a + b, 0));

export const p1 = (input: number[]) =>
    input.reduce((a, b) => Math.max(a, b), 0);

export const p2 = (input: number[]) =>
    input
        .sort((a, b) => b - a)
        .slice(0, 3)
        .reduce((a, b) => a + b, 0);
