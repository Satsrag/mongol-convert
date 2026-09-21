//! Rust-owned ZVVNMOD Birga rules that intentionally override historical Java tables.
//!
//! MenkShape stores four ornate Birgas in the BMP private-use area. The ZVVNMOD
//! representation uses the standard Mongolian Supplement scalars directly.

pub const MENK_SHAPE_BIRGAS: [char; 4] = ['\u{E23F}', '\u{E240}', '\u{E241}', '\u{E242}'];

pub const ZVVNMOD_BIRGAS: [char; 4] = ['\u{11660}', '\u{11661}', '\u{11662}', '\u{11663}'];

pub fn menk_shape_birga_to_zvvnmod(character: char) -> Option<&'static str> {
    match character {
        '\u{E23F}' => Some("\u{11660}"),
        '\u{E240}' => Some("\u{11661}"),
        '\u{E241}' => Some("\u{11662}"),
        '\u{E242}' => Some("\u{11663}"),
        _ => None,
    }
}

pub fn zvvnmod_birga_to_menk_shape(key: &str) -> Option<&'static str> {
    match key {
        "\u{11660}" => Some("\u{E23F}"),
        "\u{11661}" => Some("\u{E240}"),
        "\u{11662}" => Some("\u{E241}"),
        "\u{11663}" => Some("\u{E242}"),
        _ => None,
    }
}
