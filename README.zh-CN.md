# mongol-convert

[English](README.md) | [简体中文](README.zh-CN.md)

传统蒙古文编码转换工具，支持 Delehi、MenkLetter、UTN #57、MenkShape、Z52 和 ZVVNMOD 互转。核心用 Rust 编写，提供命令行、Rust 库，以及 C、Swift、Android、WebAssembly 包，全部在本地运行，不依赖外部程序。

## 在线试用

**<https://www.satsrag.dev/convert/>** —— 在浏览器本地运行，不上传任何内容。

## 支持的编码

| 名称 | 编码 | 标准 |
|---|---|---|
| `delehi` | 德力海 Unicode | GB/T 25914-2010 |
| `menk_letter` | 蒙科立 Unicode | GB/T 25914-2010 |
| `utn57` | Unicode（UTN #57） | GB/T 25914-2023 |
| `menk_shape` | 蒙科立字形码 | — |
| `z52` | Z52 字形码 | — |
| `zvvnmod` | ZVVNMOD 字形码（内部中间码） | — |
| `utn57_shape` | UTN #57 的书写单元拼写，如 ᠰᠠᠢᠨ → `SAIIA` | — |

任意两种编码之间都可以互转。源编码不会自动识别：Delehi 和 MenkLetter 码位相同但规则不同，请根据文本来源选择 `--from`。

## 命令行

```sh
cargo install mongol-convert --locked
```

```sh
mongol-convert translate --from z52 --to utn57 'text'
mongol-convert translate --from delehi --to menk_shape < input.txt > output.txt
```

省略文本参数时从 stdin 读取。输出末尾不加换行；出错时写 stderr 并返回非零状态。

## Rust 库

```sh
cargo add mongol-convert
```

```rust
use mongol_convert::{translate, CodeType};

let output = translate(CodeType::MenkLetter, CodeType::Utn57, "text")?;
```

## 其他平台

从 [GitHub Releases](https://github.com/Satsrag/mongol-convert/releases/latest) 下载：

| 平台 | 文件 |
|---|---|
| C ABI（Linux / macOS / Windows） | `mongol-convert-c-<平台>.zip` |
| iOS / macOS | `MongolConvertSwift.xcframework.zip`、`MongolConvertC.xcframework.zip` |
| Android | `mongol-convert-android-release.aar` |
| 浏览器 / Node.js | `mongol-convert-wasm-web-<版本>.tgz`、`mongol-convert-wasm-nodejs-<版本>.tgz` |
| Agent skill | `mongolian-convert-<版本>.zip` |

Go、Python、PHP、Java、Dart 等可以通过 C ABI 调用。各语言示例见 [USAGE.md](USAGE.md)，skill 说明见 [skills/mongolian-convert](skills/mongolian-convert/README.md)。

## 注意

- 迁移数据时请保留原文和它的源编码。转换可能合并不同写法，往返转换不保证完全一致。
- 旧编码之间的转换已与原 Java 实现逐字节比对。

## 开发

```sh
git clone https://github.com/Satsrag/mongol-convert.git
cd mongol-convert
cargo test --workspace --locked
```

发布流程见 [DISTRIBUTION.md](DISTRIBUTION.md)。

## 相关项目

- [mongol-norm](https://github.com/Satsrag/mongol-norm) —— 传统蒙古文 UTN #57 字形引擎与规范化工具，纯 Rust 实现。`mongol-convert` 用它生成 UTN #57 输出。

## 许可证

Apache-2.0。本项目原名 `meco`，移植自 Java 版 [east-mod/meco](https://github.com/east-mod/meco)。
