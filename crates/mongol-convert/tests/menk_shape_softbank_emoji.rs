use mongol_convert::{
    restore_menk_shape_emoji, translate, translate_with_options, CodeType, TranslationOptions,
};

#[test]
fn public_helper_restores_paste_text_without_decoding_it() {
    assert_eq!(restore_menk_shape_emoji("➡️🎂"), "\u{E234}\u{E34B}");
    assert_eq!(restore_menk_shape_emoji("plain 😀"), "plain 😀");
}

#[test]
fn restores_softbank_emoji_back_to_menk_shape_input() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, "➡️").unwrap(),
        translate(CodeType::MenkShape, CodeType::Zvvnmod, "\u{E234}").unwrap()
    );
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, "🔯").unwrap(),
        translate(CodeType::MenkShape, CodeType::Zvvnmod, "\u{E23E}").unwrap()
    );
}

#[test]
fn restores_inside_mixed_menk_shape_text() {
    let emoji_damaged = "\u{E2FD}🔯\u{E2B5}";
    let original = "\u{E2FD}\u{E23E}\u{E2B5}";

    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, emoji_damaged).unwrap(),
        translate(CodeType::MenkShape, CodeType::Zvvnmod, original).unwrap()
    );
}

#[test]
fn other_sources_do_not_restore_softbank_emoji() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, "➡️").unwrap(),
        "➡️"
    );
}

#[test]
fn restore_can_be_disabled_for_menk_shape_input() {
    let restored = translate(CodeType::MenkShape, CodeType::Zvvnmod, "➡️").unwrap();
    let raw = translate_with_options(
        CodeType::MenkShape,
        CodeType::Zvvnmod,
        "➡️",
        &TranslationOptions {
            restore_menk_shape_emoji: false,
            ..TranslationOptions::default()
        },
    )
    .unwrap()
    .text;

    assert_ne!(raw, restored);
    assert_eq!(raw, "➡️");
}

#[test]
fn same_encoding_menk_shape_restore_can_be_disabled() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::MenkShape, "➡️").unwrap(),
        "\u{E234}"
    );
    assert_eq!(
        translate_with_options(
            CodeType::MenkShape,
            CodeType::MenkShape,
            "➡️",
            &TranslationOptions {
                restore_menk_shape_emoji: false,
                ..TranslationOptions::default()
            },
        )
        .unwrap()
        .text,
        "➡️"
    );
}
