// The gpt-tokenizer project's headline benchmark, rebuilt from its published
// method (benchmark/src/datasets.ts and benchmarkWorker.ts in
// github.com/niieani/gpt-tokenizer): the same seven inputs, encoded, counted
// and decoded over and over with each library's default caches; at least
// 100 ms and 5 calls of warmup, then at least 300 ms and 5 calls, in batches
// of 100 calls for short inputs; the score is the mean ms per call over the
// seven inputs; five fresh processes per library, median [min, max].
//
//   node seven.mjs [o200k_base|cl100k_base]          (run where the packages are installed)
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";
const require = createRequire(path.join(process.cwd(), "x.js"));
const self = new URL(import.meta.url).pathname;

const LATIN = `Occaecat est tempor incididunt voluptate exercitation irure quis aliqua sunt dolor. Anim nostrud incididunt eu aliquip quis culpa do incididunt eu. Magna qui dolor deserunt sit velit. Dolor anim laborum ut ad in et occaecat enim elit culpa commodo. Sit ut sit mollit adipisicing. Labore culpa do cillum proident incididunt et. Reprehenderit nisi excepteur culpa consectetur mollit consectetur laborum`;
function randomUnicode(length) {
  let seed = 0x12345678;
  const random = () => { seed ^= seed << 13; seed ^= seed >>> 17; seed ^= seed << 5; return (seed >>> 0) / 0x100000000; };
  const ranges = [[0x20, 0x7f], [0xa0, 0xff], [0x100, 0x17f], [0x400, 0x4ff], [0x980, 0x9ff], [0xf00, 0xfff], [0x1200, 0x137f], [0x3040, 0x309f], [0x30a0, 0x30ff], [0x4e00, 0x4fff], [0x1f300, 0x1f6ff]];
  let s = "";
  for (let i = 0; i < length; i++) { const r = ranges[Math.floor(random() * ranges.length)]; s += String.fromCodePoint(Math.floor(random() * (r[1] - r[0])) + r[0]); }
  return s;
}
const INPUTS = {
  English: "The quick brown fox jumps over the lazy dog.",
  Chinese: "快速的棕色狐狸跳过懒狗。",
  French: "Le renard brun rapide saute par-dessus le chien paresseux.",
  Code: "function greet(name: string): string {\n  return `Hello, ${name}!`;\n}",
  Latin: LATIN,
  LatinRepeat: LATIN.repeat(2000),
  UnicodeRandom: randomUnicode(120000),
};

const utf8 = new TextDecoder("utf8", { fatal: true, ignoreBOM: true });
const LIBS = {
  "tiktoken-fast": async (enc) => {
    const t = require(process.env.TKF_PKG).get_encoding(enc);
    return { encode: (s) => t.encode(s), decode: (ids) => utf8.decode(t.decode(ids)), count: (s) => t.count(s) };
  },
  "gpt-tokenizer": async (enc) => {
    const t = require(`gpt-tokenizer/cjs/encoding/${enc}`);
    return { encode: t.encode, decode: (ids) => t.decode(ids), count: t.countTokens };
  },
  tiktoken: async (enc) => {
    const t = require("tiktoken").get_encoding(enc);
    return { encode: (s) => t.encode(s), decode: (ids) => utf8.decode(t.decode(ids)), count: (s) => t.encode(s).length };
  },
  "@goliapkg/tiktoken-wasm": async (enc) => {
    const m = await import(path.join(process.cwd(), "node_modules/@goliapkg/tiktoken-wasm/tiktoken_wasm.js"));
    await m.default({ module_or_path: readFileSync(path.join(process.cwd(), "node_modules/@goliapkg/tiktoken-wasm/tiktoken_wasm_bg.wasm")) });
    const t = m.getEncoding(enc);
    return { encode: (s) => t.encode(s), decode: (ids) => t.decode(ids), count: (s) => t.count(s) };
  },
  "bpe-lite": async (enc) => {
    const { openai, openaiO200k } = require("bpe-lite");
    const t = enc === "cl100k_base" ? openai() : openaiO200k();
    return { encode: (s) => t.encode(s), decode: (ids) => t.decode(Array.from(ids)), count: (s) => t.count(s) };
  },
  "ai-tokenizer": async (enc) => {
    const { Tokenizer } = await import(path.join(process.cwd(), "node_modules/ai-tokenizer/dist/index.js")).catch(() => import("ai-tokenizer"));
    const params = await import(`ai-tokenizer/encoding/${enc}`).catch(() => import(path.join(process.cwd(), `node_modules/ai-tokenizer/dist/encoding/${enc}.js`)));
    const t = new Tokenizer(params.default ?? params);
    return { encode: (s) => t.encode(s), decode: (ids) => t.decode(Array.from(ids)), count: (s) => t.count(s) };
  },
};

function timed(fn, minMs, minCalls, batch) {
  let calls = 0, sink = 0;
  const start = performance.now();
  do { for (let i = 0; i < batch; i++) { const v = fn(); sink ^= typeof v === "number" ? v : v.length; } calls += batch; } while (calls < minCalls || performance.now() - start < minMs);
  return { ms: (performance.now() - start) / calls, sink };
}

async function worker(lib, enc) {
  const t0 = performance.now();
  const t = await LIBS[lib](enc);
  const loadMs = performance.now() - t0;
  const ref = require("tiktoken").get_encoding(enc);
  const out = { lib, loadMs, encode: {}, count: {}, decode: {}, ok: true };
  for (const [name, text] of Object.entries(INPUTS)) {
    const ids = Array.from(t.encode(text));
    if (ids.join() !== Array.from(ref.encode(text)).join() || t.count(text) !== ids.length || t.decode(Uint32Array.from(ids)) !== text) out.ok = false;
    const batch = text.length > 10000 ? 1 : 100;
    const idsTyped = lib === "tiktoken" || lib === "tiktoken-fast" || lib === "@goliapkg/tiktoken-wasm" ? Uint32Array.from(ids) : ids;
    for (const [op, fn] of [["encode", () => t.encode(text)], ["count", () => t.count(text)], ["decode", () => t.decode(idsTyped)]]) {
      timed(fn, 100, 5, batch);
      out[op][name] = timed(fn, 300, 5, batch).ms;
    }
  }
  return out;
}

if (process.argv[2] === "--worker") {
  worker(process.argv[3], process.argv[4]).then((r) => process.stdout.write(JSON.stringify(r)));
} else {
  const enc = process.argv[2] || "o200k_base";
  const libs = (process.env.LIBS || Object.keys(LIBS).join(",")).split(",");
  const PROCS = +(process.env.PROCS || 5);
  const runs = Object.fromEntries(libs.map((l) => [l, []]));
  for (let p = 0; p < PROCS; p++) {
    for (const lib of libs.slice(p % libs.length).concat(libs.slice(0, p % libs.length))) {
      const r = spawnSync(process.execPath, ["--expose-gc", self, "--worker", lib, enc], { encoding: "utf8", env: process.env, maxBuffer: 1 << 26 });
      if (r.status !== 0) { console.error(lib, r.stderr.slice(-2000)); continue; }
      runs[lib].push(JSON.parse(r.stdout));
      process.stderr.write(`process ${p + 1}/${PROCS} ${lib}\n`);
    }
  }
  const mean = (o) => Object.values(o).reduce((a, b) => a + b, 0) / Object.values(o).length;
  const med = (xs) => { const s = [...xs].sort((a, b) => a - b); return s[s.length >> 1]; };
  console.log(`${enc}, ${process.version}, ${PROCS} processes per library; mean ms/call over the seven inputs, median [min, max]`);
  for (const op of ["encode", "count", "decode"]) {
    const rows = libs.filter((l) => runs[l].length).map((l) => { const v = runs[l].map((r) => mean(r[op])); return [l, med(v), Math.min(...v), Math.max(...v), runs[l].every((r) => r.ok)]; }).sort((a, b) => a[1] - b[1]);
    console.log(`\n${op}`);
    rows.forEach(([l, m, lo, hi, ok], i) => console.log(`  #${i + 1} ${l.padEnd(24)} ${m.toFixed(3).padStart(9)} ms [${lo.toFixed(3)}, ${hi.toFixed(3)}]${ok ? "" : "  (output differs)"}`));
  }
  console.log("\nper input, median ms/call (encode):");
  for (const name of Object.keys(INPUTS)) console.log(`  ${name.padEnd(14)} ` + libs.filter((l) => runs[l].length).map((l) => `${l} ${med(runs[l].map((r) => r.encode[name])).toFixed(4)}`).join(" | "));
  console.log("\nload ms, median: " + libs.filter((l) => runs[l].length).map((l) => `${l} ${med(runs[l].map((r) => r.loadMs)).toFixed(1)}`).join(" | "));
}
