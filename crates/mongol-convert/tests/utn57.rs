//! UTN #57 output is always available: the pure-Rust `zvvnmod-utn57` backend is linked into the
//! core, so these tests run in the default build on every platform.

use mongol_convert::{translate, CodeType, MongolConvertError};

#[test]
fn passthrough_text_reaches_utn57_unchanged() {
    // NNBSP is no longer in this list: it is the hub's suffix boundary now, and it converts.
    let input = "plain\u{180A}\u{180E}\u{200D}";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn the_hub_boundary_becomes_an_mvs() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E00D}\u{202F}\u{E04D}").unwrap(),
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E00D}").unwrap()
            + "\u{180E}"
            + &translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E04D}").unwrap(),
    );
}

/// ᠲᠠᠯ᠎ᠠ᠎ᠶᠢᠨ — a chachlag A and then a detached suffix, the case from Satsrag/mongol-convert#22.
///
/// Both separators are MVS in UTN #57 but mean different things: the first belongs to the chachlag
/// and folds into a single hub code, the second is a suffix boundary and must survive as one. The
/// legacy spelling distinguishes them itself, MVS against NNBSP, so the two sources are a genuine
/// cross-check rather than a round trip through one implementation.
#[test]
fn utn57_and_delehi_sources_agree_on_the_hub() {
    let utn57 = "\u{1832}\u{1820}\u{182F}\u{180E}\u{1820}\u{180E}\u{1836}\u{1822}\u{1828}";
    let delehi = "\u{1832}\u{1820}\u{182F}\u{180E}\u{1820}\u{202F}\u{1836}\u{1822}\u{1828}";

    let from_delehi = translate(CodeType::Delehi, CodeType::Zvvnmod, delehi).unwrap();
    assert_eq!(
        from_delehi,
        "\u{E042}\u{E005}\u{E03B}\u{E00D}\u{202F}\u{E04D}\u{E006}\u{E00C}",
        "the chachlag folds into one code and the suffix boundary stays as NNBSP"
    );
    assert_eq!(
        translate(CodeType::Utn57, CodeType::Zvvnmod, utn57).unwrap(),
        from_delehi,
        "both sources must reach the same hub"
    );

    for target in [CodeType::MenkLetter, CodeType::MenkShape, CodeType::Z52] {
        assert_eq!(
            translate(CodeType::Utn57, target, utn57).unwrap(),
            translate(CodeType::Delehi, target, delehi).unwrap(),
            "{target:?} must not depend on which source the word came from"
        );
    }
}

#[test]
fn the_boundary_is_not_doubled_on_the_way_out() {
    let hub = "\u{E042}\u{E005}\u{E03B}\u{E00D}\u{202F}\u{E04D}\u{E006}\u{E00C}";
    for target in [CodeType::Delehi, CodeType::MenkLetter] {
        let out = translate(CodeType::Zvvnmod, target, hub).unwrap();
        assert_eq!(out.matches('\u{202F}').count(), 1, "{target:?}: {out:?}");
    }
    // The shape encodings have no NNBSP of their own and keep the ordinary space they always had.
    for target in [CodeType::MenkShape, CodeType::Z52] {
        let out = translate(CodeType::Zvvnmod, target, hub).unwrap();
        assert!(!out.contains('\u{202F}'), "{target:?}: {out:?}");
        assert_eq!(out.matches(' ').count(), 1, "{target:?}: {out:?}");
    }
}

#[test]
fn utn57_identity_short_circuits() {
    let input = "\u{1820}\u{180A}\u{202F}";

    assert_eq!(
        translate(CodeType::Utn57, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn formal_zvvnmod_nirugu_converts_in_process() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E0E5}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn legacy_sources_route_through_the_zvvnmod_hub() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Utn57, "\u{E23E}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn z52_word_converts_to_canonical_unicode() {
    let output = translate(
        CodeType::Z52,
        CodeType::Utn57,
        "ᡳᡬᡦ ᢌᡭᡪᢊᡱᡱᡭᢐ ᢋᡭᡬᢎᡭᡧ",
    )
    .unwrap();

    assert!(!output.is_empty());
    assert!(
        output
            .chars()
            .all(|c| !('\u{E000}'..='\u{F8FF}').contains(&c)),
        "UTN #57 output must not contain private-use ZVVNMOD shapes: {output:?}"
    );
    assert!(
        output.chars().any(|c| ('\u{1820}'..='\u{1842}').contains(&c)),
        "UTN #57 output should contain Unicode Mongolian letters: {output:?}"
    );
    assert_eq!(output.matches(' ').count(), 2, "spaces pass through: {output:?}");
}

#[test]
fn legacy_controls_are_discarded() {
    assert_eq!(
        translate(
            CodeType::Zvvnmod,
            CodeType::Utn57,
            "\u{E140}\u{E141}\u{E142}\u{E143}\u{E144}",
        )
        .unwrap(),
        ""
    );
}

#[test]
fn utn57_decodes_back_to_the_hub() {
    // The reverse must land on exactly the hub value the forward direction started from, or the two
    // halves disagree about what a word is.
    let hub = translate(CodeType::Delehi, CodeType::Zvvnmod, "\u{1830}\u{1820}\u{180D}\u{1822}\u{180D}\u{1822}\u{1828}").unwrap();
    let utn57 = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub).unwrap();
    assert_eq!(translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57).unwrap(), hub);
}

#[test]
fn utn57_is_a_source_for_the_other_encodings() {
    // ᠮᠣᠩᠭᠣᠯ, whose UTN #57 spelling uses FVS1 and FVS2 that the letter encodings do not.
    let delehi = "\u{182E}\u{1823}\u{1829}\u{182D}\u{1823}\u{182F}";
    let utn57 = translate(CodeType::Delehi, CodeType::Utn57, delehi).unwrap();
    assert_ne!(utn57, delehi, "the two conventions spell this word differently");
    assert_eq!(translate(CodeType::Utn57, CodeType::Delehi, &utn57).unwrap(), delehi);

    let via_utn57 = translate(CodeType::Utn57, CodeType::MenkShape, &utn57).unwrap();
    let via_delehi = translate(CodeType::Delehi, CodeType::MenkShape, delehi).unwrap();
    assert_eq!(via_utn57, via_delehi, "either source should reach the same MenkShape text");
}

/// zvvnmod -> utn57 -> zvvnmod over the real corpus. It is not the identity yet: `zvvnmod-utn57`
/// 0.1.1's reverse mapping does not reconstruct every hub value, so this pins the current numbers
/// instead of pretending. Tighten the bounds when the backend improves — a run that beats them
/// fails, which is the point.
///
/// Two classes are skipped rather than papered over, because the forward direction documents that
/// it drops them: the legacy ZVVNMOD controls U+E140..=U+E144, and ZWJ.
#[test]
fn hub_round_trip_over_the_corpus_matches_the_known_gap() {
    const LOSSY: fn(char) -> bool = |c| matches!(c, '\u{E140}'..='\u{E144}' | '\u{200D}');
    const MIN_OK: usize = 1020;   // zvvnmod-utn57 0.1.1
    const MAX_BROKEN: usize = 33; // ditto

    let corpus = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/corpus_delehi.txt"),
    )
    .expect("corpus should be readable");

    let (mut ok, mut broken) = (0usize, 0usize);
    let mut sample = Vec::new();
    for word in corpus.split_whitespace() {
        let Ok(hub) = translate(CodeType::Delehi, CodeType::Zvvnmod, word) else { continue };
        if hub.chars().any(LOSSY) {
            continue;
        }
        let Ok(utn57) = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub) else { continue };
        let Ok(back) = translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57) else { continue };
        if back == hub {
            ok += 1;
        } else {
            broken += 1;
            if sample.len() < 3 {
                sample.push(format!("{word:?}: {hub:?} -> {utn57:?} -> {back:?}"));
            }
        }
    }

    assert!(ok >= MIN_OK, "round-trip regressed: {ok} words survived, expected at least {MIN_OK}");
    assert!(
        broken <= MAX_BROKEN,
        "round-trip regressed: {broken} words broke, expected at most {MAX_BROKEN}\n{}",
        sample.join("\n")
    );
}

#[test]
fn utn57_error_variant_display_is_stable() {
    assert_eq!(
        MongolConvertError::Utn57("backend unavailable".to_owned()).to_string(),
        "UTN #57 conversion failed: backend unavailable"
    );
}

/// ᠠᠪᠤᠭᠰᠠᠨ — a bowed unit followed by teeth, the case from Satsrag/mongol-convert#26.
///
/// Through zvvnmod-utn57 0.1.2 the hub's `B_O_MEDI A_MEDI` came out as `B` + nirugu + `Dd`, the
/// composite written unit that spells `O A` a second way and renders as a separate loop after the
/// bowl. mongol-norm 0.2.0 folds that duplicate out, so the spelling is `B O` with the teeth kept
/// as teeth and no `Dd` (U+1833) anywhere.
#[test]
fn a_bowl_before_teeth_is_spelled_b_o_not_dd() {
    let hub = "\u{E000}\u{E005}\u{E083}\u{E005}\u{E005}\u{E03D}\u{E005}\u{E00C}";
    let out = translate(CodeType::Zvvnmod, CodeType::Utn57, hub).unwrap();

    assert_eq!(
        out,
        "\u{1820}\u{180B}\u{1820}\u{182A}\u{1823}\u{1820}\u{1820}\u{1830}\u{1820}\u{1820}\u{180C}"
    );
    assert!(!out.contains('\u{1833}'), "no Dd in {out:?}");
    // The same word arriving as letters lands on the same spelling, and it decodes back to the hub.
    assert_eq!(
        translate(CodeType::MenkLetter, CodeType::Utn57, "ᠠᠪᠤᠭᠰᠠᠨ").unwrap(),
        out
    );
    assert_eq!(
        translate(CodeType::Utn57, CodeType::Zvvnmod, &out).unwrap(),
        hub
    );
}

/// ᠠ᠋ᠠ᠊· — a word held open by a nirugu, then a middle dot, the case from Satsrag/mongol-convert#29.
///
/// Two things went wrong on the way to Menk shape: the hub's nirugu was spelled with ZVVNMOD's own
/// E0E5, which the legacy tables did not know and passed through as a `.notdef` box, and the middle
/// dot was set flush against the stroke where Menksoft writes its own space, `U+E263`, before it.
#[test]
fn a_nirugu_and_a_middle_dot_reach_menk_shape() {
    let out = translate(
        CodeType::Utn57,
        CodeType::MenkShape,
        "\u{1820}\u{180B}\u{1820}\u{180A}\u{00B7}",
    )
    .unwrap();

    assert_eq!(out, "\u{E271}\u{E26C}\u{E23E}\u{E263}\u{E243}");
    assert!(!out.contains('\u{E0E5}'), "formal nirugu leaked: {out:?}");
}
