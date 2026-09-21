# Using meco on each platform (download model)

Every release attaches ready-to-download artifacts to the **GitHub Releases** page
(`github.com/Satsrag/mongol-convert/releases`). No package-manager account needed — just download and use.

All conversions are the same call everywhere:

```
translate(from, to, input)   // names: zvvnmod · delehi · menk_shape · menk_letter · z52 · utn57 · utn57_shape
```

The Rust crate, the `meco` command and the wasm package also offer `translate_with_warnings`: the
same text, plus one warning per hub run the UTN #57 encoder could spell only with an invented ZWJ
(the command prints them to stderr as `meco: warning: …`). The C ABI and the UniFFI bindings return
the text alone.

The **C library is the universal artifact**: `meco-c-<platform>.zip` contains
`libmeco.{so,dylib,dll}` (dynamic), `libmeco.a`/`meco.lib` (static) and `meco.h`. **C, Go, Python,
Dart and Java all load this one library** (snippets below). Swift / Objective-C / Android / Web get
their own native artifacts. The C ABI is:

```c
char *meco_translate(const char *from, const char *to, const char *input); // UTF-8 in/out; NULL on error
void  meco_free(char *ptr);            // free what meco_translate returned
const char *meco_version(void);
```

---

## Desktop / server command line

The `meco-core` crate is both a Rust library and an installable command. Install the published
release from crates.io:

```sh
cargo install meco-core --version 0.6.0 --locked
```

Convert a command-line argument:

```sh
meco translate --from z52 --to menk_shape 'text'
```

Or read UTF-8 from stdin for shell pipelines and server jobs:

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

The converter writes only the translated UTF-8 bytes to stdout and does not append a newline.
Errors go to stderr and return a non-zero exit status. `meco --help` lists canonical encoding names.

`--to utn57` works out of the box: the reviewed ZVVNMOD → UTN #57 mapping and the pure-Rust
`mongol-norm` normalizer are compiled into `meco`, so no extra package, Python, or installer is
needed. The same in-process backend ships in the Web, Android, iOS, and prebuilt C artifacts.

---

## C / C++

Download `meco-c-<platform>.zip`. `#include "meco.h"`, link `-lmeco`.

```c
#include "meco.h"
#include <stdio.h>
int main(void) {
    char *out = meco_translate("z52", "menk_shape", input_utf8);
    if (out) { fputs(out, stdout); meco_free(out); }
}
```
`cc demo.c -I. -L. -lmeco -o demo` (dynamic), or link `libmeco.a` for static.

## Go (cgo, loads the C lib)

Put `meco.h` + `libmeco.*` next to your package.

```go
/*
#cgo CFLAGS: -I${SRCDIR}
#cgo LDFLAGS: -L${SRCDIR} -lmeco
#include "meco.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

func Translate(from, to, in string) string {
    cf, ct, ci := C.CString(from), C.CString(to), C.CString(in)
    defer C.free(unsafe.Pointer(cf)); defer C.free(unsafe.Pointer(ct)); defer C.free(unsafe.Pointer(ci))
    out := C.meco_translate(cf, ct, ci)
    if out == nil { return "" }
    defer C.meco_free(out)
    return C.GoString(out)
}
```
(A pure-Go, no-cgo `wazero` module is planned; cgo works today over the downloaded lib.)

## Python (ctypes — standard library, no install)

Download `meco-c-<platform>.zip`.

```python
import ctypes
lib = ctypes.CDLL("./libmeco.dylib")   # .so on Linux, .dll on Windows
lib.meco_translate.restype  = ctypes.c_void_p
lib.meco_translate.argtypes = [ctypes.c_char_p] * 3
lib.meco_free.argtypes      = [ctypes.c_void_p]

def translate(frm, to, s):
    p = lib.meco_translate(frm.encode(), to.encode(), s.encode())
    if not p: raise RuntimeError("meco: conversion failed")
    try:    return ctypes.string_at(p).decode("utf-8")
    finally: lib.meco_free(p)

print(translate("z52", "menk_shape", s))
```

## Dart (dart:ffi)

Add `ffi` to your pubspec; download the C lib.

```dart
import 'dart:ffi';
import 'package:ffi/ffi.dart';

final _lib = DynamicLibrary.open('libmeco.dylib');
final _tr = _lib.lookupFunction<
    Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>),
    Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>)>('meco_translate');
final _free = _lib.lookupFunction<Void Function(Pointer<Utf8>), void Function(Pointer<Utf8>)>('meco_free');

String translate(String from, String to, String input) {
  final f = from.toNativeUtf8(), t = to.toNativeUtf8(), i = input.toNativeUtf8();
  final out = _tr(f, t, i);
  malloc.free(f); malloc.free(t); malloc.free(i);
  if (out == nullptr) throw Exception('meco failed');
  final s = out.toDartString();
  _free(out);
  return s;
}
```

## Java (JNA, server JVM — loads the C lib)

Add `net.java.dev.jna:jna`; put `libmeco.*` on the library path.

```java
import com.sun.jna.*;

public interface MecoLib extends Library {
    MecoLib I = Native.load("meco", MecoLib.class);
    Pointer meco_translate(String from, String to, String input);
    void meco_free(Pointer p);
}

Pointer p = MecoLib.I.meco_translate("z52", "menk_shape", s);
String out = (p == null) ? null : p.getString(0, "UTF-8");
if (p != null) MecoLib.I.meco_free(p);
```

## Android

Download `meco-android-release.aar`. In `build.gradle`:
```kotlin
dependencies { implementation(files("libs/meco-android-release.aar")); implementation("net.java.dev.jna:jna:5.14.0@aar") }
```
```kotlin
import uniffi.meco_uniffi.translate
val out = translate("z52", "menk_shape", input)
```

## iOS — Swift

Download `MecoSwift.xcframework.zip`. Drag the `.xcframework` into Xcode and add `meco_uniffi.swift`
(or use a local SwiftPM `binaryTarget`).
```swift
let out = try translate(from: "z52", to: "menk_shape", input: s)
```

## iOS — Objective-C

Download `MecoC.xcframework.zip` (the C ABI). Add it to the project, then call C directly:
```objc
@import Meco;                 // or: #import "meco.h"
char *out = meco_translate("z52", "menk_shape", s.UTF8String);
if (out) { NSString *r = @(out); meco_free(out); /* use r */ }
```

## Browser / web bundler — JavaScript

Download `meco-wasm-web-<ver>.tgz`, then `npm install ./meco-wasm-web-<ver>.tgz`.
```js
import init, { translate } from "meco-wasm";
await init();
translate("z52", "menk_shape", input);
```

## Node.js — JavaScript

Download `meco-wasm-nodejs-<ver>.tgz`, then `npm install ./meco-wasm-nodejs-<ver>.tgz`.
```js
const { translate } = require("meco-wasm");
translate("z52", "menk_shape", input);
```

## PHP

Use the package under `bindings/php` (Composer, or `require 'src/Meco.php'`); it loads `libmeco` via FFI.
```php
use Meco\Meco;
echo Meco::translate(Meco::Z52, Meco::MENK_SHAPE, $input);
```

---

### Notes
- `from`/`to` accept the canonical names above. `oyun` remains unsupported. `utn57` converts both ways
  (valid as `to`, rejected as `from`) and is available in every binding and prebuilt package.
- Unmappable in-range characters pass through unchanged (lenient policy).
- Set the dynamic-loader path at runtime if the lib isn't installed system-wide:
  macOS `DYLD_LIBRARY_PATH`, Linux `LD_LIBRARY_PATH`, Windows put the `.dll` next to the executable.
