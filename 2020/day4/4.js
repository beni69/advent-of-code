const fs = require('fs');
const input = fs.readFileSync('./in.txt').toString().trim().split('\n\n');

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

                const value = parseInt(item[j].substring(4, item[j].length - 2));
                if (item[j].endsWith('in') && value >= 59 && value <= 76) {
                    pass.hgt = true;

                } else if (item[j].endsWith('cm') && value >= 150 && value <= 193) {
                    pass.hgt = true;

                }
            } else if (item[j].startsWith('byr:')) {
                const value = parseInt(item[j].substring(4));
                if (value >= 1920 && value <= 2002) {
                    pass.byr = true;
                }

            } else if (item[j].startsWith('iyr:')) {
                const value = parseInt(item[j].substring(4));
                if (value >= 2010 && value <= 2020) {
                    pass.iyr = true;
                }

            } else if (item[j].startsWith('eyr:')) {
                const value = parseInt(item[j].substring(4));
                if (value >= 2020 && value <= 2030) {
                    pass.eyr = true;
                }

            } else if (item[j].startsWith('hcl:')) {
                const value = item[j].substring(4);
                if (/^#[0-9A-F]{6}$/i.test(value)) {
                    pass.hcl = true;
                }

            } else if (item[j].startsWith('ecl:')) {
                const value = item[j].substring(4);
                if (/^amb|blu|brn|gry|grn|hzl|oth$/i.test(value)) {
                    pass.ecl = true;
                }

            } else if (item[j].startsWith('pid:')) {
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

fs.writeFileSync('output.txt', `Part one: ${P1()}`);
fs.appendFileSync('output.txt', `\nPart two: ${P2()}`);
console.log('Done!');
