//! Menksoft's punctuation glyphs carry no side bearing, so a mark set straight after a word sits
//! on the last stroke's ink. The gap is written in with Menksoft's own space, `U+E263`, on the
//! side each mark takes, and removed again on the way back (Satsrag/mongol-convert#29).

use mongol_convert::{translate, CodeType};

const WORD: &str = "\u{1830}\u{1820}\u{1836}\u{1822}\u{1828}"; // ᠰᠠᠶᠢᠨ
const MENK_WORD: &str = "\u{e2fd}\u{e26c}\u{e27e}\u{e27e}\u{e2b5}";
const GAP: &str = "\u{e263}";

fn to_menk(hub_punctuation: char) -> String {
    translate(
        CodeType::MenkLetter,
        CodeType::MenkShape,
        &format!("{WORD}{hub_punctuation}{WORD}"),
    )
    .unwrap()
}

#[test]
fn eighteen_marks_take_the_gap_before_them() {
    let marks = [
        ('\u{1801}', '\u{e235}'), // ᠁
        ('\u{1802}', '\u{e236}'), // ᠂
        ('\u{1803}', '\u{e237}'), // ᠃
        ('\u{1804}', '\u{e238}'), // ᠄
        ('\u{1805}', '\u{e239}'), // ᠅
        ('\u{1808}', '\u{e23c}'), // ᠈
        ('\u{1809}', '\u{e23d}'), // ᠉
        ('\u{2048}', '\u{e24e}'), // ⁈
        ('\u{2049}', '\u{e24f}'), // ⁉
        ('\u{0021}', '\u{e250}'), // !
        ('\u{003f}', '\u{e251}'), // ?
        ('\u{003b}', '\u{e252}'), // ;
        ('\u{0028}', '\u{e253}'), // (
        ('\u{3008}', '\u{e255}'), // 〈
        ('\u{3014}', '\u{e257}'), // 〔
        ('\u{300a}', '\u{e259}'), // 《
        ('\u{300e}', '\u{e25b}'), // 『
        ('\u{002c}', '\u{e25d}'), // ,
    ];
    assert_eq!(marks.len(), 18);
    for (hub, menk) in marks {
        assert_eq!(
            to_menk(hub),
            format!("{MENK_WORD}{GAP}{menk}{MENK_WORD}"),
            "{hub:?}"
        );
    }
}

#[test]
fn five_closing_brackets_take_it_after() {
    let marks = [
        ('\u{0029}', '\u{e254}'), // )
        ('\u{3009}', '\u{e256}'), // 〉
        ('\u{3015}', '\u{e258}'), // 〕
        ('\u{300b}', '\u{e25a}'), // 》
        ('\u{300f}', '\u{e25c}'), // 』
    ];
    for (hub, menk) in marks {
        assert_eq!(
            to_menk(hub),
            format!("{MENK_WORD}{menk}{GAP}{MENK_WORD}"),
            "{hub:?}"
        );
    }
}

#[test]
fn the_middle_dot_takes_it_on_both_sides() {
    assert_eq!(
        to_menk('\u{00b7}'),
        format!("{MENK_WORD}{GAP}\u{e243}{GAP}{MENK_WORD}")
    );
}

#[test]
fn seven_marks_take_no_gap() {
    let marks = [
        ('\u{1800}', '\u{e234}'), // ᠀ birga
        ('\u{1806}', '\u{e23a}'), // ᠆ Todo soft hyphen
        ('\u{1807}', '\u{e23b}'), // ᠇ Sibe syllable boundary
        ('\u{00d7}', '\u{e25e}'), // ×
        ('\u{203b}', '\u{e25f}'), // ※
        ('\u{002d}', '\u{e260}'), // -
        ('\u{007c}', '\u{e261}'), // |
    ];
    for (hub, menk) in marks {
        assert_eq!(
            to_menk(hub),
            format!("{MENK_WORD}{menk}{MENK_WORD}"),
            "{hub:?}"
        );
    }
}

#[test]
fn the_digits_keep_the_left_bearing_they_already_have() {
    // ᠐..᠙ carry a 31 % em left bearing in the Menksoft font and take no gap.
    assert_eq!(
        translate(
            CodeType::MenkLetter,
            CodeType::MenkShape,
            &format!("{WORD}\u{1810}")
        )
        .unwrap(),
        format!("{MENK_WORD}\u{e244}")
    );
}

#[test]
fn an_ordinary_space_before_a_mark_becomes_the_gap_rather_than_doubling_it() {
    assert_eq!(
        translate(
            CodeType::MenkLetter,
            CodeType::MenkShape,
            &format!("{WORD} \u{1802}{WORD}")
        )
        .unwrap(),
        format!("{MENK_WORD}{GAP}\u{e236}{MENK_WORD}")
    );
}

#[test]
fn a_mark_at_the_edge_of_the_text_keeps_its_place() {
    assert_eq!(
        translate(CodeType::MenkLetter, CodeType::MenkShape, "\u{1802}").unwrap(),
        "\u{e236}"
    );
    assert_eq!(
        translate(
            CodeType::MenkLetter,
            CodeType::MenkShape,
            &format!("{WORD}\u{0029}")
        )
        .unwrap(),
        format!("{MENK_WORD}\u{e254}")
    );
}

#[test]
fn the_gap_is_removed_again_on_the_way_back() {
    for hub in [
        '\u{1802}', '\u{00b7}', '\u{0029}', '\u{0028}', '\u{002d}', '\u{1800}',
    ] {
        let source = format!("{WORD}{hub}{WORD}");
        let expected = translate(CodeType::MenkLetter, CodeType::Zvvnmod, &source).unwrap();
        let menk = translate(CodeType::MenkLetter, CodeType::MenkShape, &source).unwrap();
        assert_eq!(
            translate(CodeType::MenkShape, CodeType::Zvvnmod, &menk).unwrap(),
            expected,
            "{hub:?}"
        );
    }
}

#[test]
fn a_menksoft_space_away_from_a_mark_is_a_word_connector_and_survives() {
    let text = format!("{MENK_WORD}{GAP}{MENK_WORD}");
    let hub = translate(CodeType::MenkShape, CodeType::Zvvnmod, &text).unwrap();
    assert!(
        hub.contains(' '),
        "the connector should reach the hub: {hub:?}"
    );
}

/// Menksoft's own space belongs to MenkShape alone. Z52 has the same fault and gets the same
/// treatment, but writes it with the narrow no-break space instead — see `z52_punctuation.rs`.
#[test]
fn no_other_encoding_gets_the_menksoft_gap() {
    for target in [
        CodeType::Zvvnmod,
        CodeType::Delehi,
        CodeType::MenkLetter,
        CodeType::Z52,
        CodeType::Utn57,
    ] {
        let out = translate(
            CodeType::MenkLetter,
            target,
            &format!("{WORD}\u{1802}{WORD}"),
        )
        .unwrap();
        assert!(!out.contains('\u{e263}'), "{target:?}: {out:?}");
    }
}
