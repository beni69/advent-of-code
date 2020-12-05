const fs = require('fs');
const arrayIn = fs.readFileSync('./in.txt').toString().trim().split('\r\n');


let arrayOut = [];
arrayIn.forEach((item, i) => {
    arrayOut.push({
        'pwd': item.substring(item.indexOf(': ') + 2),
        'char': item.substring(item.indexOf(': ') - 1, item.indexOf(': ')),
        'range': item.substring(0, item.indexOf(' ')).split('-')
    });
});

// part one
function P1() {

    let correct = 0;
    arrayOut.forEach((item, i) => {
        let num = item.pwd.split(item.char).length - 1;
        if (num >= item.range[0] && num <= item.range[1]) correct++;
    });

    return correct;
}

// part two
function P2() {
    let correct = 0;
    arrayOut.forEach((item, i) => {
        let num = 0;
        if (item.pwd.charAt(item.range[0] - 1) == item.char) num++;
        if (item.pwd.charAt(item.range[1] - 1) == item.char) num++;
        if (num == 1) correct++;
    });


    return correct;
}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);

