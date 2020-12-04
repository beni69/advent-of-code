const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n\n');
// console.log(input);

function P1() {
    let valid = 0;
    input.forEach((item, i) => {
        if (item.includes('byr:') && item.includes('iyr:') && item.includes('eyr:') && item.includes('hgt:') && item.includes('hcl:') && item.includes('ecl:') && item.includes('pid:')) {
            valid++;
        }
    });
    return valid;
}

function P2() {
    input.forEach((item, i) => {
        input[i] = item.split(/\s/);
    });
    console.log(input);

    let valid = 0;

    input.forEach((item, i) => {
        let pass = {};
        for (var j = 0; j < item.length; j++) {
            if (item[j].startsWith('hgt:')) {
                // console.log('hgt detected');

                const value = parseInt(item[j].substring(4, item[j].length - 2));
                if (item[j].endsWith('in') && value >= 59 && value <= 76) {
                    pass.hgt = true;

                } else if (item[j].endsWith('cm') && value >= 150 && value <= 193) {
                    pass.hgt = true;

                }
            } else if (item[j].startsWith('byr:')) {
                // console.log('byr detected');
                const value = parseInt(item[j].substring(4));
                if (value >= 1920 && value <= 2002) {
                    pass.byr = true;
                }

            } else if (item[j].startsWith('iyr:')) {
                // console.log('iyr detected');
                const value = parseInt(item[j].substring(4));
                if (value >= 2010 && value <= 2020) {
                    pass.iyr = true;
                }

            } else if (item[j].startsWith('eyr:')) {
                // console.log('eyr detected');
                const value = parseInt(item[j].substring(4));
                if (value >= 2020 && value <= 2030) {
                    pass.eyr = true;
                }

            } else if (item[j].startsWith('hcl:')) {
                // console.log('hcl detected');
                const value = item[j].substring(4);
                if (/^#[0-9A-F]{6}$/i.test(value)) {
                    pass.hcl = true;
                }

            } else if (item[j].startsWith('ecl:')) {
                // console.log('ecl detected');
                const value = item[j].substring(4);
                if (/^amb|blu|brn|gry|grn|hzl|oth$/i.test(value)) {
                    pass.ecl = true;
                }

            } else if (item[j].startsWith('pid:')) {
                // console.log('pid detected');
                const value = item[j].substring(4);
                if (!isNaN(parseInt(value)) && value.toString().length == 9) {
                    pass.pid = true;
                }
            }
        }
        // console.log(pass);
        const {
            pid,
            hgt,
            ecl,
            iyr,
            eyr,
            byr,
            hcl
        } = pass;
        if (pid && hgt && ecl && iyr && eyr && byr && hcl) {
            valid++;
        }
    });
    return valid;
}

function Test(str) {

    // console.log(str.substring(4, str.length - 2));
    // if (str.endsWith('in') && parseInt(str.substring(4, str.length - 2)) >= 59 && parseInt(str.substring(4, str.length - 2)) <= 76) {
    //     console.log('inch correct!');
    // } else if (str.endsWith('cm') && parseInt(str.substring(4, str.length - 2)) >= 150 && parseInt(str.substring(4, str.length - 2)) <= 193) {
    //     console.log('cm correct!');
    // }
    // console.log(/^#[0-9A-F]{6}$/i.test(str));
    // const value = parseInt(str.substring(4));
    // console.log(!isNaN(parseInt(str)) && str.toString().length == 9);

}

// console.log(input);
// console.log(P1());
// console.log(P2());
// Test('pid:087499704');

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
