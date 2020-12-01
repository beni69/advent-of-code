const fs = require('fs');

// part one
function P1() {
    const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            if (input[i] + input[j] === 2020) return input[i] * input[j];
        }
    }
}

// part two
function P2() {
    const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));

    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            for (var k = 0; k < input.length; k++) {
                if (input[i] + input[j] + input[k] === 2020) return input[i] * input[j] * input[k];
            }
        }
    }
}

console.log(P1());
console.log(P2());
fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
