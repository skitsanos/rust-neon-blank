const addon = require(new URL("../bin/addon.node", import.meta.url).pathname);

const message = addon.hello();
console.log(message);
console.log(addon.greet("Bun"));
