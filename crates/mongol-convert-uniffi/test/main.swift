// Smoke test for the UniFFI Swift binding: translate(argv) -> stdout.
// Compiled with the generated mongol_convert_uniffi.swift and linked against libmongol_convert_uniffi, this exercises
// the exact code an iOS app would run. The harness compares stdout to the Java golden corpus.
// (Named main.swift so top-level statements are allowed in a multi-file build.)
import Foundation

let args = CommandLine.arguments
guard args.count >= 4 else {
    FileHandle.standardError.write("usage: swift_smoke <from> <to> <input>\n".data(using: .utf8)!)
    exit(2)
}
do {
    let out = try translate(from: args[1], to: args[2], input: args[3])
    FileHandle.standardOutput.write(out.data(using: .utf8)!)
} catch {
    FileHandle.standardError.write("error: \(error)\n".data(using: .utf8)!)
    exit(1)
}
