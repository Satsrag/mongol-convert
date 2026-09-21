// swift-tools-version:5.9
import PackageDescription

// Swift package template for iOS. SwiftPM requires Package.swift at a repository root, so
// publish this directory from a split repository (or as a generated package archive) before using a
// remote `.package(url:from:)` dependency.
//
// Two pieces, produced by the release CI (see .github/workflows/release.yml):
//   - MongolConvertSwift.xcframework : libmongol_convert_uniffi static lib for device+simulator + the FFI header/modulemap
//   - Sources/MongolConvert/mongol_convert_uniffi.swift : copy `sw/mongol_convert_uniffi.swift` here from the release archive
let package = Package(
    name: "MongolConvert",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "MongolConvert", targets: ["MongolConvert"]),
    ],
    targets: [
        // Local path while developing; for distribution swap to `url:`+`checksum:` of a GitHub release zip:
        //   .binaryTarget(name: "MongolConvertSwift", url: "https://.../MongolConvertSwift.xcframework.zip", checksum: "<sha256>")
        .binaryTarget(name: "MongolConvertSwift", path: "MongolConvertSwift.xcframework"),
        .target(name: "MongolConvert", dependencies: ["MongolConvertSwift"], path: "Sources/MongolConvert"),
    ]
)
