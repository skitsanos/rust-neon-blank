const addon = require(new URL("../bin/addon.node", import.meta.url).pathname);

const input = JSON.stringify({
  ok: true,
  values: [1, 2, 3],
  meta: { runtime: "bun" }
});

const parsed = addon.parseJson(input);
console.log(parsed);
