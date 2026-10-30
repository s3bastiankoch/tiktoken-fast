// tiktoken-fast (WebAssembly) against the JavaScript tokenizers in common use:
// tiktoken (OpenAI's Rust core compiled to WebAssembly), js-tiktoken (pure
// JavaScript) and gpt-tokenizer (pure JavaScript, with its own cache).
//
//   node bench.mjs <workload dir> [encoding ...]
//
// Each workload is capped at its first MAX_BYTES (default 1 MB). Every arm is
// checked against tiktoken's tokens first. Rounds rotate the arm order; a row
// is the median speed-up over tiktoken. tiktoken-fast and gpt-tokenizer keep
// caches: "stream" empties them at the start of each round, "cold" before
// every document, and "repeat" never (the first round is dropped), which is
// the same text encoded again with warm caches.
//
// TKF_NATIVE=<path to node/index.js> adds the native addon as a fifth arm.
// RAW=<file> appends every timed round and every check as one JSON line for
// bench/summarize.py.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
// npm packages resolve from the working directory, where they are installed
const require = createRequire(path.join(process.cwd(), "x.js"));

const [dir, ...encs] = process.argv.slice(2);
const ROUNDS = +(process.env.ROUNDS || 7);
const MAX_BYTES = +(process.env.MAX_BYTES || 1_000_000);
const NATIVE = process.env.TKF_NATIVE ? require(process.env.TKF_NATIVE) : null;
const RAW = process.env.RAW ? fs.openSync(process.env.RAW, "a") : null;
const raw = (o) => { if (RAW !== null) fs.writeSync(RAW, JSON.stringify({ lang: "js", ...o }) + "\n"); };
const PKG = process.env.TKF_PKG || path.join(path.dirname(new URL(import.meta.url).pathname), "../pkg-node/tiktoken_fast_js.js");
const fast = require(PKG);
const tiktoken = require("tiktoken");
const jsTiktoken = require("js-tiktoken");

const median = (xs) => { const s = [...xs].sort((a, b) => a - b); return s[s.length >> 1]; };
const eq = (a, b) => a.length === b.length && a.every((x, i) => x === b[i]);

const workloads = fs.readdirSync(dir).filter((f) => f.endsWith(".json")).sort().map((f) => {
  const docs = [];
  let n = 0;
  for (const d of JSON.parse(fs.readFileSync(path.join(dir, f), "utf8"))) {
    if (n >= MAX_BYTES) break;
    docs.push(d);
    n += Buffer.byteLength(d);
  }
  return [f.slice(0, -5), docs, n];
});

for (const enc of encs.length ? encs : ["cl100k_base", "o200k_base"]) {
  const tk = tiktoken.get_encoding(enc);
  const js = jsTiktoken.getEncoding(enc);
  const gpt = require(`gpt-tokenizer/cjs/encoding/${enc}`);
  const ours = fast.get_encoding(enc);
  const native = NATIVE ? NATIVE.get_encoding(enc) : null;
  const arms = {
    tiktoken: (d) => tk.encode_ordinary(d),
    "js-tiktoken": (d) => js.encode(d, [], []),
    "gpt-tokenizer": (d) => gpt.encode(d, { disallowedSpecial: new Set() }),
    "tiktoken-fast": (d) => ours.encode_ordinary(d),
  };
  if (native) arms["tiktoken-fast-native"] = (d) => native.encode_ordinary(d);
  // the two with caches get the same treatment: emptied at the start of each
  // round, and before every document in "cold"
  const resets = { "gpt-tokenizer": () => gpt.clearMergeCache(), "tiktoken-fast": () => ours.reset_caches() };
  if (native) resets["tiktoken-fast-native"] = () => native.reset_caches();
  for (const [wname, docs, nbytes] of workloads) {
    const bad = {};
    for (const [a, f] of Object.entries(arms)) {
      if (a === "tiktoken") continue;
      bad[a] = docs.filter((d) => !eq(Array.from(f(d)), Array.from(tk.encode_ordinary(d)))).length;
      raw({ kind: "check", enc, workload: wname, arm: a, docs: docs.length, bytes: nbytes, mism: bad[a] });
    }
    for (const sc of docs.length > 1 ? ["stream", "repeat", "cold"] : ["stream", "repeat"]) {
      const names = Object.keys(arms);
      const t = Object.fromEntries(names.map((a) => [a, []]));
      for (let r = 0; r < ROUNDS + (sc === "repeat"); r++) {
        const order = names.slice(r % names.length).concat(names.slice(0, r % names.length));
        for (const a of order) {
          const f = arms[a];
          let total = 0;
          const reset = sc === "repeat" ? null : resets[a];
          if (reset) reset();
          if (reset && sc === "cold") {
            for (const d of docs) { reset(); const s = performance.now(); f(d); total += performance.now() - s; }
          } else {
            const s = performance.now();
            for (const d of docs) f(d);
            total = performance.now() - s;
          }
          if (!(sc === "repeat" && r === 0)) {
            t[a].push(total);
            raw({ kind: "time", enc, workload: wname, scenario: sc, call: "encode_ordinary", arm: a, round: r - (sc === "repeat"), seconds: total / 1e3, bytes: nbytes });
          }
        }
      }
      const base = median(t.tiktoken);
      let line = `${enc.padEnd(11)} ${wname.padEnd(18)} ${sc.padEnd(6)} tiktoken ${(nbytes / base / 1e3).toFixed(1).padStart(6)} MB/s`;
      for (const a of names.slice(1)) line += ` | ${a} ${(base / median(t[a])).toFixed(2)}x`;
      console.log(line + ` | mism ${JSON.stringify(bad)}`);
    }
  }
  tk.free();
}
