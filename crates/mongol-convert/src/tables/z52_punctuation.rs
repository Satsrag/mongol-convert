//! Strict Z52 punctuation encoding.
//!
//! Z52 fonts and input methods overload `U+184F..=U+1863` with punctuation glyphs even though
//! those code points are Todo/Sibe letters in Unicode. Evidence includes the published ZCode
//! keyboard (`Satsrag/embed_input`, `zcode_embed_ime/lib/zcode_layout.dart`) and its bundled reduced
//! Z52 font, corroborated by `east-mod/meco`'s `fonts/z52/7 - Z52 Tsagaan Tig.otf`. Apply this table
//! only at an explicit Z52 target boundary; never reinterpret these code points in ordinary Unicode
//! text.

/// Standard semantic punctuation to strict Z52 font code positions, sorted by input code point.
pub const UNICODE_TO_Z52_PUNCTUATION: &[(char, &str)] = &[
    ('!', "\u{1852}"),
    ('(', "\u{1855}"),
    (')', "\u{1856}"),
    (',', "\u{185f}"),
    ('-', "\u{1862}"),
    (';', "\u{1854}"),
    ('?', "\u{1853}"),
    ('|', "\u{1863}"),
    ('\u{00b7}', "\u{184f}"),
    ('\u{00d7}', "\u{1860}"),
    ('\u{203b}', "\u{1861}"),
    ('\u{2048}', "\u{1850}"),
    ('\u{2049}', "\u{1851}"),
    ('\u{3008}', "\u{1857}"),
    ('\u{3009}', "\u{1858}"),
    ('\u{300a}', "\u{185b}"),
    ('\u{300b}', "\u{185c}"),
    ('\u{300e}', "\u{185d}"),
    ('\u{300f}', "\u{185e}"),
    ('\u{3014}', "\u{1859}"),
    ('\u{3015}', "\u{185a}"),
];

#[inline]
pub fn contains(c: char) -> bool {
    UNICODE_TO_Z52_PUNCTUATION
        .binary_search_by_key(&c, |(source, _)| *source)
        .is_ok()
}

#[inline]
pub fn get(key: &str) -> Option<&'static str> {
    let mut chars = key.chars();
    let c = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    UNICODE_TO_Z52_PUNCTUATION
        .binary_search_by_key(&c, |(source, _)| *source)
        .ok()
        .map(|index| UNICODE_TO_Z52_PUNCTUATION[index].1)
}
