<?php

declare(strict_types=1);

namespace MongolConvert;

/**
 * Mongolian Encoding Converter.
 *
 * Thin PHP wrapper over the Rust `mongol-convert` core via FFI. Drop-in replacement for hand-maintained
 * PHP ports: one shared, Java-verified engine instead of a parallel reimplementation.
 *
 * Usage:
 *   use MongolConvert\MongolConvert;
 *   echo MongolConvert::translate(MongolConvert::Z52, MongolConvert::MENK_SHAPE, $input);
 *
 * The native library (libmongol_convert.{so,dylib,dll}) is resolved from, in order:
 *   1. the MONGOL_CONVERT_LIB environment variable (absolute path), then
 *   2. prebuilt/<os>-<arch>/libmongol_convert.<ext> shipped in this package, then
 *   3. the bare name "libmongol_convert.<ext>" (system loader path).
 */
final class MongolConvert
{
    public const ZVVNMOD = 'zvvnmod';
    public const DELEHI = 'delehi';
    public const MENK_SHAPE = 'menk_shape';
    public const MENK_LETTER = 'menk_letter';
    public const Z52 = 'z52';
    /** Canonical UTN #57 Unicode. Output-only: valid as $to, rejected as $from. */
    public const UTN57 = 'utn57';

    private static ?\FFI $ffi = null;

    /**
     * Convert $input from one Mongolian encoding to another (UTF-8 in/out).
     *
     * @throws \RuntimeException on an unknown encoding name or an unsupported conversion.
     */
    public static function translate(string $from, string $to, string $input): string
    {
        $ffi = self::ffi();
        $ptr = $ffi->mongol_convert_translate($from, $to, $input);
        if (\FFI::isNull($ptr)) {
            throw new \RuntimeException("mongol-convert: translate failed (from=$from, to=$to)");
        }
        try {
            return \FFI::string($ptr);
        } finally {
            $ffi->mongol_convert_free($ptr);
        }
    }

    /** Native core version. */
    public static function version(): string
    {
        // PHP FFI auto-converts a `const char *` return into a PHP string; `char *` stays CData.
        $v = self::ffi()->mongol_convert_version();
        return \is_string($v) ? $v : \FFI::string($v);
    }

    private static function ffi(): \FFI
    {
        if (self::$ffi === null) {
            self::$ffi = \FFI::cdef(
                'char *mongol_convert_translate(const char *from, const char *to, const char *input);'
                . 'void mongol_convert_free(char *ptr);'
                . 'const char *mongol_convert_version(void);',
                self::libraryPath()
            );
        }
        return self::$ffi;
    }

    private static function libraryPath(): string
    {
        $env = getenv('MONGOL_CONVERT_LIB');
        if (is_string($env) && $env !== '') {
            return $env;
        }
        $ext = ['Darwin' => 'dylib', 'Windows' => 'dll'][\PHP_OS_FAMILY] ?? 'so';
        $arch = \in_array(php_uname('m'), ['arm64', 'aarch64'], true) ? 'aarch64' : 'x86_64';
        $os = strtolower(\PHP_OS_FAMILY);
        $bundled = __DIR__ . "/../prebuilt/$os-$arch/libmongol_convert.$ext";
        return is_file($bundled) ? $bundled : "libmongol_convert.$ext";
    }
}
