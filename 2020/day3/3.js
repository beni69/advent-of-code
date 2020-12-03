const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\r\n');

function CountTrees(right, down) {

    let x = 0;
    let trees = 0;

    for (var y = 0; y < input.length; y += down) {
        if (input[y].charAt(x) == '#') trees++;
        x += right;
        if (x > input[y].length - 1) x -= input[y].length;
    }
    return trees;
}

function P1() {
    return CountTrees(3, 1);
}

function P2() {
    return CountTrees(1, 1) * CountTrees(3, 1) * CountTrees(5, 1) * CountTrees(7, 1) * CountTrees(1, 2);
}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
