// tiktoken-fast (WebAssembly) against the tiktoken npm package.
//   node parity.mjs        (run where tiktoken is installed; TKF_PKG = the built package)
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire(path.join(process.cwd(), "x.js"));
const fast = require(process.env.TKF_PKG || path.join(path.dirname(new URL(import.meta.url).pathname), "../pkg-node/tiktoken_fast_js.js"));
const tiktoken = require("tiktoken");

const eq = (a, b) => a.length === b.length && a.every((x, i) => x === b[i]);
let seed = 1;
const rand = () => ((seed = (seed * 1103515245 + 12345) >>> 0) / 2 ** 32);
const pools = [
  "abcdefghijklmnopqrstuvwxyz      ",
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,;:!?'\"()[]{}<>-_/\\|@#$%^&*+=~`\n\t",
  "äöüßéèêàçñøåæœ",
  "абвгдежзийклмнопрстуфхцчшщыэюя",
  "的一是不了人我在有他这中大来上国个到说们",
  "สวัสดีชาวโลก",
  "😀🙂👍🏽🎉🔥",
];
function text(n) {
  let s = "";
  for (let i = 0; i < n; i++) { const p = [...pools[Math.floor(rand() * pools.length)]]; s += p[Math.floor(rand() * p.length)]; }
  return s;
}
const samples = ["", "hello world", "Hello, World! It's 2026 and we're testing... 1234567890", "<|endoftext|> inside", "a".repeat(3000), " ".repeat(300) + "x"];
for (let i = 0; i < 500; i++) samples.push(text(1 + Math.floor(rand() * 300)));

let failures = 0;
for (const name of ["r50k_base", "p50k_base", "cl100k_base", "o200k_base"]) {
  const a = tiktoken.get_encoding(name), b = fast.get_encoding(name);
  for (const t of samples) {
    const ra = Array.from(a.encode_ordinary(t)), rb = Array.from(b.encode_ordinary(t));
    if (!eq(ra, rb)) { failures++; console.log("encode_ordinary differs", name, JSON.stringify(t)); continue; }
    if (!eq(Array.from(a.encode(t, "all")), Array.from(b.encode(t, "all")))) { failures++; console.log("encode all differs", name, JSON.stringify(t)); }
    if (b.count(t) !== ra.length) { failures++; console.log("count differs", name); }
    if (!eq(Array.from(a.decode(new Uint32Array(ra))), Array.from(b.decode(new Uint32Array(rb))))) { failures++; console.log("decode differs", name); }
  }
  let threw = 0;
  for (const e of [a, b]) { try { e.encode("x <|endoftext|>"); } catch { threw++; } }
  if (threw !== 2) { failures++; console.log("special token check differs", name); }
  a.free(); b.free();
}
console.log(failures ? `${failures} failures` : "all equal");
process.exit(failures ? 1 : 0);
