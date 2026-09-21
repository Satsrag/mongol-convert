# mongol-convert-wasm — WebAssembly binding (web / Node / edge)

wasm-bindgen binding over `mongol-convert`, producing an npm package for the browser, Node, Deno/Bun and
edge runtimes (Cloudflare Workers, etc.). API:

```js
import init, { translate, version } from "mongol-convert-wasm";  // bundler/web target
// or: const { translate, version } = require("./pkg/mongol_convert_wasm.js");  // nodejs target
translate("z52", "menk_shape", input); // -> String; throws on unknown encoding / unsupported path
translate("delehi", "utn57_shape", input); // -> "SAIIA"-style written-unit spelling; reads too
translate_with_warnings("zvvnmod", "utn57", input); // -> { text, warnings: string[], repairs: [] }; same throws
translate_with_options("menk_letter", "utn57", input, true); // also repairs lost suffix separators
// -> { text, warnings, repairs: string[] }; `repairs` has one entry per space turned into NNBSP.
// The repair flag is accepted for menk_letter / delehi input only; other sources throw.
```

`from`/`to` are canonical encoding names: `zvvnmod`, `delehi`, `menk_shape`, `menk_letter`, `z52`.
`utn57` (canonical UTN #57 Unicode) works as either; the conversion runs inside the wasm module
with no network or filesystem access.

## Status

Verified on Node (wasm-bindgen `--target nodejs`): **200/200 byte-exact** vs the Java golden corpus
across Z52↔MenkShape, Delehi↔Z52, Menk_Letter→Delehi, Delehi→Menk_Letter.

## Build

**With wasm-pack** (recommended — emits a ready npm package incl. `package.json`):

```sh
wasm-pack build crates/mongol-convert-wasm --target web      # browsers / bundlers
wasm-pack build crates/mongol-convert-wasm --target nodejs   # Node (CommonJS)
wasm-pack build crates/mongol-convert-wasm --target bundler  # webpack/vite/rollup
```

**With wasm-bindgen-cli** (what CI used here; pin the CLI to the `wasm-bindgen` crate version):

```sh
cargo build -p mongol-convert-wasm --target wasm32-unknown-unknown --release
cargo install wasm-bindgen-cli --version 0.2.125   # match Cargo.lock
wasm-bindgen --target nodejs --out-dir crates/mongol-convert-wasm/pkg \
  target/wasm32-unknown-unknown/release/mongol_convert_wasm.wasm
node crates/mongol-convert-wasm/test/node_verify.js          # 200/200 byte-exact
```

The generated `pkg/` is git-ignored (reproducible). `wasm-opt` (binaryen) can shrink the ~210 KB
`.wasm` further. Publish `pkg/` to npm as usual.
