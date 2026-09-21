# zvvnmod/mongol-convert (PHP)

Mongolian Encoding Converter for PHP — a thin **FFI** binding to the Rust `mongol-convert` core, verified
byte-exact against the original Java (`200/200` on the shared golden corpus). One engine for PHP,
servers, web and mobile instead of a hand-maintained PHP reimplementation.

## Install

```bash
composer require zvvnmod/mongol-convert
```

Requires `php >= 7.4` with `ext-ffi` enabled (`ffi.enable=1`, or preload the library in production).

## Use

```php
use MongolConvert\MongolConvert;

echo MongolConvert::translate(MongolConvert::Z52, MongolConvert::MENK_SHAPE, $z52Input);
echo MongolConvert::translate(MongolConvert::DELEHI, MongolConvert::ZVVNMOD, $unicodeInput);
echo MongolConvert::version();
```

Encodings: `MongolConvert::ZVVNMOD`, `MongolConvert::DELEHI`, `MongolConvert::MENK_SHAPE`, `MongolConvert::MENK_LETTER`, `MongolConvert::Z52`,
and `MongolConvert::UTN57` (canonical UTN #57 Unicode), valid as either `$from` or `$to`.
`translate()` throws `RuntimeException` on an unknown encoding or unsupported conversion.

## The native library

The wrapper loads `libmongol_convert` (the `mongol-convert-cabi` C ABI). It is resolved in this order:

1. `MONGOL_CONVERT_LIB` env var (absolute path to the lib), else
2. `prebuilt/<os>-<arch>/libmongol_convert.{so,dylib,dll}` shipped in the package (filled by the release CI
   for `linux-x86_64`, `linux-aarch64`, `darwin-aarch64`, `darwin-x86_64`, `windows-x86_64`), else
3. the bare name `libmongol_convert.{ext}` from the system loader path.

To build it locally for your platform (drops it into `prebuilt/`):

```bash
bash bindings/php/scripts/build-lib.sh
```

### Production note

With `ffi.enable=preload`, declare the library in `opcache.preload` for best performance and to
allow FFI outside the CLI. The wrapper uses `FFI::cdef` (definitions inline), so no separate header
file is needed; for preloading you can switch to `FFI::load` of a `.h` if you prefer.

## Replacing the old PHP port

This package is a drop-in for the hand-written `meco_php`: same conversions, but backed by the
single Java-verified Rust core, so it can't drift. Swap `MongolConvert\TranslateService::translate(...)`
calls for `MongolConvert\MongolConvert::translate(...)` (note the argument order is `from, to, input`).
