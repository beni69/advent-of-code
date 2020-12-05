const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n');

function P1(retID = false) {
    let IDsOut = [];
    let biggest = 0;
    let smallest = 1000;
    input.forEach((item, i) => {
        const ray = item.split('');
        let min = 0;
        let max = 128;
        // console.log(ray);

        for (let i = 0; i <= 7; i++) {

            if (ray[i] == 'F') {
                max = (max / 2) + (min / 2);

            } else if (ray[i] == 'B') {
                min = (max / 2) + (min / 2);

            }
        }
        const row = min;
        min = 0;
        max = 8;
        for (let i = 7; i < 10; i++) {

            if (ray[i] == 'L') {
                max = (max / 2) + (min / 2);

            } else if (ray[i] == 'R') {
                min = (max / 2) + (min / 2);

            }
        }
        const col = min;
        const ID = (row * 8) + col;
        biggest = ID > biggest ? ID : biggest;
        smallest = ID < smallest ? ID : smallest;
        // console.log(smallest);
        IDsOut.push(ID);
    });
    if (retID) {
        return { ids: IDsOut, biggest: biggest, smallest: smallest };
    } else {
        return biggest;
    }
}

function P2() {
    const { ids, biggest, smallest } = P1(true);
    let sol = [];

    for (let i = smallest; i < biggest; i++) {
        let found = false;
        ids.forEach((item, j) => {
            if (item == i) found = true
        });
        if (!found) sol.push(i);
    }
    return sol;
}

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
