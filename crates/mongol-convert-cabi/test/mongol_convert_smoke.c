/* Smoke test for the mongol-convert C ABI: translate(argv[1], argv[2], argv[3]) and print the result.
 * Drives the .dylib/.so exactly as a server (PHP FFI / cgo / JNI) would; the harness compares
 * its stdout against the Java golden corpus. */
#include "mongol_convert.h"
#include <stdio.h>

int main(int argc, char **argv) {
    if (argc < 4) {
        fprintf(stderr, "usage: %s <from> <to> <input>\n", argv[0]);
        return 2;
    }
    char *out = mongol_convert_translate(argv[1], argv[2], argv[3]);
    if (out == NULL) {
        fprintf(stderr, "mongol_convert_translate returned NULL\n");
        return 1;
    }
    fputs(out, stdout);
    mongol_convert_free(out);
    return 0;
}
