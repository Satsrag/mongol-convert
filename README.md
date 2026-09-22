# mongol-convert

[English](README.md) | [简体中文](README.zh-CN.md)

Convert traditional Mongolian text between encodings: Delehi, MenkLetter, UTN #57, MenkShape, Z52, and ZVVNMOD. Written in Rust, available as a CLI, a Rust library, and C, Swift, Android, and WebAssembly packages. Everything runs locally with no external dependencies.

## Try it online

**<https://www.satsrag.dev/convert/>** — runs entirely in your browser; nothing is uploaded.

## Supported encodings

| Name | Encoding | Standard |
|---|---|---|
| `delehi` | Delehi Unicode | GB/T 25914-2010 |
| `menk_letter` | Menksoft Unicode | GB/T 25914-2010 |
| `utn57` | Unicode (UTN #57) | GB/T 25914-2023 |
| `menk_shape` | Menksoft shape code | — |
| `z52` | Z52 shape code | — |
| `zvvnmod` | ZVVNMOD shape code (the internal hub) | — |
| `utn57_shape` | UTN #57 spelled as written units, e.g. ᠰᠠᠢᠨ → `SAIIA` | — |

Any encoding can be converted to any other. The source encoding is not detected automatically: Delehi and MenkLetter share code points but follow different rules, so choose `--from` based on where the text came from.

## Command line

```sh
cargo install mongol-convert --locked
```

```sh
mongol-convert translate --from z52 --to utn57 'text'
mongol-convert translate --from delehi --to menk_shape < input.txt > output.txt
```

Without a text argument, input is read from stdin. Output has no trailing newline; errors go to stderr with a non-zero exit status.

## Rust library

```sh
cargo add mongol-convert
```

```rust
use mongol_convert::{translate, CodeType};

let output = translate(CodeType::MenkLetter, CodeType::Utn57, "text")?;
```

## Other platforms

Download from [GitHub Releases](https://github.com/Satsrag/mongol-convert/releases/latest):

| Platform | Asset |
|---|---|
| C ABI (Linux / macOS / Windows) | `mongol-convert-c-<platform>.zip` |
| iOS / macOS | `MongolConvertSwift.xcframework.zip`, `MongolConvertC.xcframework.zip` |
| Android | `mongol-convert-android-release.aar` |
| Browser / Node.js | `mongol-convert-wasm-web-<version>.tgz`, `mongol-convert-wasm-nodejs-<version>.tgz` |
| Agent skill | `mongolian-convert-<version>.zip` |

Go, Python, PHP, Java, Dart, and others can load the C ABI. See [USAGE.md](USAGE.md) for per-language examples and [skills/mongolian-convert](skills/mongolian-convert/README.md) for the skill.

## Notes

- Keep the original text and its encoding when migrating data. Conversion may merge different spellings, so a round trip is not guaranteed to be identical.
- Conversions between the legacy encodings are verified byte-for-byte against the original Java implementation.

## Development

```sh
git clone https://github.com/Satsrag/mongol-convert.git
cd mongol-convert
cargo test --workspace --locked
```

See [DISTRIBUTION.md](DISTRIBUTION.md) for the release process.

## Related

- [mongol-norm](https://github.com/Satsrag/mongol-norm) — UTN #57 shaping engine and canonical normalizer for traditional Mongolian, pure Rust. `mongol-convert` uses it for UTN #57 output.

## License

Apache-2.0. Formerly `meco`; a Rust port of the Java [east-mod/meco](https://github.com/east-mod/meco).
