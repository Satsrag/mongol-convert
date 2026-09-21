#!/usr/bin/env bash
# Build the browser demo into a self-contained directory that can be dropped onto any static host.
#
#   crates/mongol-convert-wasm/web/build.sh [OUT_DIR]     # default OUT_DIR: crates/mongol-convert-wasm/web/dist
#
# Produces OUT_DIR/{index.html, pkg/, fonts/}. Fonts are the per-encoding fonts the demo needs to
# render Menksoft/Z52 PUA text and UTN #57 Unicode text; they are fetched rather than committed here.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
out="${1:-$here/dist}"

echo "==> cargo build (wasm32-unknown-unknown, release)"
cargo build --manifest-path "$root/Cargo.toml" -p mongol-convert-wasm --target wasm32-unknown-unknown --release

echo "==> wasm-bindgen --target web"
rm -rf "$out/pkg"
wasm-bindgen --target web --no-typescript \
    --out-dir "$out/pkg" \
    "$root/target/wasm32-unknown-unknown/release/mongol_convert_wasm.wasm"

echo "==> fonts"
mkdir -p "$out/fonts"
fetch() { # fetch <url-path> <dest-name>
    [ -f "$out/fonts/$2" ] || curl -sfL "https://raw.githubusercontent.com/Satsrag/meco/HEAD/$1" -o "$out/fonts/$2"
}
fetch "fonts/delehi/mnglwhiteotf.ttf"              delehi.ttf
fetch "fonts/menk/MQG8103.ttf"                     menk_letter.ttf
fetch "fonts/menk/MenksoftQagan_shape.ttf"         menk_shape.ttf
fetch "fonts/z52/7%20-%20Z52%20Tsagaan%20Tig.otf"  z52.otf
# zvvnmod.ttf is generated from fonts/zvvnmod/zvvnmod.sfd; the built copy lives on the tools site.
[ -f "$out/fonts/ZvvnMod.ttf" ] || \
    curl -sfL "https://raw.githubusercontent.com/Satsrag/satsrag.github.io/HEAD/mapping/assets/zvvnmod.ttf" \
         -o "$out/fonts/ZvvnMod.ttf"

# utn57.ttf = Noto Sans Mongolian 3.002, the mongfontbuilder reference build for UTN #57 shaping
# (SIL OFL 1.1 — the licence is staged next to it).
noto_ver=3.002
if [ ! -f "$out/fonts/utn57.ttf" ]; then
    tmp="$(mktemp -d)"
    curl -sfL -o "$tmp/noto.zip" \
        "https://github.com/notofonts/mongolian/releases/download/NotoSansMongolian-v${noto_ver}/NotoSansMongolian-v${noto_ver}.zip"
    unzip -q -o "$tmp/noto.zip" -d "$tmp"
    cp "$tmp/NotoSansMongolian/full/ttf/NotoSansMongolian-Regular.ttf" "$out/fonts/utn57.ttf"
    cp "$tmp/OFL.txt" "$out/fonts/utn57-NotoSansMongolian-OFL.txt"
    rm -rf "$tmp"
fi

cp "$here/index.html" "$out/index.html"
echo "==> done: $out"
echo "    preview:  python3 -m http.server -d '$out' 8000"
