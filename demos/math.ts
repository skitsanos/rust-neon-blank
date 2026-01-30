const addon = require(new URL("../bin/addon.node", import.meta.url).pathname);

const values = [1, 2, 3, 4.5];
console.log("add:", addon.add(2, 3));
console.log("sumArray:", addon.sumArray(values));
console.log("stats:", addon.stats(values));
