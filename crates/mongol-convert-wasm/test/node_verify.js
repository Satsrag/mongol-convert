// Verify the WASM (Node target) binding against the Java golden corpus.
// Usage: node node_verify.js [pkgDir]   (pkgDir defaults to ../pkg produced by `wasm-pack build`)
const path = require("path");
const fs = require("fs");

// Entry: the wasm-bindgen (nodejs target) JS module, or a wasm-pack pkg dir.
const entry = process.argv[2] || path.join(__dirname, "..", "pkg", "mongol_convert_wasm.js");
const lib = require(entry);
const golden = path.join(__dirname, "..", "..", "mongol-convert", "tests", "golden", "golden.tsv");

function unesc(s) {
  let o = "";
  for (let i = 0; i < s.length; ) {
    if (s[i] === "\\" && s[i + 1] === "u") {
      o += String.fromCodePoint(parseInt(s.substr(i + 2, 4), 16));
      i += 6;
    } else {
      o += s[i++];
    }
  }
  return o;
}

const want = new Set(["Z52|Menk_Shape", "Menk_Shape|Z52", "Delehi|Z52", "Menk_Letter|Delehi", "Delehi|Menk_Letter"]);
const buckets = new Map([...want].map((k) => [k, []]));
for (const line of fs.readFileSync(golden, "utf8").split("\n")) {
  if (!line) continue;
  const [f, t, i, o] = line.split("\t");
  const k = `${f}|${t}`;
  if (want.has(k) && buckets.get(k).length < 40) buckets.get(k).push([unesc(i), unesc(o)]);
}

let total = 0, ok = 0;
for (const [k, rows] of buckets) {
  const [f, t] = k.split("|");
  let bad = 0;
  for (const [inp, exp] of rows) {
    total++;
    let got;
    try { got = lib.translate(f.toLowerCase(), t.toLowerCase(), inp); } catch (e) { got = `<throw:${e}>`; }
    if (got === exp) ok++;
    else { bad++; if (bad <= 1) console.log(`  MISMATCH ${k} in=${JSON.stringify(inp)} exp=${JSON.stringify(exp)} got=${JSON.stringify(got)}`); }
  }
  console.log(`${k}: ${rows.length - bad}/${rows.length} ok`);
}
console.log(`\nWASM (Node) vs Java golden: ${ok}/${total} byte-exact`);
console.log("version():", lib.version());
process.exit(ok === total ? 0 : 1);
