const fs = require('fs');
const array = fs.readFileSync('./1').toString().split('\r\n');

console.log(array);
console.log(array.length);

setTimeout(async function(){

    for (var i = 0; i < array.length; i++) {
        for (var i2 = 0; i2 < array.length; i2++) {
            if (array[i] + array[i2] == 2020) console.log('megvan');
        }
    }
}, 500)
