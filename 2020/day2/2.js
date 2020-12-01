const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));

console.log(input);

// part one
function P1() {

}

// part two
function P2() {

}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
