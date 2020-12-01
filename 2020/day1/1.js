const fs = require('fs');

// part one
function P1() {
    const input = fs.readFileSync('./in1.txt').toString().split('\r\n').map(x => parseInt(x));
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            if (input[i] + input[j] === 2020) return input[i] * input[j];
        }
    }
}

console.log(P1());
fs.writeFileSync('output.txt', `Part one: ${P1()}`)

// part two
function P2() {
    const input = fs.readFileSync('./in2.txt').toString().split('\r\n').map(x => parseInt(x));

    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            for (var k = 0; k < input.length; k++) {
                if (input[i] + input[j] + input[k] === 2020) return input[i] * input[j] * input[k];
            }
        }
    }
}

console.log(P2());
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
