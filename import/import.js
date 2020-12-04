// WARNING: unstable, testing required
const fs = require('fs');
const fetch = require('node-fetch');
const config = require('../config');

let day;
if (!process.argv[2]) {
    console.error('No day number provided');
    console.warn('Usage: node <script name> <day> [path and name]');
    return;
} else if (isNaN(parseInt(process.argv[2])) || process.argv[2].length > 2) {
    console.error('Invalid day provided. Only enter the day number ex. \'5\'');
    console.warn('Usage: node <script name> <day> [path and name]');
    return;
} else {
    day = process.argv[2];
}

let pathArr;
let nameSpecified = true;
let pathStr;
if (process.argv[3]) {
    pathStr = process.argv[3];
    if (pathStr.endsWith('/')) {
        nameSpecified = false;
        console.warn(`No file name specified. File will be created as input-${day}.txt`);
    }
    if (!pathStr.includes('/')) {
        console.warn('No directory specified. File will be created in root');
    } else {
        if (pathStr.endsWith('/')) {
            pathArr = pathStr.substring(0, pathStr.length - 1).split('/');
        } else {
            pathArr = pathStr.split('/');
        }
        if (pathStr == `./${pathArr[pathArr.length-1]}` || pathStr == './') {
            pathArr = false;
        }
    }
} else {
    nameSpecified = false;
    console.warn(`No file name specified. File will be created as input-${day}.txt`);
    console.warn('No directory specified. File will be created in root');
}


const opts = {
    headers: {
        cookie: `session=${config.key}`
    }
};
fetch(`https://adventofcode.com/2020/day/${day}/input`, opts)
    .then(res => res.text())
    .then((content) => {
        let dirs;
        if (pathArr && nameSpecified) {
            dirs= pathStr.replace(pathArr[pathArr.length-1], '');
        }

        if (pathArr && !fs.existsSync(pathStr)) {
            fs.mkdirSync(dirs, { recursive: true });
        }
        console.log(pathStr);
        console.log(pathArr);
        console.log(dirs);
        console.log('NAMESPEC: '+ nameSpecified);

        try {
            fs.writeFileSync(pathStr, content.trim());
        } catch (e) {
            console.error(e);
            return;
        }
        console.log('Success!');
    });


// TODO: further testing
