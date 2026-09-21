//! ᠤᠯᠤᠰ + two NNBSP + ᠤᠨ — a detached-suffix boundary typed twice, the case from
//! Satsrag/mongol-convert#40.
//!
//! The hub keeps both separators, as the legacy targets do; UTN #57 allows one MVS between the
//! stem's final letter and the suffix's first, so the encoder writes one and reports the repetition.

use meco_core::{translate, translate_with_warnings, CodeType, Warning};

/// ᠤᠯᠤᠰ ᠤᠨ in menk_letter, with the boundary written `n` times.
fn ulus_un(n: usize) -> String {
    format!(
        "\u{1824}\u{182F}\u{1824}\u{1830}{}\u{1824}\u{1828}",
        "\u{202F}".repeat(n)
    )
}

#[test]
fn a_boundary_typed_twice_reaches_utn57_as_one_mvs() {
    let once = translate(CodeType::MenkLetter, CodeType::Utn57, &ulus_un(1)).unwrap();
    let twice = translate(CodeType::MenkLetter, CodeType::Utn57, &ulus_un(2)).unwrap();

    assert_eq!(once.matches('\u{180E}').count(), 1, "{once:?}");
    assert_eq!(twice, once);
    assert_eq!(
        once,
        "\u{1820}\u{180B}\u{1823}\u{182F}\u{1823}\u{1830}\u{180E}\u{1824}\u{180B}\u{1820}\u{180C}"
    );
}

#[test]
fn the_repetition_is_reported_and_the_hub_keeps_it() {
    let conversion =
        translate_with_warnings(CodeType::MenkLetter, CodeType::Utn57, &ulus_un(2)).unwrap();
    assert_eq!(conversion.warnings.len(), 1, "{:?}", conversion.warnings);
    match &conversion.warnings[0] {
        Warning::Utn57(reason) => assert!(reason.contains("2 separators"), "{reason}"),
        other => panic!("unexpected warning {other:?}"),
    }

    // The hub and the legacy targets carry the text as it was written.
    assert_eq!(
        translate(CodeType::MenkLetter, CodeType::Zvvnmod, &ulus_un(2)).unwrap(),
        "\u{E000}\u{E008}\u{E03A}\u{E008}\u{E03E}\u{202F}\u{202F}\u{E001}\u{E00C}"
    );
    assert_eq!(
        translate(CodeType::MenkLetter, CodeType::Delehi, &ulus_un(2))
            .unwrap()
            .matches('\u{202F}')
            .count(),
        2
    );
}

#[test]
fn a_boundary_typed_once_raises_no_warning() {
    let conversion =
        translate_with_warnings(CodeType::MenkLetter, CodeType::Utn57, &ulus_un(1)).unwrap();
    assert!(conversion.warnings.is_empty(), "{:?}", conversion.warnings);
}
