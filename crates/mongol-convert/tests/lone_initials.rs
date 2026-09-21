//! A lone initial glyph written as a word — the case from Satsrag/mongol-convert#45.
//!
//! `AZwj` is the initial A with nothing after it, and the hub keeps it as `E000` (A i). Written
//! bare in UTN #57 it read back as the isolated A, so the page turned `AZwj` into `A`. mongol-norm
//! 0.2.1 spells a lone `A:init` or `I:init` with the trailing ZWJ `O:init` already had. An isolated
//! consonant is its initial written unit, so a consonant needs no joiner and comes back bare.

use mongol_convert::{translate, CodeType};

/// What the converter page does: the left pane to the hub, the hub to the right pane.
fn page_round_trip(text: &str) -> String {
    let hub = translate(CodeType::Utn57Shape, CodeType::Zvvnmod, text).unwrap();
    translate(CodeType::Zvvnmod, CodeType::Utn57Shape, &hub).unwrap()
}

#[test]
fn the_issue_text_survives_the_page_round_trip() {
    let text = "A\nAZwj\nZwjAZwj\nZwjA";
    assert_eq!(page_round_trip(text), text);
}

#[test]
fn every_lone_initial_vowel_survives_the_page_round_trip() {
    for text in ["AZwj", "IZwj", "OZwj"] {
        assert_eq!(page_round_trip(text), text);
    }
}

#[test]
fn a_lone_initial_consonant_comes_back_bare() {
    // B:init is the isolated B too: the joiner carries nothing, so it is not written back.
    assert_eq!(page_round_trip("BZwj"), "B");
    assert_eq!(page_round_trip("B"), "B");
}

#[test]
fn the_hub_initials_read_back_as_themselves_through_utn57() {
    for (hub, utn57) in [
        ("\u{E000}", "\u{1820}\u{180B}\u{200D}"),
        ("\u{E04D}", "\u{1822}\u{180B}\u{200D}"),
    ] {
        assert_eq!(
            translate(CodeType::Zvvnmod, CodeType::Utn57, hub).unwrap(),
            utn57
        );
        assert_eq!(
            translate(CodeType::Utn57, CodeType::Zvvnmod, utn57).unwrap(),
            hub
        );
    }
}
