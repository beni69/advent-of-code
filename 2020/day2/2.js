const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().split('\r\n').map(x => parseInt(x)).filter(x => !isNaN(x));

console.log(input);
