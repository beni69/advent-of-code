const fs = require('fs');
const path = require('path');
const fetch = require('node-fetch');
let config;
try {
    config = require('./config');

} catch (error) {
    console.error('You need to create a file called config.json and add your session cookie in a key called \'key\'');
    return;
}

// Defining year/day
const usage = `Usage: node ${path.basename(__filename)} <day|year/day> [path and name]`;
let year;
let day;
if (!process.argv[2]) {
    console.error('No day number provided');
    console.warn(usage);
    return;
} else {
    yearDay = process.argv[2].split('/');
    currentYear = new Date().getFullYear();
    if (yearDay.length == 1) {
        year = currentYear;
        day = yearDay[0];

    } else if (yearDay.length == 2) {
        year = yearDay[0];
        day = yearDay[1];
    } if (year < 2015 || year > currentYear || day < 1 || day > 25) {
        console.error('Invalid day or year provided. Only enter the numbers ex. 12 or 2018/12');
        return;
    }
}


// Defining path
let pathStr;
if (process.argv[3]) {
    pathStr = process.argv[3];
    if (pathStr.endsWith('/')) {
        pathStr += `input-${day}.txt`;
    }
} else {
    console.warn(`No file name specified. File will be created as input-${day}.txt`);
    console.warn('No directory specified. File will be created in root');
}

// defining cookie
const opts = {
    headers: {
        cookie: `session=${config.key}`
    }
};

// getting the actual website
fetch(`https://adventofcode.com/${year}/day/${day}/input`, opts)
    .then(res => res.text())
    .then((content) => {

        // writing it to file
        try {
            writeFileSyncRecursive(pathStr || `input-${day}.txt`, content.trim(), 'utf-8');
        } catch (e) {
            console.error(e);
            return;
        }
        console.log('Success!');
    });



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
