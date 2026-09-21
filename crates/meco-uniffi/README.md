# meco-uniffi — iOS (Swift) & Android (Kotlin) bindings

UniFFI binding over `meco-core`. One Rust crate generates idiomatic Swift and Kotlin. Proc-macro
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
`libmeco_uniffi`, matches the Java golden corpus **120/120 byte-exact** across Z52↔MenkShape,
Delehi→MenkLetter, MenkLetter→Z52. Kotlin is generated; run it on a JVM/Android with JNA.

## Generate the bindings

```sh
cd mongol-convert
cargo build -p meco-uniffi
LIB=target/debug/libmeco_uniffi.dylib   # or .so on Linux
cargo run -p meco-uniffi --bin uniffi-bindgen -- generate --library "$LIB" --language swift  --out-dir crates/meco-uniffi/bindings/swift
cargo run -p meco-uniffi --bin uniffi-bindgen -- generate --library "$LIB" --language kotlin --out-dir crates/meco-uniffi/bindings/kotlin
```

Generated files (git-ignored — regenerate as part of packaging):
- Swift: `meco_uniffi.swift`, `meco_uniffiFFI.h`, `meco_uniffiFFI.modulemap`
- Kotlin: `uniffi/meco_uniffi/meco_uniffi.kt`

## iOS (XCFramework)

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo build -p meco-uniffi --release --target aarch64-apple-ios
cargo build -p meco-uniffi --release --target aarch64-apple-ios-sim
# (regenerate Swift bindings as above)
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libmeco_uniffi.a -headers crates/meco-uniffi/bindings/swift \
  -library target/aarch64-apple-ios-sim/release/libmeco_uniffi.a -headers crates/meco-uniffi/bindings/swift \
  -output Meco.xcframework
```
Ship `Meco.xcframework` + `meco_uniffi.swift` (a Swift Package or CocoaPod). For a static lib set
`crate-type = ["staticlib"]` for iOS builds.

## Android (AAR via cargo-ndk)

```sh
cargo install cargo-ndk
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o jniLibs build -p meco-uniffi --release
# place meco_uniffi.kt under src/main/kotlin, jniLibs/ under src/main/, add net.java.dev.jna:jna (aar)
```
Package `jniLibs/*/libmeco_uniffi.so` + the Kotlin file + JNA into an AAR.

## Notes

- `meco-core` stays `#![forbid(unsafe_code)]`; UniFFI's scaffolding/unsafe lives only in this crate.
- The C ABI (`meco-cabi`) is the path for servers (PHP-FFI / cgo / JNI·Panama); this crate is for
  native mobile apps where idiomatic Swift/Kotlin + automatic memory management are wanted.
