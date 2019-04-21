console.time();
const airQuality = require('../json_files/airQuality.json');

const parseColumns = (aq) => {
    const myColumns = aq.meta.view.columns;

    return myColumns
        .map(column => column.name);
};

const getDataArray = (aq) => {
    const myData = aq.data;
    return myData
        .map(dataItem => dataItem);
};

const joinData = (data, columns) => {
    const myData = data
        .map(item => item
            .map((col, index) => [columns[index], col])
            .filter((col, index) => index > 7)
        )
        .map(item => new Map(item))
        .map(item => {
            let myObj = {};
            for (const [key, value] of item.entries()) {
                myObj = Object.assign({}, myObj, { [key]: value }); 
            }
            return myObj;
        });
    return myData;
};

joinData(getDataArray(airQuality), parseColumns(airQuality));
console.timeEnd();