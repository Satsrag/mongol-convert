//! `utn57_shape`: a UTN #57 word spelled as its written units, ᠰᠠᠢᠨ as `SAIIA`.
//!
//! The encoding is `utn57` seen through mongol-norm's shape function, so it must agree with `utn57`
//! in both directions and pass everything else through like the other encodings do.

use mongol_convert::{translate, translate_with_warnings, CodeType};

/// ᠰᠠᠢᠨ as the letter encodings spell it.
const SAIN_DELEHI: &str = "\u{1830}\u{1820}\u{1822}\u{1828}";

#[test]
fn a_word_reaches_the_shape_encoding_from_any_source() {
    assert_eq!(
        translate(CodeType::Delehi, CodeType::Utn57Shape, SAIN_DELEHI).unwrap(),
        "SAIIA"
    );
    assert_eq!(
        translate(CodeType::MenkLetter, CodeType::Utn57Shape, SAIN_DELEHI).unwrap(),
        "SAIIA"
    );
}

/// The shape encoding never disagrees with `utn57`: read as a source, it reaches every target
/// exactly as the UTN #57 spelling it stands for does. (That the legacy targets are not the
/// identity of their own input — ᠰᠠᠢᠨ comes back from the hub with FVS3 marks — is the hub's
/// Java-parity behaviour, the same on both paths.)
#[test]
fn a_spelled_word_reaches_every_target_as_the_utn57_spelling_does() {
    let utn57 = translate(CodeType::Delehi, CodeType::Utn57, SAIN_DELEHI).unwrap();
    for spelling in ["SAIIA", "S+A+I+I+A"] {
        assert_eq!(
            translate(CodeType::Utn57Shape, CodeType::Utn57, spelling).unwrap(),
            utn57
        );
        for target in [
            CodeType::Zvvnmod,
            CodeType::Delehi,
            CodeType::MenkLetter,
            CodeType::MenkShape,
            CodeType::Z52,
        ] {
            assert_eq!(
                translate(CodeType::Utn57Shape, target, spelling).unwrap(),
                translate(CodeType::Utn57, target, &utn57).unwrap(),
                "{spelling} → {target:?}"
            );
        }
    }
    assert_eq!(
        translate(CodeType::Utn57Shape, CodeType::Zvvnmod, "SAIIA").unwrap(),
        translate(CodeType::Delehi, CodeType::Zvvnmod, SAIN_DELEHI).unwrap()
    );
}

#[test]
fn passthrough_and_the_detached_suffix_survive() {
    // ᠲᠠᠯ᠎ᠠ ᠶᠢᠨ, then a comma and a number.
    let delehi =
        "\u{1832}\u{1820}\u{182F}\u{180E}\u{1820}\u{202F}\u{1836}\u{1822}\u{1828}\u{1802} 2024";
    let shape = translate(CodeType::Delehi, CodeType::Utn57Shape, delehi).unwrap();
    assert_eq!(shape, "TALMvsAaMvsIIA\u{1802} 2024");

    let utn57 = translate(CodeType::Delehi, CodeType::Utn57, delehi).unwrap();
    let back = translate(CodeType::Utn57Shape, CodeType::Delehi, &shape).unwrap();
    assert_eq!(
        back,
        translate(CodeType::Utn57, CodeType::Delehi, &utn57).unwrap()
    );
    assert!(back.ends_with("\u{1802} 2024"), "{back:?}");
    assert_eq!(
        back.matches('\u{202F}').count(),
        1,
        "the suffix boundary: {back:?}"
    );
}

#[test]
fn the_shape_encoding_is_its_own_identity_and_round_trips_through_the_hub() {
    assert_eq!(
        translate(CodeType::Utn57Shape, CodeType::Utn57Shape, "SAIIA").unwrap(),
        "SAIIA"
    );
    let hub = translate(CodeType::Utn57Shape, CodeType::Zvvnmod, "MOAGNNOLMvsOA").unwrap();
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57Shape, &hub).unwrap(),
        "MOAGNNOLMvsOA"
    );
}

#[test]
fn output_carries_the_utn57_warnings_and_input_carries_none() {
    // A lone medial glyph is spelled with an invented ZWJ, which the shape shows as `Zwj`.
    let padded =
        translate_with_warnings(CodeType::Zvvnmod, CodeType::Utn57Shape, "\u{E09C}").unwrap();
    assert_eq!(padded.warnings.len(), 1, "{:?}", padded.warnings);
    assert_eq!(padded.text, "ZwjGO");
    assert!(
        translate_with_warnings(CodeType::Utn57Shape, CodeType::Utn57, "SAIIA")
            .unwrap()
            .warnings
            .is_empty()
    );
}

#[test]
fn a_spelling_that_is_not_one_is_an_error_not_passthrough() {
    let error = translate(CodeType::Utn57Shape, CodeType::Utn57, "Hello").unwrap_err();
    assert!(error.to_string().contains("utn57_shape:"), "{error}");
}

/// The shape is a pure function of the spelling, and normalizing a shape is canonical: wherever
/// hub → utn57 → hub lands, hub → utn57_shape → hub lands too.
#[test]
fn the_shape_round_trip_is_the_utn57_round_trip() {
    let corpus = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/corpus_delehi.txt"),
    )
    .expect("corpus should be readable");
    let (mut agree, mut checked) = (0usize, 0usize);
    let mut sample = Vec::new();
    for word in corpus.split_whitespace().take(400) {
        let Ok(hub) = translate(CodeType::Delehi, CodeType::Zvvnmod, word) else {
            continue;
        };
        let Ok(utn57) = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub) else {
            continue;
        };
        let Ok(via_utn57) = translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57) else {
            continue;
        };
        let shape = translate(CodeType::Zvvnmod, CodeType::Utn57Shape, &hub).unwrap();
        let via_shape = translate(CodeType::Utn57Shape, CodeType::Zvvnmod, &shape).unwrap();
        checked += 1;
        if via_shape == via_utn57 {
            agree += 1;
        } else if sample.len() < 3 {
            sample.push(format!(
                "{word:?}: {shape} → {via_shape:?} vs {via_utn57:?}"
            ));
        }
    }
    assert!(checked > 300, "corpus too small: {checked}");
    assert_eq!(
        agree,
        checked,
        "shape and utn57 round trips disagree:\n{}",
        sample.join("\n")
    );
}
