<?php

declare(strict_types=1);

// Verify the PHP FFI binding against the Java golden corpus.
// Run: MONGOL_CONVERT_LIB=/abs/path/to/libmongol_convert.dylib php -d ffi.enable=1 test/smoke.php

require __DIR__ . '/../src/MongolConvert.php';

use MongolConvert\MongolConvert;

$golden = __DIR__ . '/../../../crates/mongol-convert/tests/golden/golden.tsv';

function unesc(string $s): string
{
    return preg_replace_callback('/\\\\u([0-9a-fA-F]{4})/', static function (array $m): string {
        return mb_chr(hexdec($m[1]), 'UTF-8');
    }, $s);
}

$want = ['Z52|Menk_Shape', 'Menk_Shape|Z52', 'Delehi|Z52', 'Menk_Letter|Delehi', 'Delehi|Menk_Letter'];
$buckets = array_fill_keys($want, []);

foreach (file($golden, FILE_IGNORE_NEW_LINES) as $line) {
    if ($line === '') {
        continue;
    }
    [$f, $t, $i, $o] = explode("\t", $line);
    $k = "$f|$t";
    if (isset($buckets[$k]) && count($buckets[$k]) < 40) {
        $buckets[$k][] = [unesc($i), unesc($o)];
    }
}

$total = 0;
$ok = 0;
foreach ($buckets as $k => $rows) {
    [$f, $t] = explode('|', $k);
    $bad = 0;
    foreach ($rows as [$inp, $exp]) {
        $total++;
        try {
            $got = MongolConvert::translate(strtolower($f), strtolower($t), $inp);
        } catch (\Throwable $e) {
            $got = '<throw:' . $e->getMessage() . '>';
        }
        if ($got === $exp) {
            $ok++;
        } else {
            $bad++;
            if ($bad <= 1) {
                fwrite(STDERR, "  MISMATCH $k in=" . bin2hex($inp) . " exp=" . bin2hex($exp) . " got=" . bin2hex($got) . "\n");
            }
        }
    }
    printf("%-25s %d/%d ok\n", $k, count($rows) - $bad, count($rows));
}

printf("\nPHP FFI vs Java golden: %d/%d byte-exact\n", $ok, $total);
printf("version(): %s\n", MongolConvert::version());
exit($ok === $total ? 0 : 1);
