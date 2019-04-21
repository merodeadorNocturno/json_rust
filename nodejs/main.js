console.time('vsRust');
const myCurrencies = require('../json_files/currencies.json');

const myFilter = () => {
    return myCurrencies
        .filter(currency => currency.id > 768 && currency.id < 1899);
};

myFilter().map(item => console.log(item.name));
console.timeEnd('vsRust');
