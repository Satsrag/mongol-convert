use meco_core::{translate, CodeType};

const MENK_BIRGAS: &str = "\u{E23F}\u{E240}\u{E241}\u{E242}";
const UNICODE_BIRGAS: &str = "\u{11660}\u{11661}\u{11662}\u{11663}";

#[test]
fn menk_shape_birgas_use_unicode_codepoints_in_zvvnmod() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_BIRGAS).unwrap(),
        UNICODE_BIRGAS
    );
}

#[test]
fn zvvnmod_birgas_are_encoded_for_menk_shape() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, UNICODE_BIRGAS).unwrap(),
        MENK_BIRGAS
    );
}

#[test]
fn birgas_roundtrip_through_zvvnmod_in_both_directions() {
    let zvvnmod = translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_BIRGAS).unwrap();
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, &zvvnmod).unwrap(),
        MENK_BIRGAS
    );

    let menk = translate(CodeType::Zvvnmod, CodeType::MenkShape, UNICODE_BIRGAS).unwrap();
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, &menk).unwrap(),
        UNICODE_BIRGAS
    );
}

#[test]
fn every_source_spells_the_nirugu_the_hub_way() {
    // The hub uses ZVVNMOD's own code for the nirugu, E0E5 — the same one the UTN #57 crate reads
    // and writes — so all four sources agree on one hub text (Satsrag/mongol-convert#29).
    for source in [CodeType::Delehi, CodeType::MenkLetter] {
        assert_eq!(
            translate(source, CodeType::Zvvnmod, "A\u{180A}B").unwrap(),
            "A\u{E0E5}B",
            "source={source:?}"
        );
    }

    // ᠦ᠊, each source in its own spelling, all landing on one hub text.
    for (source, word) in [
        (CodeType::Delehi, "\u{1826}\u{180A}"),
        (CodeType::MenkLetter, "\u{1826}\u{180A}"),
        (CodeType::Utn57, "\u{1826}\u{180A}"),
        (CodeType::MenkShape, "\u{E271}\u{E291}\u{E27E}\u{E23E}"),
        (CodeType::Z52, "\u{1865}\u{186D}\u{186C}\u{180A}"),
    ] {
        assert_eq!(
            translate(source, CodeType::Zvvnmod, word).unwrap(),
            "\u{E000}\u{E008}\u{E006}\u{E0E5}",
            "source={source:?}"
        );
    }

    let contexts = [
        ("\u{180A}\u{1820}", "\u{E0E5}\u{E00C}"),
        (
            "\u{1820}\u{180A}\u{1820}",
            "\u{E000}\u{E005}\u{E0E5}\u{E00C}",
        ),
        ("\u{1820}\u{180A}", "\u{E000}\u{E005}\u{E0E5}"),
    ];
    for source in [CodeType::Delehi, CodeType::MenkLetter] {
        for (input, expected) in contexts {
            assert_eq!(
                translate(source, CodeType::Zvvnmod, input).unwrap(),
                expected,
                "source={source:?}, input={input:?}"
            );
        }
    }
}

#[test]
fn each_encoding_gets_the_nirugu_it_spells() {
    const HUB: &str = "\u{E0E5}";
    const UNICODE: &str = "\u{180A}";
    const MENK_SHAPE: &str = "\u{E23E}";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, HUB).unwrap(),
        MENK_SHAPE
    );
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_SHAPE).unwrap(),
        HUB
    );

    for encoding in [CodeType::Delehi, CodeType::MenkLetter, CodeType::Z52] {
        assert_eq!(
            translate(CodeType::Zvvnmod, encoding, HUB).unwrap(),
            UNICODE,
            "target={encoding:?}"
        );
        assert_eq!(
            translate(encoding, CodeType::Zvvnmod, UNICODE).unwrap(),
            HUB,
            "source={encoding:?}"
        );
        assert_eq!(
            translate(CodeType::MenkShape, encoding, MENK_SHAPE).unwrap(),
            UNICODE,
            "MenkShape -> {encoding:?}"
        );
        assert_eq!(
            translate(encoding, CodeType::MenkShape, UNICODE).unwrap(),
            MENK_SHAPE,
            "{encoding:?} -> MenkShape"
        );
    }
}

/// A hub written before the spellings were unified still converts: U+180A is read as a nirugu
/// wherever the hub is an input, and only E0E5 is ever produced.
#[test]
fn a_hub_spelled_the_old_way_is_still_read() {
    const OLD_HUB: &str = "\u{E000}\u{E008}\u{E006}\u{180A}";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, OLD_HUB).unwrap(),
        "\u{E271}\u{E291}\u{E27E}\u{E23E}"
    );
    for encoding in [CodeType::Delehi, CodeType::MenkLetter] {
        assert_eq!(
            translate(CodeType::Zvvnmod, encoding, OLD_HUB).unwrap(),
            "\u{1826}\u{180A}",
            "target={encoding:?}"
        );
    }
}

#[test]
fn unicode_letter_encodings_use_zvvnmod_birgas() {
    for encoding in [CodeType::Delehi, CodeType::MenkLetter, CodeType::Z52] {
        assert_eq!(
            translate(CodeType::MenkShape, encoding, MENK_BIRGAS).unwrap(),
            UNICODE_BIRGAS,
            "MenkShape -> {encoding:?}"
        );
        assert_eq!(
            translate(encoding, CodeType::MenkShape, UNICODE_BIRGAS).unwrap(),
            MENK_BIRGAS,
            "{encoding:?} -> MenkShape"
        );
    }
}

#[test]
fn a_nirugu_is_shaped_inside_the_word_on_the_way_to_utn57() {
    // Handed a bare U+180A the UTN #57 crate ends the run there and pins the letter before it with
    // a ZWJ; handed the hub's own code it shapes the nirugu in place.
    assert_eq!(
        translate(
            CodeType::MenkShape,
            CodeType::Utn57,
            "\u{E271}\u{E26C}\u{E23E}"
        )
        .unwrap(),
        "\u{1820}\u{180B}\u{1820}\u{180A}"
    );
    assert_eq!(
        translate(
            CodeType::Delehi,
            CodeType::Utn57,
            "\u{1820}\u{180A}\u{1820}"
        )
        .unwrap(),
        "\u{1820}\u{180B}\u{1820}\u{180A}\u{1820}\u{180C}"
    );
}
