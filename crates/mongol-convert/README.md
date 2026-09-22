# mongol-convert

Convert traditional Mongolian text between encodings. Pure Rust, no I/O, builds for native and `wasm32`. MSRV 1.82.

传统蒙古文编码转换。纯 Rust 实现，无 I/O，可编译到本地平台和 `wasm32`。最低 Rust 版本 1.82。

**Try it online / 在线试用:** <https://www.satsrag.dev/convert/>

## Encodings / 支持的编码

| Name / 名称 | Encoding / 编码 |
|---|---|
| `delehi` | Delehi Unicode / 德力海 Unicode (GB/T 25914-2010) |
| `menk_letter` | Menksoft Unicode / 蒙科立 Unicode (GB/T 25914-2010) |
| `utn57` | Unicode UTN #57 (GB/T 25914-2023) |
| `menk_shape` | Menksoft shape code / 蒙科立字形码 |
| `z52` | Z52 shape code / Z52 字形码 |
| `zvvnmod` | ZVVNMOD shape code / ZVVNMOD 字形码 |
| `utn57_shape` | UTN #57 as written units / UTN #57 书写单元拼写 |

Any encoding converts to any other. The source encoding is not auto-detected.

任意编码之间都可互转。源编码不会自动识别，请按文本来源指定。

## Library / 库

```sh
cargo add mongol-convert
```

```rust
use mongol_convert::{translate, CodeType};

let output = translate(CodeType::MenkLetter, CodeType::Utn57, "text")?;
```

## CLI / 命令行

```sh
cargo install mongol-convert --locked
mongol-convert translate --from z52 --to utn57 'text'
mongol-convert translate --from delehi --to menk_shape < input.txt > output.txt
```

Run `mongol-convert --help` for all options. / 运行 `mongol-convert --help` 查看全部选项。

## Options / 选项

| Option / 选项 | Default / 默认 | CLI / 命令行 |
|---|---|---|
| `repair_suffix_separators` | off / 关 | `--repair-suffix-separators` |
| `restore_menk_shape_emoji` | on / 开 | `--no-restore-menk-shape-emoji` (turns it off / 关闭) |

- **Suffix separator repair / 词尾分隔符修复**: for `menk_letter` and `delehi` input, turns a space before a known suffix back into NNBSP (U+202F), e.g. `ᠤᠯᠤᠰ ᠤᠨ` → `ᠤᠯᠤᠰ\u{202F}ᠤᠨ`. Each repair is reported as a warning.
  仅用于 `menk_letter` 和 `delehi` 输入：把已知词尾前的空格改回 NNBSP（U+202F），每处修改都会返回一条警告。
- **MenkShape emoji restore / MenkShape 表情还原**: for `menk_shape` input, turns emoji that chat apps (e.g. WeChat) rewrote back into MenkShape glyphs. Turn it off to keep real emoji.
  仅用于 `menk_shape` 输入：把微信等聊天软件改写成的表情还原为蒙科立字形码。文本里有真正的表情时请关闭。

```rust
use mongol_convert::{translate_with_options, CodeType, TranslationOptions};

let options = TranslationOptions {
    repair_suffix_separators: true,
    ..TranslationOptions::default()
};
let result = translate_with_options(CodeType::MenkLetter, CodeType::MenkLetter, "ᠤᠯᠤᠰ ᠤᠨ", &options)?;
assert_eq!(result.text, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ");
```

```sh
mongol-convert translate --from menk_letter --to menk_letter --repair-suffix-separators 'ᠤᠯᠤᠰ ᠤᠨ'
mongol-convert translate --from menk_shape --to utn57 --no-restore-menk-shape-emoji 'text'
```

Options go after `--to` and before the text. Repairs are listed on stderr.

选项写在 `--to` 之后、文本之前。修复记录输出到 stderr。

## More / 更多

C, Swift, Android and WebAssembly packages, usage and verification details:
<https://github.com/Satsrag/mongol-convert>

C、Swift、Android、WebAssembly 包，以及用法和验证说明见上方仓库。

Related / 相关项目: [mongol-norm](https://crates.io/crates/mongol-norm) — UTN #57 shaping engine and canonical normalizer, used for UTN #57 output. / UTN #57 字形引擎与规范化工具，`mongol-convert` 用它生成 UTN #57 输出。

License / 许可证: Apache-2.0
