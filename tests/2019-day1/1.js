const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));

// part one
function P1() {
    let output = 0;
    input.forEach((item, i) => {
        output += Math.floor(item / 3) - 2;
    });
    return output;
}

console.log(P1());
fs.writeFileSync('output.txt', `Part one: ${P1()}`);

// part two
function P2() {
    let output = 0;

    function Calculate(item) {
        let result = Math.floor(item / 3) - 2;
        output += result;
        if (Math.floor(result / 3) - 2 > 0) {
            Calculate(result);
        }
    }

    input.forEach((item, i) => {
        Calculate(item);
    });

    return output;
}

console.log(P2());
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
