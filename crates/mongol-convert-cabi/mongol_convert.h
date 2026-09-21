#ifndef MONGOL_CONVERT_H
#define MONGOL_CONVERT_H

/* C ABI for the mongol-convert Mongolian Encoding Converter core.
 * Loadable from PHP FFI, Go (cgo), Java (JNI/Panama), Python (ctypes), etc. */

#ifdef __cplusplus
extern "C" {
#endif

/* Translate UTF-8 `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names:
 * "zvvnmod", "delehi", "menk_shape", "menk_letter", "z52", plus the output-only "utn57".
 * Returns a newly allocated UTF-8 C string the caller must release with mongol_convert_free(), or NULL on
 * error (NULL arg, invalid UTF-8, unknown encoding, unsupported conversion). */
char *mongol_convert_translate(const char *from, const char *to, const char *input);

/* Release a string returned by mongol_convert_translate(). NULL is ignored. */
void mongol_convert_free(char *ptr);

/* Library version (static; do not free). */
const char *mongol_convert_version(void);

#ifdef __cplusplus
}
#endif

#endif /* MONGOL_CONVERT_H */
