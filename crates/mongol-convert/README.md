# mongol-convert

`mongol-convert` is the pure-Rust engine for the Mongolian Encoding Converter. It converts among
`Zvvnmod`, `Delehi`, `MenkShape`, `MenkLetter`, `Z52`, and canonical UTN #57 Unicode in both
directions; conversions route through the Zvvnmod hub. Everything runs in process with
no I/O, so the crate builds for `wasm32-unknown-unknown` as well as native targets. The minimum
supported Rust version is 1.82.

## Basic use

```rust
use mongol_convert::{translate, CodeType};

let input = "\u{E0E5}";
let output = translate(CodeType::MenkShape, CodeType::Zvvnmod, input)
    .expect("ZVVNMOD conversion should succeed");
```

## Command line

The package also installs a `mongol-convert` binary without changing how Rust projects depend on the library:

```sh
cargo install mongol-convert --version 0.7.0 --locked
mongol-convert translate --from z52 --to menk_shape 'text'
```

Omit the final text argument to read UTF-8 from stdin. Output is written unchanged to stdout without
an extra newline, so the command is safe in pipelines:

```sh
printf '%s' 'text' | mongol-convert translate --from z52 --to menk_shape
```

Run `mongol-convert --help` for the canonical encoding names and `mongol-convert --version` to verify the installed
release.

## Optional suffix separator repair

For MenkLetter or Delehi input with lost suffix separators, enable input repair before conversion:

```rust
use mongol_convert::{translate_with_options, CodeType, TranslationOptions};

let result = translate_with_options(
    CodeType::MenkLetter,
    CodeType::MenkLetter,
    "ᠤᠯᠤᠰ ᠤᠨ",
    &TranslationOptions { repair_suffix_separators: true, ..TranslationOptions::default() },
).unwrap();
assert_eq!(result.text, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ");
assert_eq!(result.warnings.len(), 1);
```

With the CLI built from this checkout, place the option after the target encoding:

```sh
mongol-convert translate --from menk_letter --to menk_shape --repair-suffix-separators 'ᠤᠯᠤᠰ ᠤᠨ'
```

The option also works with stdin. Repairs are listed on stderr; stdout contains only converted
text. Use `--` before a literal text argument that matches the option name.

Repair is **off by default**. It replaces a single space (U+0020) or NBSP (U+00A0) before one
of the exact suffix spellings below with NNBSP (U+202F). A lost separator matters because a
suffix after NNBSP is shaped as a particle, but the same letters after a space are shaped as an
independent word. The space must follow a token that can take a suffix: a Mongolian word, a
number, a word in another script (`APP ᠪᠡᠷ`), a closing bracket or quote (`︾ ᠶᠢᠨ`) or a unit
symbol (`60° ᠡᠴᠡ`), but not sentence punctuation, an opening bracket or another space.

The suffix spellings are the original 19 case suffixes

`ᠶᠢᠨ`, `ᠤᠨ`, `ᠦᠨ`, `ᠤ`, `ᠦ`, `ᠶᠢ`, `ᠢ`, `ᠳᠤ`, `ᠳᠦ`, `ᠲᠤ`, `ᠲᠦ`,
`ᠳᠤᠷ`, `ᠳᠦᠷ`, `ᠲᠤᠷ`, `ᠲᠦᠷ`, `ᠠᠴᠠ`, `ᠡᠴᠡ`, `ᠢᠶᠠᠷ`, `ᠢᠶᠡᠷ`,

plus iyan/iyen, luγ-a/lüge, nuγud/nügüd, ud/üd, daγan/degen, taγan/tegen (also `ᠲᠡᠬᠡᠨ`),
yuγan/yügen, ačaγan/ečegen, duni/düni, tuni/tüni, dahi/dehi, taki/teki, bar/ber, ban/ben,
tai/tei, nar/ner and the ordinals duγar/düger.

No vowel-harmony check is made: after NNBSP a suffix renders the same whatever precedes it, and
the written spelling is kept. After a Mongolian word, three rules decline stems after which the same
letters are usually an independent word (after other tokens the spelling is taken as written):

- Ordinals are not repaired after a Mongolian word, since dugar is also a word; after a number
  (`3 ᠳᠤᠭᠠᠷ`) or any other token they are.
- T-initial suffixes other than tai/tei need a stem ending in a consonant that selects them,
  such as ᠷ, ᠭ, ᠰ or ᠳ; after a vowel, `ᠲᠦᠷ` is the word tür.
- Bar/ber and ban/ben need a stem ending in a vowel or ᠶ, so `ᠴᠠᠭᠠᠨ ᠪᠠᠷ` ("white tiger")
  is left alone.

Stems may contain variation selectors, ZWJ/ZWNJ or MVS, and internal MVS characters
in suffixes are preserved. In suffix chains the rules use the immediately preceding segment.

See the [particle mapping audit](../../docs/suffix-separator-repair.md) for the exact Unicode
spellings, the corpus evidence and the decisions for all 49 entries in the pinned font table.
Repair supports 54 spellings in total; this is not a complete Mongolian suffix inventory.
Particles whose normal separator is an ordinary space (`ᠴᠤ`, `ᠶᠤᠮ`, `ᠬᠦ`, `ᠨᠢ`, `ᠦᠭᠡᠢ`)
are excluded. A font's particle table is not a repair allowlist.

This is an explicit spelling heuristic, not grammatical validation: it cannot tell whether a
suffix-like token was intended as a separate word or a quoted letter. In two 50 MB corpus tests with
every NNBSP removed, at least 99.65% of repairs restored an original NNBSP, and most of the rest
were suffixes the writer had spaced after a Latin word. Concatenated words,
FVS-bearing suffix spellings, existing NNBSP/MVS, tabs, newlines and runs of multiple spaces are
left alone. The suffix inventory is based on the separated suffix examples in
[L2/19-130](https://unicode.org/L2/L2019/19130-mwg3-8-mong-spec-r.pdf) and
[L2/18-293](https://www.unicode.org/L2/L2018/18293-nnbsp-solution.pdf).

Each change returns `Warning::RepairedSuffixSeparator` with the **original input UTF-8 byte
offset** and replaced character, followed by any conversion warnings. Repair runs even for
same-encoding conversions. Any supported target can be used; the usual conversion rules then
represent the repaired boundary in that target. Enabling repair for other source encodings
returns `MongolConvertError::UnsupportedInputRepair`, since their suffix spellings require different rules.
The existing `translate` and `translate_with_warnings` APIs keep their behavior. This option is
exposed in Rust, the CLI and the WebAssembly binding (`translate_with_options`, on by default in
the web demo); the other platform bindings still use the default API.

## MenkShape emoji restore

When `menk_shape` is the source encoding, conversion restores legacy SoftBank/iOS emoji that
collide with the MenkShape private-use range before decoding. This is on by default so text copied
through chat apps such as WeChat can recover MenkShape glyphs that were rewritten as emoji.

Disable it when the emoji should be treated as literal Unicode text:

```rust
use mongol_convert::{translate_with_options, CodeType, TranslationOptions};

let result = translate_with_options(
    CodeType::MenkShape,
    CodeType::Zvvnmod,
    "➡️",
    &TranslationOptions {
        restore_menk_shape_emoji: false,
        ..TranslationOptions::default()
    },
).unwrap();
assert_eq!(result.text, "➡️");
```

The CLI equivalent is `--no-restore-menk-shape-emoji`.

## UTN #57 output

Canonical UTN #57 Unicode output is part of the default build and uses the same API:

```rust
use mongol_convert::{translate, CodeType};

let input = "\u{E0E5}";
let output = translate(CodeType::MenkShape, CodeType::Utn57, input)
    .expect("UTN #57 conversion should succeed");
assert_eq!(output, "\u{180A}");
```

The conversion is performed in process by the pure-Rust `zvvnmod-utn57` crate and its pinned
`mongol-norm` normalizer. No Python, subprocess, installer, filesystem, or network access is
involved, so the same code path runs on servers, desktops, mobile, and WebAssembly.

A failing conversion returns `MongolConvertError::Utn57(reason)`. Reverse conversion from UTN #57 remains
unsupported and returns `MongolConvertError::Unsupported(CodeType::Utn57)`. Identity and blank-input
conversions keep the normal short-circuit behavior.

The `utn57-command` feature from 0.2.x is kept as a deprecated no-op so existing
`--features utn57-command` commands still build; it no longer changes anything.

The conversion path is:

```text
source encoding
→ mongol-convert ZVVNMOD hub
→ zvvnmod-utn57 0.1.0 positioned written units
→ mongol-norm 0.1.1 (linked in)
→ canonical Unicode
```

For bindings, distribution, and the Java-oracle verification details, see the
[mongol-convert repository](https://github.com/Satsrag/mongol-convert).
