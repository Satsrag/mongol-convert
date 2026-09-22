# Using mongol-convert on each platform (download model)

Every release attaches ready-to-download artifacts to the **GitHub Releases** page
(`github.com/Satsrag/mongol-convert/releases`). No package-manager account needed — just download and use.

All conversions are the same call everywhere:

```
translate(from, to, input)   // names: zvvnmod · delehi · menk_shape · menk_letter · z52 · utn57 · utn57_shape
```

The Rust crate, the `mongol-convert` command and the wasm package also offer `translate_with_warnings`: the
same text, plus one warning per hub run the UTN #57 encoder could spell only with an invented ZWJ
(the command prints them to stderr as `mongol-convert: warning: …`). The C ABI and the UniFFI bindings return
the text alone.

The **C library is the universal artifact**: `mongol-convert-c-<platform>.zip` contains
`libmongol_convert.{so,dylib,dll}` (dynamic), `libmongol_convert.a`/`mongol_convert.lib` (static) and `mongol_convert.h`. **C, Go, Python,
Dart and Java all load this one library** (snippets below). Swift / Objective-C / Android / Web get
their own native artifacts. The C ABI is:

```c
char *mongol_convert_translate(const char *from, const char *to, const char *input); // UTF-8 in/out; NULL on error
void  mongol_convert_free(char *ptr);            // free what mongol_convert_translate returned
const char *mongol_convert_version(void);
```

---

## Desktop / server command line

The `mongol-convert` crate is both a Rust library and an installable command. Install the published
release from crates.io:

```sh
cargo install mongol-convert --version 0.7.1 --locked
```

Convert a command-line argument:

```sh
mongol-convert translate --from z52 --to menk_shape 'text'
```

Or read UTF-8 from stdin for shell pipelines and server jobs:

```sh
printf '%s' 'text' | mongol-convert translate --from z52 --to menk_shape
```

The converter writes only the translated UTF-8 bytes to stdout and does not append a newline.
Errors go to stderr and return a non-zero exit status. `mongol-convert --help` lists canonical encoding names.

`--to utn57` works out of the box: the reviewed ZVVNMOD → UTN #57 mapping and the pure-Rust
`mongol-norm` normalizer are compiled into `mongol-convert`, so no extra package, Python, or installer is
needed. The same in-process backend ships in the Web, Android, iOS, and prebuilt C artifacts.

---

## C / C++

Download `mongol-convert-c-<platform>.zip`. `#include "mongol_convert.h"`, link `-lmeco`.

```c
#include "mongol_convert.h"
#include <stdio.h>
int main(void) {
    char *out = mongol_convert_translate("z52", "menk_shape", input_utf8);
    if (out) { fputs(out, stdout); mongol_convert_free(out); }
}
```
`cc demo.c -I. -L. -lmeco -o demo` (dynamic), or link `libmongol_convert.a` for static.

## Go (cgo, loads the C lib)

Put `mongol_convert.h` + `libmongol_convert.*` next to your package.

```go
/*
#cgo CFLAGS: -I${SRCDIR}
#cgo LDFLAGS: -L${SRCDIR} -lmeco
#include "mongol_convert.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

func Translate(from, to, in string) string {
    cf, ct, ci := C.CString(from), C.CString(to), C.CString(in)
    defer C.free(unsafe.Pointer(cf)); defer C.free(unsafe.Pointer(ct)); defer C.free(unsafe.Pointer(ci))
    out := C.mongol_convert_translate(cf, ct, ci)
    if out == nil { return "" }
    defer C.mongol_convert_free(out)
    return C.GoString(out)
}
```
(A pure-Go, no-cgo `wazero` module is planned; cgo works today over the downloaded lib.)

## Python (ctypes — standard library, no install)

Download `mongol-convert-c-<platform>.zip`.

```python
import ctypes
lib = ctypes.CDLL("./libmongol_convert.dylib")   # .so on Linux, .dll on Windows
lib.mongol_convert_translate.restype  = ctypes.c_void_p
lib.mongol_convert_translate.argtypes = [ctypes.c_char_p] * 3
lib.mongol_convert_free.argtypes      = [ctypes.c_void_p]

def translate(frm, to, s):
    p = lib.mongol_convert_translate(frm.encode(), to.encode(), s.encode())
    if not p: raise RuntimeError("mongol-convert: conversion failed")
    try:    return ctypes.string_at(p).decode("utf-8")
    finally: lib.mongol_convert_free(p)

print(translate("z52", "menk_shape", s))
```

## Dart (dart:ffi)

Add `ffi` to your pubspec; download the C lib.

```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

final _lib = DynamicLibrary.open('libmongol_convert.dylib');
final _tr = _lib.lookupFunction<
    Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
    Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>)>('mongol_convert_translate');
final _free = _lib.lookupFunction<Void Function(Pointer<Utf8>), void Function(Pointer<Utf8>)>('mongol_convert_free');

String translate(String from, String to, String input) {
  final f = from.toNativeUtf8(), t = to.toNativeUtf8(), i = input.toNativeUtf8();
  final out = _tr(f, t, i);
  malloc.free(f); malloc.free(t); malloc.free(i);
  if (out == nullptr) throw Exception('mongol-convert failed');
  final s = out.toDartString();
  _free(out);
  return s;
}
```

## Java (JNA, server JVM — loads the C lib)

Add `net.java.dev.jna:jna`; put `libmongol_convert.*` on the library path.

```java
import com.sun.jna.*;

public interface MongolConvertLib extends Library {
    MongolConvertLib I = Native.load("mongol_convert", MongolConvertLib.class);
    Pointer mongol_convert_translate(String from, String to, String input);
    void mongol_convert_free(Pointer p);
}

Pointer p = MongolConvertLib.I.mongol_convert_translate("z52", "menk_shape", s);
String out = (p == null) ? null : p.getString(0, "UTF-8");
if (p != null) MongolConvertLib.I.mongol_convert_free(p);
```

## Android

Download `mongol-convert-android-release.aar`. In `build.gradle`:
```kotlin
dependencies { implementation(files("libs/mongol-convert-android-release.aar")); implementation("net.java.dev.jna:jna:5.14.0@aar") }
```
```kotlin
import uniffi.mongol_convert_uniffi.translate
val out = translate("z52", "menk_shape", input)
```

## iOS — Swift

Download `MongolConvertSwift.xcframework.zip`. Drag the `.xcframework` into Xcode and add `mongol_convert_uniffi.swift`
(or use a local SwiftPM `binaryTarget`).
```swift
let out = try translate(from: "z52", to: "menk_shape", input: s)
```

## iOS — Objective-C

Download `MongolConvertC.xcframework.zip` (the C ABI). Add it to the project, then call C directly:
```objc
@import MongolConvert;                 // or: #import "mongol_convert.h"
char *out = mongol_convert_translate("z52", "menk_shape", s.UTF8String);
if (out) { NSString *r = @(out); mongol_convert_free(out); /* use r */ }
```

## Browser / web bundler — JavaScript

Download `mongol-convert-wasm-web-<ver>.tgz`, then `npm install ./mongol-convert-wasm-web-<ver>.tgz`.
```js
import init, { translate } from "mongol-convert-wasm";
await init();
translate("z52", "menk_shape", input);
```

## Node.js — JavaScript

Download `mongol-convert-wasm-nodejs-<ver>.tgz`, then `npm install ./mongol-convert-wasm-nodejs-<ver>.tgz`.
```js
const { translate } = require("mongol-convert-wasm");
translate("z52", "menk_shape", input);
```

## PHP

Use the package under `bindings/php` (Composer, or `require 'src/MongolConvert.php'`); it loads `libmongol_convert` via FFI.
```php
use MongolConvert\MongolConvert;
echo MongolConvert::translate(MongolConvert::Z52, MongolConvert::MENK_SHAPE, $input);
```

---

### Notes
- `from`/`to` accept the canonical names above. `oyun` remains unsupported. `utn57` converts both ways
  (valid as `to`, rejected as `from`) and is available in every binding and prebuilt package.
- Unmappable in-range characters pass through unchanged (lenient policy).
- Set the dynamic-loader path at runtime if the lib isn't installed system-wide:
  macOS `DYLD_LIBRARY_PATH`, Linux `LD_LIBRARY_PATH`, Windows put the `.dll` next to the executable.
