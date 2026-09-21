<?php

declare(strict_types=1);

/*
 * How to use the mongol-convert PHP package.
 *
 * In your project, after `composer require zvvnmod/mongol-convert`:
 *     require 'vendor/autoload.php';
 *     use MongolConvert\MongolConvert;
 *     echo MongolConvert::translate(MongolConvert::Z52, MongolConvert::MENK_SHAPE, $input);
 *
 * This standalone example just requires the class directly so it runs from the repo.
 */

require __DIR__ . '/src/MongolConvert.php';

use MongolConvert\MongolConvert;

echo "mongol-convert core version: " . MongolConvert::version() . "\n\n";

/** Render a string as code points so it's visible regardless of terminal font. */
function cps(string $s): string
{
    if ($s === '') {
        return '(empty)';
    }
    $out = [];
    foreach (preg_split('//u', $s, -1, PREG_SPLIT_NO_EMPTY) as $ch) {
        $out[] = sprintf('U+%04X', mb_ord($ch, 'UTF-8'));
    }
    return implode(' ', $out);
}

// Three real Delehi (standard Unicode) Mongolian words from the test corpus.
$words = array_slice(
    array_values(array_filter(array_map('trim', file(__DIR__ . '/../../crates/mongol-convert/tests/golden/corpus_delehi.txt')))),
    0,
    3
);

foreach ($words as $n => $w) {
    echo "── word " . ($n + 1) . " ──────────────────────────────────────────\n";
    echo "delehi (Unicode) in : " . cps($w) . "\n";

    // Convert the standard-Unicode word into each encoding.
    $z52  = MongolConvert::translate(MongolConvert::DELEHI, MongolConvert::Z52, $w);
    $menk = MongolConvert::translate(MongolConvert::DELEHI, MongolConvert::MENK_LETTER, $w);

    echo "  -> z52             : " . cps($z52) . "\n";
    echo "  -> menk_letter     : " . cps($menk) . "\n";

    // The urgent path you asked about: zcode (Z52) <-> menk_shape.
    $shape = MongolConvert::translate(MongolConvert::Z52, MongolConvert::MENK_SHAPE, $z52);
    echo "  z52 -> menk_shape  : " . cps($shape) . "   <-- zcode↔menk_shape\n";

    // Round-trip back to Unicode and check it matches the input.
    $back = MongolConvert::translate(MongolConvert::Z52, MongolConvert::DELEHI, $z52);
    echo "  z52 -> delehi (back): " . cps($back) . "  [" . ($back === $w ? "round-trip OK" : "differs") . "]\n\n";
}
