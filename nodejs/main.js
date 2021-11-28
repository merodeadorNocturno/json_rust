console.time('nodejs');
const myCurrencies = require('../json_files/currencies.json');

const myFilter = () => {
    return myCurrencies
        // .filter(currency => currency.id > 76 && currency.id < 1899)
        .filter(currency => currency.name === "XRP");
};

const result = myFilter();
console.timeEnd('nodejs');
console.log(`coins ${result[0].name}`);