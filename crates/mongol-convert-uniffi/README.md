# mongol-convert-uniffi — iOS (Swift) & Android (Kotlin) bindings

UniFFI binding over `mongol-convert`. One Rust crate generates idiomatic Swift and Kotlin. Proc-macro
mode (no `.udl`). Public API:

```
translate(from: String, to: String, input: String) throws -> String   // Swift
fun translate(from: String, to: String, input: String): String        // Kotlin (throws MecoException)
version() -> String
```

`from`/`to` are canonical encoding names: `zvvnmod`, `delehi`, `menk_shape`, `menk_letter`, `z52`.
`utn57` (canonical UTN #57 Unicode) works as either and runs fully in process.

## Status

Swift binding **verified on host** (arm64-apple-darwin): the generated Swift, linked against
`libmongol_convert_uniffi`, matches the Java golden corpus **120/120 byte-exact** across Z52↔MenkShape,
Delehi→MenkLetter, MenkLetter→Z52. Kotlin is generated; run it on a JVM/Android with JNA.

## Generate the bindings

```sh
cd mongol-convert
cargo build -p mongol-convert-uniffi
LIB=target/debug/libmongol_convert_uniffi.dylib   # or .so on Linux
cargo run -p mongol-convert-uniffi --bin uniffi-bindgen -- generate --library "$LIB" --language swift  --out-dir crates/mongol-convert-uniffi/bindings/swift
cargo run -p mongol-convert-uniffi --bin uniffi-bindgen -- generate --library "$LIB" --language kotlin --out-dir crates/mongol-convert-uniffi/bindings/kotlin
```

Generated files (git-ignored — regenerate as part of packaging):
- Swift: `mongol_convert_uniffi.swift`, `mongol_convert_uniffiFFI.h`, `mongol_convert_uniffiFFI.modulemap`
- Kotlin: `uniffi/mongol_convert_uniffi/mongol_convert_uniffi.kt`

## iOS (XCFramework)

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo build -p mongol-convert-uniffi --release --target aarch64-apple-ios
cargo build -p mongol-convert-uniffi --release --target aarch64-apple-ios-sim
# (regenerate Swift bindings as above)
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmongol_convert_uniffi.a -headers crates/mongol-convert-uniffi/bindings/swift \
  -library target/aarch64-apple-ios-sim/release/libmongol_convert_uniffi.a -headers crates/mongol-convert-uniffi/bindings/swift \
  -output MongolConvert.xcframework
```
Ship `MongolConvert.xcframework` + `mongol_convert_uniffi.swift` (a Swift Package or CocoaPod). For a static lib set
`crate-type = ["staticlib"]` for iOS builds.

## Android (AAR via cargo-ndk)

```sh
cargo install cargo-ndk
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o jniLibs build -p mongol-convert-uniffi --release
# place mongol_convert_uniffi.kt under src/main/kotlin, jniLibs/ under src/main/, add net.java.dev.jna:jna (aar)
```
Package `jniLibs/*/libmongol_convert_uniffi.so` + the Kotlin file + JNA into an AAR.

## Notes

- `mongol-convert` stays `#![forbid(unsafe_code)]`; UniFFI's scaffolding/unsafe lives only in this crate.
- The C ABI (`mongol-convert-cabi`) is the path for servers (PHP-FFI / cgo / JNI·Panama); this crate is for
  native mobile apps where idiomatic Swift/Kotlin + automatic memory management are wanted.
