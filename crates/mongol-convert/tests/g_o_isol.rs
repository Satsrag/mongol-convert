//! ᠬᠦ as a whole word — the case from Satsrag/mongol-convert#32.
//!
//! ZVVNMOD had no `G i O f` glyph: every other bowed consonant carries both the word-initial and
//! the medial ligature with a final O, G only the medial one (`E09C`), so the Java tables reached
//! for it at the start of a word too. The UTN #57 encoder reads the hub by position, saw a medial
//! glyph with nothing to its left, and invented a ZWJ. The hub now has `E096` for the word-initial
//! ligature; the legacy tables never see it, since the ink is the one they already write for
//! `E09C`.

use mongol_convert::{translate, translate_with_warnings, CodeType, Warning};

const HUB_G_O_ISOL: &str = "\u{E096}";
const HUB_G_O_FINA: &str = "\u{E09C}";
/// ᠭ FVS2 ᠥ FVS2 — `G:init O:fina`, pinned the way mongol-norm spells it, and no ZWJ.
const UTN57_G_O_ISOL: &str = "\u{182D}\u{180C}\u{1825}\u{180C}";

#[test]
fn a_whole_word_reaches_the_hub_as_the_initial_ligature_from_every_source() {
    let sources = [
        (CodeType::MenkLetter, "\u{182C}\u{1826}"), // ᠬᠦ
        (CodeType::MenkLetter, "\u{182C}\u{1825}"), // ᠬᠥ
        (CodeType::MenkLetter, "\u{182D}\u{1826}"), // ᠭᠦ
        (CodeType::MenkLetter, "\u{182D}\u{1825}"), // ᠭᠥ
        (CodeType::Delehi, "\u{182C}\u{1826}"),
        (CodeType::MenkShape, "\u{E2DD}\u{E287}"),
        (CodeType::MenkShape, "\u{E2D4}\u{E287}"),
        (CodeType::Z52, "\u{188B}\u{186D}"),
    ];
    for (from, input) in sources {
        assert_eq!(
            translate(from, CodeType::Zvvnmod, input).unwrap(),
            HUB_G_O_ISOL,
            "{from:?} {input:?}"
        );
        assert_eq!(
            translate(from, CodeType::Utn57, input).unwrap(),
            UTN57_G_O_ISOL,
            "{from:?} {input:?}"
        );
    }
}

#[test]
fn the_ligature_inside_a_word_is_still_the_medial_glyph() {
    // ᠠᠬᠦ
    assert_eq!(
        translate(
            CodeType::MenkLetter,
            CodeType::Zvvnmod,
            "\u{1820}\u{182C}\u{1826}"
        )
        .unwrap(),
        "\u{E000}\u{E005}\u{E09C}"
    );
}

#[test]
fn every_word_of_running_text_gets_the_initial_ligature() {
    assert_eq!(
        translate(
            CodeType::MenkLetter,
            CodeType::Zvvnmod,
            "\u{182C}\u{1826} \u{182C}\u{1826}\u{1802}\u{182C}\u{1826}"
        )
        .unwrap(),
        "\u{E096} \u{E096}\u{1802}\u{E096}"
    );
}

#[test]
fn the_initial_ligature_writes_the_same_ink_as_the_medial_one_in_every_legacy_target() {
    for target in [
        CodeType::MenkLetter,
        CodeType::Delehi,
        CodeType::MenkShape,
        CodeType::Z52,
    ] {
        let from_isol = translate(CodeType::Zvvnmod, target, HUB_G_O_ISOL).unwrap();
        assert_eq!(
            from_isol,
            translate(CodeType::Zvvnmod, target, HUB_G_O_FINA).unwrap(),
            "{target:?}"
        );
        assert!(
            !from_isol.contains('\u{E096}'),
            "{target:?}: hub code leaked: {from_isol:?}"
        );
    }
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, HUB_G_O_ISOL).unwrap(),
        "\u{E2DD}\u{E287}"
    );
}

#[test]
fn the_initial_ligature_round_trips_through_utn57() {
    let utn57 = translate(CodeType::Zvvnmod, CodeType::Utn57, HUB_G_O_ISOL).unwrap();
    assert_eq!(utn57, UTN57_G_O_ISOL);
    assert_eq!(
        translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57).unwrap(),
        HUB_G_O_ISOL
    );
}

#[test]
fn a_hand_written_medial_glyph_alone_is_still_padded_and_now_reported() {
    let padded = translate_with_warnings(CodeType::Zvvnmod, CodeType::Utn57, HUB_G_O_FINA).unwrap();

    assert_eq!(padded.text, format!("\u{200D}{UTN57_G_O_ISOL}"));
    assert_eq!(padded.warnings.len(), 1, "{:?}", padded.warnings);
    match &padded.warnings[0] {
        Warning::Utn57(reason) => assert!(reason.contains("U+E09C"), "{reason}"),
        other => panic!("unexpected warning {other:?}"),
    }
    assert!(padded.warnings[0].to_string().contains("ZWJ"));
    // The text-only entry point is the same conversion.
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, HUB_G_O_FINA).unwrap(),
        padded.text
    );
}

#[test]
fn a_conversion_the_hub_can_spell_carries_no_warning() {
    for (from, to, input) in [
        (CodeType::Zvvnmod, CodeType::Utn57, HUB_G_O_ISOL),
        (CodeType::MenkLetter, CodeType::Utn57, "\u{182C}\u{1826}"),
        (
            CodeType::MenkLetter,
            CodeType::MenkShape,
            "\u{182C}\u{1826}",
        ),
        (CodeType::Zvvnmod, CodeType::Zvvnmod, HUB_G_O_FINA),
    ] {
        let conversion = translate_with_warnings(from, to, input).unwrap();
        assert!(
            conversion.warnings.is_empty(),
            "{from:?}→{to:?} {input:?}: {:?}",
            conversion.warnings
        );
    }
}
