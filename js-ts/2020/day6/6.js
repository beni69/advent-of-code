const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n\n');

const RemoveDupe = string => {
    var unique = '';
    for (var i = 0; i < string.length; i++) {
        if (unique.indexOf(string[i]) == -1) {
            unique += string[i];
        }
    }
    return unique;
};

function P1() {
    x = input.map(y => y = y.replace(/\s/g, ''));
    let sum = 0;
    x.forEach((item, i) => {
        sum += RemoveDupe(item).length;
    });
    return sum;
}

function P2() {
    x = input.map(y => y = y.split(/\s/g));

    const Count = arr => {
        let str = arr.join('');
        let out = str;
        for (let j = 0; j < str.length; j++) {
            if (str.match(new RegExp(str[j], 'gi')).length != arr.length) {
                out = out.replace(new RegExp(str[j], 'gi'), '');
            }
        }
        return out;
    };
    let sum = 0;
    x.forEach((item, i) => {
        sum += RemoveDupe(Count(item)).length;
    });
    return sum;
}


fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
