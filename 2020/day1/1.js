const fs = require('fs');
const array = fs.readFileSync('./in1.txt').toString().split('\r\n').map(x => parseInt(x));

function GetOutput() {
    for (var i = 0; i < array.length; i++) {
        for (var j = 0; j < array.length; j++) {
            if (array[i] + array[j] === 2020) return array[i] * array[j];
        }
    }
}

console.log(GetOutput());
