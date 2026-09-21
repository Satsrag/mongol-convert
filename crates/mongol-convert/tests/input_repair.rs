use mongol_convert::{
    translate, translate_with_options, translate_with_warnings, CodeType, MongolConvertError,
    TranslationOptions, Warning,
};

const REPAIR: TranslationOptions = TranslationOptions {
    repair_suffix_separators: true,
    restore_menk_shape_emoji: true,
};
const RAW: &str = "ᠤᠯᠤᠰ ᠤᠨ";
const FIXED: &str = "ᠤᠯᠤᠰ\u{202F}ᠤᠨ";

#[test]
fn audited_reflexives_match_manual_repairs_across_targets() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (raw, fixed, shape, edits) in [
            ("ᠨᠣᠮ ᠢᠶᠠᠨ", "ᠨᠣᠮ\u{202F}ᠢᠶᠠᠨ", "NOMMvsIIAA", 1),
            ("ᠭᠡᠷ\u{A0}ᠢᠶᠡᠨ", "ᠭᠡᠷ\u{202F}ᠢᠶᠡᠨ", "GARMvsIIAA", 1),
        ] {
            for to in [
                CodeType::MenkLetter,
                CodeType::Delehi,
                CodeType::MenkShape,
                CodeType::Zvvnmod,
                CodeType::Z52,
                CodeType::Utn57,
                CodeType::Utn57Shape,
            ] {
                let result = translate_with_options(from, to, raw, &REPAIR).unwrap();
                assert_eq!(result.text, translate(from, to, fixed).unwrap());
                assert_eq!(result.warnings.len(), edits);
                if to == CodeType::Utn57Shape {
                    assert_eq!(result.text, shape);
                }
            }
            let again = translate_with_options(from, from, fixed, &REPAIR).unwrap();
            assert_eq!(again.text, fixed);
            assert!(again.warnings.is_empty());
        }
        let chain = translate_with_options(from, from, "ᠤᠯᠤᠰ ᠤᠨ ᠢᠶᠠᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ\u{202F}ᠢᠶᠠᠨ");
        assert_eq!(chain.warnings.len(), 2);
        let existing = translate_with_options(from, from, "ᠭᠡᠷ\u{202F}ᠦᠨ ᠢᠶᠡᠨ", &REPAIR).unwrap();
        assert_eq!(existing.text, "ᠭᠡᠷ\u{202F}ᠦᠨ\u{202F}ᠢᠶᠡᠨ");
        assert_eq!(existing.warnings.len(), 1);
    }
}

#[test]
fn vowel_harmony_and_stem_controls_are_not_checked() {
    // Rendering after NNBSP depends only on the suffix letters, so the written harmony of the
    // suffix is kept as it is, whatever the stem.
    let cases = [
        ("ᠭᠡᠷ", "ᠢᠶᠠᠨ"),
        ("ᠨᠣᠮ", "ᠢᠶᠡᠨ"),
        ("ᠬᠣᠲᠠ", "ᠢᠶᠠᠨ"),
        ("ᠪᠢᠴᠢᠭ", "ᠢᠶᠡᠨ"),
        ("ᠨᠣᠡᠮ", "ᠢᠶᠠᠨ"),
        ("ᠨᠣᠮ", "ᠯᠦᠭᠡ"),
        ("ᠭᠡᠷ", "ᠶᠤᠭᠠᠨ"),
        ("ᠪᠢᠴᠢᠭ", "ᠲᠠᠢ"),
        ("ᠭᠡᠷ", "ᠲᠠᠬᠢ"),
        ("ᠨᠡ\u{180E}ᠷᠡ", "ᠶᠦᠭᠡᠨ"),
        ("ᠤᠨᠠᠭ\u{180E}ᠠ", "ᠢᠶᠠᠨ"),
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix) in cases {
            assert_single_repair(from, stem, suffix);
        }
        for raw in ["ᠨᠣᠮ\u{180B} ᠢᠶᠠᠨ", "ᠭᠡᠷ\u{200D} ᠢᠶᠡᠨ", "ᠮᠠᠯ\u{200D} ᠤᠳ"]
        {
            let fixed = raw.replace(' ', "\u{202F}");
            let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
            assert_eq!(result.text, translate(from, from, &fixed).unwrap());
            assert_eq!(
                result.warnings.len(),
                1 + translate_with_warnings(from, from, &fixed)
                    .unwrap()
                    .warnings
                    .len()
            );
        }
    }
}

#[test]
fn reflexive_repair_declines_longer_words_and_layout() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for raw in [
            "ᠨᠣᠮ ᠢᠶᠠᠨ\u{180B}",
            "ᠭᠡᠷ ᠢᠶᠡᠨᠡ",
            "ᠨᠣᠮ ᠢᠶᠠᠨx",
            "ᠨᠣᠮᠢᠶᠠᠨ",
            "ᠭᠡᠷ\nᠢᠶᠡᠨ",
            "ᠢᠶᠠᠨ",
            "ᠢᠶᠡᠨ",
        ] {
            let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
            assert_eq!(result.text, raw);
            assert!(result.warnings.is_empty(), "{raw:?}");
        }
    }
}

#[test]
fn comitative_and_plural_repairs_preserve_spelling_across_targets() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        // The ger examples also occur with NNBSP in the existing Delehi golden corpus.
        // Plural recognition must not require a vowel-final stem: ger ends in r.
        for raw in [
            "ᠭᠡᠷ ᠯᠦᠭᠡ",
            "ᠭᠡᠷ ᠨᠦᠭᠦᠳ",
            "ᠨᠣᠮ ᠯᠤᠭ\u{180E}ᠠ",
            "ᠣᠶᠤᠲᠠᠨ ᠨᠤᠭᠤᠳ",
            "ᠬᠣᠲᠠ ᠨᠤᠭᠤᠳ",
            "ᠡᠭᠡᠴᠢ ᠨᠦᠭᠦᠳ",
        ] {
            let fixed = raw.replace(' ', "\u{202F}");
            for separator in [' ', '\u{A0}'] {
                let input = raw.replace(' ', &separator.to_string());
                for to in [
                    CodeType::MenkLetter,
                    CodeType::Delehi,
                    CodeType::MenkShape,
                    CodeType::Zvvnmod,
                    CodeType::Z52,
                    CodeType::Utn57,
                    CodeType::Utn57Shape,
                ] {
                    let result = translate_with_options(from, to, &input, &REPAIR).unwrap();
                    assert_eq!(result.text, translate(from, to, &fixed).unwrap());
                    assert_eq!(
                        result.warnings,
                        vec![Warning::RepairedSuffixSeparator {
                            byte_offset: input.find(separator).unwrap(),
                            original: separator,
                        }]
                    );
                }
            }
            let again = translate_with_options(from, from, &fixed, &REPAIR).unwrap();
            assert_eq!(again.text, fixed);
            assert!(again.warnings.is_empty());
        }
        let chain = translate_with_options(from, from, "ᠭᠡᠷ ᠨᠦᠭᠦᠳ ᠦᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠭᠡᠷ\u{202F}ᠨᠦᠭᠦᠳ\u{202F}ᠦᠨ");
        assert_eq!(chain.warnings.len(), 2);
    }
}

#[test]
fn comitative_and_plural_repair_respects_boundaries() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix) in [
            ("ᠭᠡᠷ", "ᠯᠦᠭᠡ"),
            ("ᠭᠡᠷ", "ᠨᠦᠭᠦᠳ"),
            ("ᠨᠣᠮ", "ᠯᠤᠭ\u{180E}ᠠ"),
            ("ᠬᠣᠲᠠ", "ᠨᠤᠭᠤᠳ"),
        ] {
            for raw in [
                suffix.to_owned(),
                format!("{stem} {suffix}\u{180B}"),
                format!("{stem} {suffix}ᠡ"),
                format!("{stem} {suffix}x"),
                format!("{stem}{suffix}"),
                format!("{stem}  {suffix}"),
                format!("{stem}\n{suffix}"),
            ] {
                let result = translate_with_options(from, from, &raw, &REPAIR).unwrap();
                assert_eq!(result.text, raw);
                assert!(result.warnings.is_empty(), "{raw:?}");
            }
        }
        // The independent word nüküd has QA, not the GA of the plural nügüd.
        let raw = "ᠭᠡᠷ ᠨᠦᠬᠦᠳ";
        let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
        assert_eq!(result.text, raw);
        assert!(result.warnings.is_empty());
    }
}

#[test]
fn remaining_audited_suffix_families_match_manual_nnbsp() {
    let cases = [
        ("ᠮᠠᠯ", "ᠤᠳ"),
        ("ᠭᠡᠷ", "ᠦᠳ"),
        ("ᠪᠠᠭᠰᠢ", "ᠳᠠᠭᠠᠨ"),
        ("ᠡᠭᠡᠴᠢ", "ᠳᠡᠭᠡᠨ"),
        ("ᠤᠯᠤᠰ", "ᠲᠠᠭᠠᠨ"),
        ("ᠭᠡᠷ", "ᠲᠡᠭᠡᠨ"),
        ("ᠨᠣᠮ", "ᠶᠤᠭᠠᠨ"),
        ("ᠭᠡᠷ", "ᠶᠦᠭᠡᠨ"),
        ("ᠨᠣᠮ", "ᠠᠴᠠᠭᠠᠨ"),
        ("ᠭᠡᠷ", "ᠡᠴᠡᠭᠡᠨ"),
        ("ᠨᠣᠮ", "ᠳᠤᠨᠢ"),
        ("ᠡᠭᠡᠴᠢ", "ᠳᠦᠨᠢ"),
        ("ᠤᠯᠤᠰ", "ᠲᠤᠨᠢ"),
        ("ᠭᠡᠷ", "ᠲᠦᠨᠢ"),
        ("ᠬᠣᠲᠠ", "ᠳᠠᠬᠢ"),
        ("ᠭᠡᠷ", "ᠳᠡᠬᠢ"),
        ("ᠨᠡᠷ\u{180E}ᠡ", "ᠶᠦᠭᠡᠨ"), // existing golden-corpus example
        ("ᠤᠨᠠᠭ\u{180E}ᠠ", "ᠨᠤᠭᠤᠳ"),
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix) in cases {
            for separator in [' ', '\u{A0}'] {
                // Leading non-ASCII text makes warning offsets differ from character counts.
                let raw = format!("例：{stem}{separator}{suffix}᠃");
                let fixed = format!("例：{stem}\u{202F}{suffix}᠃");
                for to in [
                    CodeType::MenkLetter,
                    CodeType::Delehi,
                    CodeType::MenkShape,
                    CodeType::Zvvnmod,
                    CodeType::Z52,
                    CodeType::Utn57,
                    CodeType::Utn57Shape,
                ] {
                    let result = translate_with_options(from, to, &raw, &REPAIR).unwrap();
                    assert_eq!(result.text, translate(from, to, &fixed).unwrap(), "{raw}");
                    assert_eq!(
                        result.warnings,
                        vec![Warning::RepairedSuffixSeparator {
                            byte_offset: "例：".len() + stem.len(),
                            original: separator,
                        }]
                    );
                    assert_eq!(
                        translate_with_options(from, to, &raw, &TranslationOptions::default())
                            .unwrap(),
                        translate_with_warnings(from, to, &raw).unwrap(),
                    );
                }
                let again = translate_with_options(from, from, &fixed, &REPAIR).unwrap();
                assert_eq!(again.text, fixed);
                assert!(again.warnings.is_empty());
            }
            for raw in [
                format!("{stem} {suffix}ᠡ"), // exact suffix, not a longer word's prefix
                format!("{stem} {suffix}\u{180B}"),
                format!("{stem}\n{suffix}"),
                format!("{stem}  {suffix}"),
            ] {
                let result = translate_with_options(from, from, &raw, &REPAIR).unwrap();
                assert_eq!(result.text, raw);
                assert!(result.warnings.is_empty(), "{raw}");
            }
        }
        let chain = translate_with_options(from, from, "ᠮᠠᠯ ᠤᠳ ᠠᠴᠠᠭᠠᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠮᠠᠯ\u{202F}ᠤᠳ\u{202F}ᠠᠴᠠᠭᠠᠨ");
        assert_eq!(chain.warnings.len(), 2);
        // Independent dictionary phrase, not an ordinal.
        assert_unchanged(from, "ᠡᠨᠡ ᠳᠤᠭᠠᠷ ᠵᠠᠶᠢᠰᠠᠩ");
    }
}

#[test]
fn particle_shaping_support_does_not_automatically_enable_repair() {
    // Deliberately deferred entries from the pinned Hudum particle mapping. This is an
    // exclusion regression, not a claim that these constructed phrases are grammatical.
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for particle in [
            "ᠤᠤ",
            "ᠦᠦ",
            "ᠪᠦᠦ",
            "ᠠ",
            "ᠡ",
            "ᠴᠤ",
            "ᠴᠦ",
            "ᠨᠦᠭᠡᠨ",
            "ᠶᠦᠮ",
            "ᠶᠦᠮᠰᠡᠨ",
            "ᠬᠦ",
            "ᠳᠠᠭ",
            "ᠳᠡᠭ",
            "ᠳᠤᠭᠠᠷ",
            "ᠳᠦᠭᠡᠷ",
            "ᠳᠠ",
            "ᠳᠡ",
        ] {
            for stem in ["ᠨᠣᠮ", "ᠭᠡᠷ"] {
                let raw = format!("{stem} {particle}");
                let result = translate_with_options(from, from, &raw, &REPAIR).unwrap();
                assert_eq!(result.text, raw);
                assert!(result.warnings.is_empty(), "{particle}");
            }
        }
    }
}

const TARGETS: [CodeType; 7] = [
    CodeType::MenkLetter,
    CodeType::Delehi,
    CodeType::MenkShape,
    CodeType::Zvvnmod,
    CodeType::Z52,
    CodeType::Utn57,
    CodeType::Utn57Shape,
];

fn assert_single_repair(from: CodeType, before: &str, suffix: &str) {
    for separator in [' ', '\u{A0}'] {
        let raw = format!("例：{before}{separator}{suffix}᠃");
        let fixed = format!("例：{before}\u{202F}{suffix}᠃");
        for to in TARGETS {
            let result = translate_with_options(from, to, &raw, &REPAIR).unwrap();
            assert_eq!(result.text, translate(from, to, &fixed).unwrap(), "{raw}");
            assert_eq!(
                result.warnings,
                vec![Warning::RepairedSuffixSeparator {
                    byte_offset: "例：".len() + before.len(),
                    original: separator,
                }],
                "{raw}"
            );
        }
        let again = translate_with_options(from, from, &fixed, &REPAIR).unwrap();
        assert_eq!(again.text, fixed);
        assert!(again.warnings.is_empty());
    }
}

fn assert_unchanged(from: CodeType, raw: &str) {
    let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
    assert_eq!(result.text, raw);
    assert!(result.warnings.is_empty(), "{raw:?}");
}

#[test]
fn bar_ban_tai_and_nar_follow_the_stems_that_take_them() {
    // Delehi golden corpus: mori ᠪᠡᠷ, nidü ᠪᠠᠨ and ken ᠲᠠᠢ, whatever the stem's harmony.
    let cases = [
        ("ᠬᠣᠲᠠ", "ᠪᠠᠷ"),
        ("ᠮᠣᠷᠢ", "ᠪᠡᠷ"),
        ("ᠡᠭᠡᠴᠢ", "ᠪᠡᠷ"),
        ("ᠨᠡᠷ\u{180E}ᠡ", "ᠪᠡᠷ"),
        ("ᠬᠣᠲᠠ", "ᠪᠠᠨ"),
        ("ᠨᠢᠳᠦ", "ᠪᠠᠨ"),
        ("ᠡᠭᠡᠴᠢ", "ᠪᠡᠨ"),
        ("ᠪᠠᠭᠰᠢ", "ᠲᠠᠢ"),
        ("ᠬᠡᠨ", "ᠲᠠᠢ"),
        ("ᠨᠣᠮ", "ᠲᠠᠢ"),
        ("ᠭᠡᠷ", "ᠲᠡᠢ"),
        ("ᠪᠠᠭᠱᠢ", "ᠨᠠᠷ"),
        ("ᠡᠭᠡᠴᠢ", "ᠨᠡᠷ"),
        ("ᠡᠵᠡᠨ", "ᠨᠡᠷ"),
        ("ᠪᠢᠴᠢ", "ᠪᠡᠷ"),
        ("ᠰᠠᠶ", "ᠪᠠᠷ"), // final ᠶ is a diphthong
        ("ᠬᠣᠲᠠ\u{200D}", "ᠪᠠᠷ"),
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix) in cases {
            assert_single_repair(from, stem, suffix);
        }
        let chain = translate_with_options(from, from, "ᠪᠠᠭᠰᠢ ᠨᠠᠷ ᠲᠠᠢ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠪᠠᠭᠰᠢ\u{202F}ᠨᠠᠷ\u{202F}ᠲᠠᠢ");
        assert_eq!(chain.warnings.len(), 2);
        for raw in [
            "ᠴᠠᠭᠠᠨ ᠪᠠᠷ", // consonant-final: "white tiger", not the instrumental
            "ᠨᠣᠮ ᠪᠡᠷ",
            "ᠨᠣᠮ ᠪᠠᠨ",
            "ᠬᠣᠲᠠ ᠪᠠᠷᠰ",
            "ᠬᠣᠲᠠ ᠪᠠᠷ\u{180B}",
            "ᠬᠣᠲᠠ  ᠪᠠᠷ",
            "ᠬᠣᠲᠠ\nᠪᠠᠷ",
            "ᠪᠠᠭᠰᠢ ᠲᠠᠢᠢ",
            "ᠪᠠᠭᠰᠢ ᠨᠠᠷ\u{180E}ᠠ",
            "ᠲᠠᠢ",
        ] {
            assert_unchanged(from, raw);
        }
    }
}

#[test]
fn t_initial_suffixes_follow_the_consonants_that_select_them() {
    let cases = [
        ("ᠡᠴᠦᠰ", "ᠲᠡᠬᠡᠨ"), // feminine g and k share ink; this is the corpus spelling of tegen
        ("ᠭᠡᠷ", "ᠲᠡᠭᠡᠨ"),
        ("ᠭᠠᠵᠠᠷ", "ᠲᠠᠬᠢ"),
        ("ᠭᠡᠷ", "ᠲᠡᠬᠢ"),
        ("ᠪᠢᠴᠢᠭ", "ᠲᠦ"),
        ("ᠤᠯᠤᠰ", "ᠲᠤ"),
        ("ᠭᠡᠷ", "ᠲᠦᠷ"),
        ("ᠠᠪ", "ᠲᠤᠷ"),
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix) in cases {
            assert_single_repair(from, stem, suffix);
        }
        // A selector does not hide the final consonant.
        let selected = translate_with_options(from, from, "ᠭᠡᠷ\u{180B} ᠲᠦ", &REPAIR).unwrap();
        assert_eq!(selected.text, "ᠭᠡᠷ\u{180B}\u{202F}ᠲᠦ");
        for raw in [
            "ᠪᠤᠶᠤ ᠲᠦᠷ", // the independent word tür, not the dative
            "ᠡᠭᠡᠴᠢ ᠲᠦᠷ",
            "ᠨᠡᠷ\u{180E}ᠡ ᠲᠦᠷ",
            "ᠬᠥᠮᠦᠨ ᠲᠦᠷ",
            "ᠡᠭᠡᠴᠢ ᠲᠦ",
            "ᠣᠶᠤᠲᠠᠨ ᠲᠤ",
            "ᠬᠣᠲᠠ ᠲᠠᠬᠢ",
            "ᠬᠣᠲᠠ ᠲᠠᠭᠠᠨ",
            "ᠨᠣᠮ ᠲᠤᠨᠢ",
            "ᠨᠣᠮ ᠲᠡᠬᠡᠨ",
            "ᠡᠴᠦᠰ ᠲᠡᠬᠡᠨᠡ",
        ] {
            assert_unchanged(from, raw);
        }
    }
}

#[test]
fn any_word_bracket_or_symbol_can_take_a_suffix() {
    // Corpus: APP ᠪᠡᠷ, DEX ᠲᠠᠢ, ︾ ᠶᠢᠨ, 60° ᠡᠴᠡ, 600℃ ᠲᠦ. No stem rule applies, since the
    // writer's spelling after such a token is the only evidence.
    let cases = [
        ("APP", "ᠪᠡᠷ"),
        ("DEX", "ᠲᠠᠢ"),
        ("iPhone", "ᠶᠢ"),
        ("MP3", "ᠪᠡᠷ"),
        ("ᠨᠣᠮ3", "ᠤ"),
        ("x3", "ᠤ"),
        ("A", "ᠳᠤᠭᠠᠷ"),
        ("中文", "ᠤᠨ"),
        ("︽ᠨᠣᠮ︾", "ᠶᠢᠨ"),
        ("（ᠨᠣᠮ）", "ᠲᠦᠷ"),
        ("\"ᠨᠣᠮ\"", "ᠢ"),
        ("60°", "ᠡᠴᠡ"),
        ("600℃", "ᠲᠦ"),
        ("99％", "ᠡᠴᠡ"),
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (before, suffix) in cases {
            assert_single_repair(from, before, suffix);
        }
        for raw in [
            "ᠨᠣᠮ᠂ ᠢ", // sentence punctuation
            "ᠨᠣᠮ᠃ ᠲᠦᠷ",
            "ᠨᠣᠮ, ᠤᠨ",
            "ᠨᠣᠮ ︽ ᠤᠨ", // opening bracket
            "ᠨᠣᠮ ( ᠤᠨ",
            " ᠤᠨ", // line start
            "ᠨᠣᠮ\t ᠤᠨ",
            "\u{200D} ᠤᠨ", // a control alone is not a token
        ] {
            assert_unchanged(from, raw);
        }
    }
}

#[test]
fn number_context_repairs_case_suffixes_and_ordinals() {
    let cases = [
        ("25", "ᠤ"),
        ("2019", "ᠤᠨ"),
        ("᠒᠐", "ᠶᠢᠨ"),
        ("3.5", "ᠢ"),
        ("10", "ᠳᠤ"),
        ("2", "ᠲᠦ"),
        ("7", "ᠡᠴᠡ"),
        ("5", "ᠪᠡᠷ"),
        ("95583", "ᠲᠠᠢ"),
        ("1", "ᠳᠡᠬᠢ"),
        ("3", "ᠳᠤᠭᠠᠷ"),
        ("᠑", "ᠳᠦᠭᠡᠷ"),
        ("12", "ᠳᠦᠭᠡᠷ"), // writer's choice of spelling is kept, not recomputed
        ("3", "ᠤᠳ"),
        ("3", "ᠨᠠᠷ"),
        ("3", "ᠪᠠᠨ"),
        ("3", "ᠲᠦᠷ"), // no stem to check after a number
    ];
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (number, suffix) in cases {
            assert_single_repair(from, number, suffix);
        }
        let chain = translate_with_options(from, from, "3 ᠳᠤᠭᠠᠷ ᠤᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "3\u{202F}ᠳᠤᠭᠠᠷ\u{202F}ᠤᠨ");
        assert_eq!(chain.warnings.len(), 2);
        for raw in [
            "3 ᠳ\u{180B}ᠤᠭᠠᠷ", // FVS spelling already renders; left alone
            "3 ᠳᠤᠭᠠᠷᠠ",
            "3  ᠤ",
            "3\nᠤ",
            "3\u{202F}ᠤ",
            "ᠡᠨᠡ ᠳᠤᠭᠠᠷ",
            "ᠨᠢᠭᠡ ᠳᠦᠭᠡᠷ", // numeral words take attached ordinals
            "ᠭᠤᠷᠪᠠᠨ ᠳᠤᠭᠠᠷ",
        ] {
            assert_unchanged(from, raw);
        }
    }
}

#[test]
fn repair_precedes_decoding_for_every_target() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for to in [
            CodeType::MenkLetter,
            CodeType::Delehi,
            CodeType::MenkShape,
            CodeType::Zvvnmod,
            CodeType::Z52,
            CodeType::Utn57,
            CodeType::Utn57Shape,
        ] {
            let result = translate_with_options(from, to, RAW, &REPAIR).unwrap();
            assert_eq!(
                result.text,
                translate(from, to, FIXED).unwrap(),
                "{from:?} -> {to:?}"
            );
            assert_eq!(
                result.warnings[0],
                Warning::RepairedSuffixSeparator {
                    byte_offset: "ᠤᠯᠤᠰ".len(),
                    original: ' ',
                }
            );
            assert_eq!(
                translate_with_options(from, to, RAW, &TranslationOptions::default()).unwrap(),
                translate_with_warnings(from, to, RAW).unwrap(),
            );
        }
    }
    assert_ne!(
        translate(CodeType::MenkLetter, CodeType::Zvvnmod, RAW).unwrap(),
        translate(CodeType::MenkLetter, CodeType::Zvvnmod, FIXED).unwrap(),
    );
}

#[test]
fn repairs_suffix_chains_and_reports_original_byte_offsets() {
    let raw = "中文: ᠤᠯᠤᠰ\u{A0}ᠤᠨ ᠢᠶᠠᠷ᠃\nᠭᠡᠷ ᠦᠨ";
    let result =
        translate_with_options(CodeType::MenkLetter, CodeType::MenkLetter, raw, &REPAIR).unwrap();
    assert_eq!(
        result.text,
        "中文: ᠤᠯᠤᠰ\u{202F}ᠤᠨ\u{202F}ᠢᠶᠠᠷ᠃\nᠭᠡᠷ\u{202F}ᠦᠨ"
    );
    let offsets = [
        raw.find('\u{A0}').unwrap(),
        raw.find(" ᠢᠶᠠᠷ").unwrap(),
        raw.find(" ᠦᠨ").unwrap(),
    ];
    for (warning, offset) in result.warnings.iter().zip(offsets) {
        assert!(
            matches!(warning, Warning::RepairedSuffixSeparator { byte_offset, .. } if *byte_offset == offset)
        );
    }
    assert_eq!(result.warnings.len(), 3);
    let again = translate_with_options(
        CodeType::MenkLetter,
        CodeType::MenkLetter,
        &result.text,
        &REPAIR,
    )
    .unwrap();
    assert_eq!(again.text, result.text);
    assert!(again.warnings.is_empty());
}

#[test]
fn preserves_layout_existing_controls_and_unrecognised_words() {
    for raw in [
        "",
        "  \n",
        "ᠤᠨ",
        " ᠤᠨ",
        "ᠤᠯᠤᠰ\nᠤᠨ",
        "ᠤᠯᠤᠰ\tᠤᠨ",
        "ᠤᠯᠤᠰ  ᠤᠨ",
        "ᠤᠯᠤᠰ\u{A0}\u{A0}ᠤᠨ",
        "ᠤᠯᠤᠰᠤᠨ",
        "ᠤᠯᠤᠰ ᠤᠨᠠᠭ᠎ᠠ",
        "ᠤᠯᠤᠰ ᠤᠨ\u{180B}",
        "ᠤᠯᠤᠰ ᠤᠨ\u{200D}",
        "ᠤᠯᠤᠰ\u{202F}ᠤᠨ",
        "ᠤᠯᠤᠰ\u{180E}ᠤᠨ",
        "ᠤᠯᠤᠰ\u{202F}\u{202F}ᠤᠨ",
        "ᠴᠠᠭᠠᠨ ᠪᠠᠷ",
        "ᠤᠯᠤᠰ ᠤᠨLatin",
        "ᠤᠯᠤᠰ ᠤᠨ_abc",
        "ᠤᠯᠤᠰ, ᠤᠨ",
    ] {
        let result =
            translate_with_options(CodeType::Delehi, CodeType::Delehi, raw, &REPAIR).unwrap();
        assert_eq!(result.text, raw);
        assert!(result.warnings.is_empty(), "{raw:?}");
    }
}

#[test]
fn rejects_repair_for_encodings_whose_suffix_spellings_are_not_supported() {
    for from in [
        CodeType::Utn57,
        CodeType::Utn57Shape,
        CodeType::MenkShape,
        CodeType::Z52,
        CodeType::Zvvnmod,
        CodeType::Oyun,
    ] {
        assert_eq!(
            translate_with_options(from, from, RAW, &REPAIR),
            Err(MongolConvertError::UnsupportedInputRepair(from))
        );
    }
}
