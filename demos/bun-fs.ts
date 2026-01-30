const addon = require(new URL("../bin/addon.node", import.meta.url).pathname);

const path = new URL("./demo-output.txt", import.meta.url).pathname;
const payload = `runtime: ${Bun.version}\nmessage: ${addon.hello()}\n`;

await Bun.write(path, payload);
const file = Bun.file(path);
const text = await file.text();
const hash = Bun.hash(text, "sha256");

console.log({
  path,
  bytes: file.size,
  hash,
  text
});
