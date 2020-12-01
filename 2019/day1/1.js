const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));

// console.log(input);
let output = 0;

input.forEach((item, i) => {
    output+= Math.floor(item / 3) - 2;
});

console.log(output);
