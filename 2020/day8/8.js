const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n').map(x => { return { cmd: x.split(' ')[0], args: parseInt(x.split(' ')[1]), done: false }; });
// console.log(input);

function P1() {
    let acc = 0;
    let i = 0;
    while (true) {
        const item = input[i];
        if (item.done) break;
        item.done = true;
        i++;
        switch (item.cmd) {
            case 'acc':
                acc += item.args;
                break;

            case 'jmp':
                i += item.args - 1;
                break;

            case 'nop':
                break;
        }
    }
    return acc;
}

function P2() {

}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
