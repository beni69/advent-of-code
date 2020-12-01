const fs = require('fs');
const in1 = fs.readFileSync('./in1.txt').toString().split('\r\n').map(x => parseInt(x));

function P1Out(input) {
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            if (input[i] + input[j] === 2020) return input[i] * input[j];
        }
    }
}

console.log(P1Out(in1));

const in2 = fs.readFileSync('./in2.txt').toString().split('\r\n').map(x => parseInt(x));

function P2Out(input) {
    for (var i = 0; i < input.length; i++) {
        for (var j = 0; j < input.length; j++) {
            for (var k = 0; k < input.length; k++) {
                if (input[i] + input[j] + input[k] === 2020) return input[i] * input[j] * input[k];
            }
        }
    }
}

console.log(P2Out(in2));
