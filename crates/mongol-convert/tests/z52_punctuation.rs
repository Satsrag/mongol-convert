use mongol_convert::{translate, CodeType};

const UNICODE_PUNCTUATION: &str =
    "\u{00b7}\u{2048}\u{2049}!?;()\u{3008}\u{3009}\u{3014}\u{3015}\u{300a}\u{300b}\u{300e}\u{300f},\u{00d7}\u{203b}-|";
const Z52_PUNCTUATION: &str =
    "\u{184f}\u{1850}\u{1851}\u{1852}\u{1853}\u{1854}\u{1855}\u{1856}\u{1857}\u{1858}\u{1859}\u{185a}\u{185b}\u{185c}\u{185d}\u{185e}\u{185f}\u{1860}\u{1861}\u{1862}\u{1863}";

/// The narrow no-break space Z52 writes around its own punctuation. Its glyph in `z52.otf` is
/// 19.5 % of an em — exactly the bearing that font gives its own `᠄` and `᠁` — while the 21 marks
/// below carry none of their own, measuring 0.0–0.7 % on both sides.
const GAP: &str = "\u{202f}";

/// The 21 marks run together, each with the gap on the side it takes. Between `(` and `)`, or
/// `〈` and `〉`, there is none: the opener takes it before and the closer after, so nothing falls
/// between them.
const Z52_PUNCTUATION_SPACED: &str = "\u{184f}\u{202f}\u{1850}\u{202f}\u{1851}\u{202f}\u{1852}\u{202f}\u{1853}\u{202f}\u{1854}\u{202f}\u{1855}\u{1856}\u{202f}\u{1857}\u{1858}\u{202f}\u{1859}\u{185a}\u{202f}\u{185b}\u{185c}\u{202f}\u{185d}\u{185e}\u{202f}\u{185f}\u{1860}\u{1861}\u{1862}\u{1863}";

#[test]
fn strict_z52_output_encodes_all_21_punctuation_positions() {
    let out = translate(CodeType::Zvvnmod, CodeType::Z52, UNICODE_PUNCTUATION).unwrap();
    assert_eq!(out, Z52_PUNCTUATION_SPACED);
    // The 21 marks are all there, in order: only the gap is new.
    assert_eq!(out.replace(GAP, ""), Z52_PUNCTUATION);
}

#[test]
fn the_gap_lands_on_the_side_each_mark_takes() {
    let word = "\u{1830}\u{1820}\u{1836}\u{1822}\u{1828}"; // ᠰᠠᠶᠢᠨ
    let z52_word = translate(CodeType::MenkLetter, CodeType::Z52, word).unwrap();
    let spell = |mark: char| {
        translate(
            CodeType::MenkLetter,
            CodeType::Z52,
            &format!("{word}{mark}{word}"),
        )
        .unwrap()
    };
    // Before: ! ? ; ( 〈 〔 《 『 , ⁈ ⁉
    for (mark, z52) in [
        ('!', '\u{1852}'),
        ('?', '\u{1853}'),
        (';', '\u{1854}'),
        ('(', '\u{1855}'),
        ('\u{3008}', '\u{1857}'),
        ('\u{3014}', '\u{1859}'),
        ('\u{300a}', '\u{185b}'),
        ('\u{300e}', '\u{185d}'),
        (',', '\u{185f}'),
        ('\u{2048}', '\u{1850}'),
        ('\u{2049}', '\u{1851}'),
    ] {
        assert_eq!(
            spell(mark),
            format!("{z52_word}{GAP}{z52}{z52_word}"),
            "{mark:?}"
        );
    }
    // After: the five closing brackets.
    for (mark, z52) in [
        (')', '\u{1856}'),
        ('\u{3009}', '\u{1858}'),
        ('\u{3015}', '\u{185a}'),
        ('\u{300b}', '\u{185c}'),
        ('\u{300f}', '\u{185e}'),
    ] {
        assert_eq!(
            spell(mark),
            format!("{z52_word}{z52}{GAP}{z52_word}"),
            "{mark:?}"
        );
    }
    // Both: the middle dot.
    let middle_dot = '\u{184f}';
    assert_eq!(
        spell('\u{00b7}'),
        format!("{z52_word}{GAP}{middle_dot}{GAP}{z52_word}")
    );
    // None: × ※ - |, whose reference bearings are themselves ~0.
    for (mark, z52) in [
        ('\u{00d7}', '\u{1860}'),
        ('\u{203b}', '\u{1861}'),
        ('-', '\u{1862}'),
        ('|', '\u{1863}'),
    ] {
        assert_eq!(
            spell(mark),
            format!("{z52_word}{z52}{z52_word}"),
            "{mark:?}"
        );
    }
}

#[test]
fn the_ten_mongolian_marks_z52_passes_through_keep_the_bearing_they_have() {
    // ᠂ measures 18.3 % em on the left in `z52.otf`, ᠄ 19.5 %: the gap is already drawn in.
    let word = "\u{1830}\u{1820}\u{1836}\u{1822}\u{1828}";
    let z52_word = translate(CodeType::MenkLetter, CodeType::Z52, word).unwrap();
    for mark in [
        '\u{1800}', '\u{1801}', '\u{1802}', '\u{1803}', '\u{1804}', '\u{1805}', '\u{1806}',
        '\u{1807}', '\u{1808}', '\u{1809}',
    ] {
        assert_eq!(
            translate(
                CodeType::MenkLetter,
                CodeType::Z52,
                &format!("{word}{mark}{word}")
            )
            .unwrap(),
            format!("{z52_word}{mark}{z52_word}"),
            "{mark:?}"
        );
    }
}

#[test]
fn the_gap_is_removed_again_on_the_way_back() {
    let word = "\u{1830}\u{1820}\u{1836}\u{1822}\u{1828}";
    for mark in ['!', '(', ')', '\u{00b7}', ',', '-', '\u{1802}'] {
        let source = format!("{word}{mark}{word}");
        let hub = translate(CodeType::MenkLetter, CodeType::Zvvnmod, &source).unwrap();
        let z52 = translate(CodeType::MenkLetter, CodeType::Z52, &source).unwrap();
        assert_eq!(
            translate(CodeType::Z52, CodeType::Zvvnmod, &z52).unwrap(),
            hub,
            "{mark:?}"
        );
    }
}

#[test]
fn z52_input_decodes_all_21_punctuation_positions() {
    assert_eq!(
        translate(CodeType::Z52, CodeType::Zvvnmod, Z52_PUNCTUATION).unwrap(),
        UNICODE_PUNCTUATION
    );
}

#[test]
fn strict_z52_punctuation_round_trips() {
    let encoded = translate(CodeType::Zvvnmod, CodeType::Z52, UNICODE_PUNCTUATION).unwrap();
    assert_eq!(
        translate(CodeType::Z52, CodeType::Zvvnmod, &encoded).unwrap(),
        UNICODE_PUNCTUATION
    );
}

#[test]
fn existing_mongolian_punctuation_remains_unicode() {
    let punctuation = "\u{1801}\u{1802}\u{1803}\u{1804}";
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Z52, punctuation).unwrap(),
        punctuation
    );
}

#[test]
fn todo_and_sibe_letters_are_not_reinterpreted_outside_z52_routes() {
    let encodings = [
        CodeType::Zvvnmod,
        CodeType::Delehi,
        CodeType::MenkShape,
        CodeType::MenkLetter,
    ];

    for source in encodings {
        for target in encodings {
            assert_eq!(
                translate(source, target, Z52_PUNCTUATION).unwrap(),
                Z52_PUNCTUATION,
                "{source:?} -> {target:?} must preserve Unicode TODO/SIBE letters"
            );
        }
    }
}

#[test]
fn strict_z52_punctuation_preserves_mixed_text_and_whitespace() {
    let input = "EN  \u{00b7};\t(中🙂)\r\n\u{1802}\u{1803}  ";
    // "EN  " loses one space to the gap; the tab and the CR already separate their marks.
    let expected =
        "EN \u{202f}\u{184f}\u{202f}\u{1854}\t\u{1855}中🙂\u{1856}\r\n\u{1802}\u{1803}  ";
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Z52, input).unwrap(),
        expected
    );
}

#[test]
fn unicode_like_sources_use_strict_z52_punctuation_output() {
    for source in [CodeType::Zvvnmod, CodeType::Delehi, CodeType::MenkLetter] {
        assert_eq!(
            translate(source, CodeType::Z52, UNICODE_PUNCTUATION).unwrap(),
            Z52_PUNCTUATION_SPACED,
            "source {source:?}"
        );
    }
}
