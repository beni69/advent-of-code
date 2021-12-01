const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n');
const { performance } = require('perf_hooks');
const time = performance.now();


function P1() {
    let bags = [];
    input.forEach(item => {
        let rule = item.split(' contain ');
        bags.push({ bag: rule[0].replace(/ bags/i, ''), contains: rule[1].split(', ').map(x => x.replace(/[0-9]| bags| bag|\./gi, '').trim()) });
    });
    let sol = ['shiny gold'];
    for (let i = 0; i < bags.length; i++) {
        const contents = bags[i].contains.join();
        const reg1 = new RegExp(sol.join('|'), 'i');
        if (reg1.test(contents) && !sol.toString().includes(bags[i].bag)) {
            sol.push(bags[i].bag);
            i = -1;
        }
    }
    return sol.length - 1;
}

function P2() {
    let bags = [];
    input.forEach(item => {
        let rule = item.split(' contain ');
        bags.push({ bag: rule[0].replace(/ bags/i, ''), contains: rule[1].split(', ').map(x => x.replace(/ bags| bag|\./gi, '').trim()) });
    });
    let sol = { done: [], next: ['shiny gold'] };
    for (let i = 0; i < bags.length; i++) {
        const reg1 = new RegExp(sol.next.join('|'));

    }

    let sum = 0;
    amounts.forEach((item, i) => {
        sum += item.amount;
    });
    console.log(amou);
    return sum;
}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
console.log(`Runtime: ${Math.round(performance.now() - time)}ms`);
