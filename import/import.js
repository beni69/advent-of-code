const fs = require('fs');
const path = require('path');
const fetch = require('node-fetch');
const config = require('../config');

const usage = `Usage: node ${path.basename(__filename)} <day|year/day> [path and name]`;
let year;
let day;
if (!process.argv[2]) {
    console.error('No day number provided');
    console.warn(usage);
    return;
} else if (isNaN(parseInt(process.argv[2])) || process.argv[2].length > 2) {
    console.error('Invalid day provided. Only enter the day number ex. \'5\'');
    console.warn(usage);
    return;
}
// else {
//     yearDay = process.argv[2].split('/');
//     if (yearDay.length == 1) {
//         day = yearDay[0];

//     } else if (yearDay.length == 2) {
//         year = yearDay[0];
//         day = yearDay[1];
//     }
// }

let pathArr;
let pathStr;
if (process.argv[3]) {
    pathStr = process.argv[3];
    if (pathStr.endsWith('/')) {
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
        if (pathStr == `./${pathArr[pathArr.length - 1]}` || pathStr == './') {
            pathArr = false;
        }
    }
} else {
    console.warn(`No file name specified. File will be created as input-${day}.txt`);
    console.warn('No directory specified. File will be created in root');
}


const opts = {
    headers: {
        cookie: `session=${config.key}`
    }
};
fetch(`https://adventofcode.com/${year}/day/${day}/input` || `https://adventofcode.com/2020/day/${day}/input`, opts)
    .then(res => res.text())
    .then((content) => {

        try {
            writeFileSyncRecursive(pathStr || `input-${day}.txt`, content.trim(), 'utf-8');
        } catch (e) {
            console.error(e);
            return;
        }
        console.log('Success!');
    });

// TODO: further testing


function writeFileSyncRecursive(filename, content, charset) {
    // -- normalize path separator to '/' instead of path.sep,
    // -- as / works in node for Windows as well, and mixed \\ and / can appear in the path
    let filepath = filename.replace(/\\/g, '/');

    // -- preparation to allow absolute paths as well
    let root = '';
    if (filepath[0] === '/') {
        root = '/';
        filepath = filepath.slice(1);
    }
    else if (filepath[1] === ':') {
        root = filepath.slice(0, 3);   // c:\
        filepath = filepath.slice(3);
    }

    // -- create folders all the way down
    const folders = filepath.split('/').slice(0, -1);  // remove last item, file
    folders.reduce(
        (acc, folder) => {
            const folderPath = acc + folder + '/';
            if (!fs.existsSync(folderPath)) {
                fs.mkdirSync(folderPath);
            }
            return folderPath;
        },
        root // first 'acc', important
    );

    // -- write file
    fs.writeFileSync(root + filepath, content, charset);
}
