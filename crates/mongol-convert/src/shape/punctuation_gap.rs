//! Punctuation spacing for the two shape encodings.
//!
//! Menksoft's punctuation glyphs carry no side bearing — measured in
//! MenksoftQagan_shape.ttf, every mark in `E234..=E261` has a left and right bearing of 0.0–0.7 %
//! of an em, so the glyph fills its advance box and the mark sits on the neighbouring letter's
//! ink. The Unicode encodings do not need this: their fonts draw the same marks with the gap built
//! in (Almas Mongolian White gives `᠂` 16.9 % on the left, Menk Qagan Tig 18.3 %), which is why
//! this runs for the shape encodings and nothing else. Z52 has the same fault over the 21 marks it
//! spells with its own code points, `184F..=1863`, every one of them 0.0–0.7 % on both sides; the
//! 10 Mongolian marks it passes through as Unicode are drawn with the gap built in (`᠂` 18.3 %,
//! `᠄` 19.5 %) and are left alone.
//!
//! Each encoding writes the gap with a space of its own. MenkShape uses Menksoft's `U+E263`:
//! 30.3 % of an em, the same width as `U+0020` in that font, and, being a private-use scalar, its
//! line-break class is AL, so no line can break between it and the mark. Z52 uses `U+202F`, the
//! narrow no-break space, which measures 19.5 % of an em in `z52.otf` — exactly the bearing that
//! font gives its own `᠄` and `᠁`, so the gap matches the ones already drawn in. An ordinary space
//! would be right in neither: it allows the break.
//!
//! Which side each mark takes was decided from the reference fonts' own bearings, side by side
//! with the rendered ink: the marks that end a clause take the gap before them, closing brackets
//! take it after, `·` takes both, and the four whose reference bearings are themselves ~0 take
//! none.
//!
//! Going the other way the gap is removed again, so a shape text and the hub text it came from
//! agree, and a round trip through either side is stable.

use crate::code_type::CodeType;

/// The side of a mark that carries the gap.
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Side {
    Before,
    After,
    Both,
}

impl Side {
    const fn before(self) -> bool {
        matches!(self, Side::Before | Side::Both)
    }

    const fn after(self) -> bool {
        matches!(self, Side::After | Side::Both)
    }
}

/// One encoding's gap: the space it is written with, and the marks that take it.
pub(crate) struct Gap {
    space: char,
    spaced: &'static [(char, Side)],
}

/// The gap an encoding writes, if it writes one.
pub(crate) fn of(code: CodeType) -> Option<&'static Gap> {
    match code {
        CodeType::MenkShape => Some(&MENK_SHAPE),
        CodeType::Z52 => Some(&Z52),
        _ => None,
    }
}

/// MenkShape: Menksoft's own space, over the 24 marks in `E234..=E261` that take a gap.
pub(crate) static MENK_SHAPE: Gap = Gap {
    space: '\u{E263}',
    spaced: MENK_SHAPE_SPACED,
};

/// Z52: the narrow no-break space, over the 17 of its own marks that take a gap. The 10 Mongolian
/// marks Z52 passes through as Unicode already carry an 18.3–19.5 % em bearing and are absent here.
pub(crate) static Z52: Gap = Gap {
    space: '\u{202F}',
    spaced: Z52_SPACED,
};

/// Every Menksoft punctuation mark that takes a gap, and on which side.
///
/// Sorted by code point; `side` is a binary search over it. The marks left out take no gap:
/// `E234` birga, `E23A` Todo soft hyphen, `E23B` Sibe syllable boundary, `E25E` ×, `E25F` ※,
/// `E260` -, `E261` |, and the digits `E244..=E24D`, which already have a 31 % em left bearing.
const MENK_SHAPE_SPACED: &[(char, Side)] = &[
    ('\u{E235}', Side::Before), // ᠁ ellipsis
    ('\u{E236}', Side::Before), // ᠂ comma
    ('\u{E237}', Side::Before), // ᠃ full stop
    ('\u{E238}', Side::Before), // ᠄ colon
    ('\u{E239}', Side::Before), // ᠅ four dots
    ('\u{E23C}', Side::Before), // ᠈ Manchu comma
    ('\u{E23D}', Side::Before), // ᠉ Manchu full stop
    ('\u{E243}', Side::Both),   // · middle dot
    ('\u{E24E}', Side::Before), // ⁈
    ('\u{E24F}', Side::Before), // ⁉
    ('\u{E250}', Side::Before), // !
    ('\u{E251}', Side::Before), // ?
    ('\u{E252}', Side::Before), // ;
    ('\u{E253}', Side::Before), // (
    ('\u{E254}', Side::After),  // )
    ('\u{E255}', Side::Before), // 〈
    ('\u{E256}', Side::After),  // 〉
    ('\u{E257}', Side::Before), // 〔
    ('\u{E258}', Side::After),  // 〕
    ('\u{E259}', Side::Before), // 《
    ('\u{E25A}', Side::After),  // 》
    ('\u{E25B}', Side::Before), // 『
    ('\u{E25C}', Side::After),  // 』
    ('\u{E25D}', Side::Before), // ,
];

/// Z52's own punctuation, `184F..=1863`. `1860` ×, `1861` ※, `1862` -, `1863` | take no gap, the
/// same four that take none in MenkShape.
const Z52_SPACED: &[(char, Side)] = &[
    ('\u{184F}', Side::Both),   // ·
    ('\u{1850}', Side::Before), // ⁈
    ('\u{1851}', Side::Before), // ⁉
    ('\u{1852}', Side::Before), // !
    ('\u{1853}', Side::Before), // ?
    ('\u{1854}', Side::Before), // ;
    ('\u{1855}', Side::Before), // (
    ('\u{1856}', Side::After),  // )
    ('\u{1857}', Side::Before), // 〈
    ('\u{1858}', Side::After),  // 〉
    ('\u{1859}', Side::Before), // 〔
    ('\u{185A}', Side::After),  // 〕
    ('\u{185B}', Side::Before), // 《
    ('\u{185C}', Side::After),  // 》
    ('\u{185D}', Side::Before), // 『
    ('\u{185E}', Side::After),  // 』
    ('\u{185F}', Side::Before), // ,
];

impl Gap {
    fn side(&self, c: char) -> Option<Side> {
        self.spaced
            .binary_search_by_key(&c, |&(mark, _)| mark)
            .ok()
            .map(|index| self.spaced[index].1)
    }

    /// Write the gap into shape text.
    ///
    /// A mark that already has the gap on the side it takes is left alone; an ordinary space there
    /// becomes the gap rather than being doubled; any other whitespace, and the edge of the text,
    /// already separates the mark and is left as it is. Everything else gets one inserted.
    pub(crate) fn insert(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len() + chars.len());
        for (index, &c) in chars.iter().enumerate() {
            let Some(side) = self.side(c) else {
                out.push(c);
                continue;
            };
            if side.before() {
                match out.chars().next_back() {
                    None => {}
                    Some(' ') => {
                        out.pop();
                        out.push(self.space);
                    }
                    Some(previous) if previous == self.space || previous.is_whitespace() => {}
                    Some(_) => out.push(self.space),
                }
            }
            out.push(c);
            if side.after() {
                match chars.get(index + 1) {
                    None => {}
                    // The ordinary space is dropped below, once it sits behind the gap.
                    Some(' ') => out.push(self.space),
                    Some(&next) if next == self.space || next.is_whitespace() => {}
                    Some(_) => out.push(self.space),
                }
            }
        }
        // An ordinary space now sitting right after a gap is the one it replaced.
        let mut collapsed = String::with_capacity(out.len());
        let mut previous = None;
        for c in out.chars() {
            if c == ' ' && previous == Some(self.space) {
                continue;
            }
            collapsed.push(c);
            previous = Some(c);
        }
        collapsed
    }

    /// Take the gap back out, so it does not reach the hub.
    ///
    /// Only a gap on the side the mark takes is removed; one anywhere else is a space the text
    /// meant to have and is left alone.
    pub(crate) fn strip(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut skip_next = false;
        for (index, &c) in chars.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }
            if c == self.space
                && chars
                    .get(index + 1)
                    .is_some_and(|&next| self.side(next).is_some_and(Side::before))
            {
                continue;
            }
            out.push(c);
            if self.side(c).is_some_and(Side::after) && chars.get(index + 1) == Some(&self.space) {
                skip_next = true;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MENK_GAP: &str = "\u{E263}";
    const Z52_GAP: &str = "\u{202F}";

    #[test]
    fn both_tables_are_sorted_so_the_search_finds_every_mark() {
        for gap in [&MENK_SHAPE, &Z52] {
            assert!(gap.spaced.windows(2).all(|pair| pair[0].0 < pair[1].0));
            for &(mark, _) in gap.spaced {
                assert!(gap.side(mark).is_some(), "{mark:?} not found");
            }
        }
        let count = |gap: &Gap, want: Side| gap.spaced.iter().filter(|(_, s)| *s == want).count();
        assert_eq!(MENK_SHAPE.spaced.len(), 24);
        assert_eq!(count(&MENK_SHAPE, Side::Before), 18);
        assert_eq!(count(&MENK_SHAPE, Side::After), 5);
        assert_eq!(count(&MENK_SHAPE, Side::Both), 1);
        assert_eq!(Z52.spaced.len(), 17);
        assert_eq!(count(&Z52, Side::Before), 11);
        assert_eq!(count(&Z52, Side::After), 5);
        assert_eq!(count(&Z52, Side::Both), 1);
    }

    #[test]
    fn only_the_encoding_that_owns_a_mark_spaces_it() {
        assert_eq!(
            Z52.insert("\u{E2B5}\u{E236}\u{E271}"),
            "\u{E2B5}\u{E236}\u{E271}"
        );
        assert_eq!(
            MENK_SHAPE.insert("\u{186B}\u{1852}\u{1867}"),
            "\u{186B}\u{1852}\u{1867}"
        );
    }

    #[test]
    fn marks_that_take_no_gap_are_untouched() {
        for mark in [
            '\u{E234}', '\u{E23A}', '\u{E23B}', '\u{E25E}', '\u{E25F}', '\u{E260}', '\u{E261}',
        ] {
            let text = format!("\u{E2B5}{mark}\u{E271}");
            assert_eq!(MENK_SHAPE.insert(&text), text, "{mark:?}");
            assert_eq!(MENK_SHAPE.strip(&text), text, "{mark:?}");
        }
        // Z52 leaves x, the reference mark, the hyphen and the bar alone, and every Mongolian mark
        // it passes through as Unicode.
        for mark in [
            '\u{1860}', '\u{1861}', '\u{1862}', '\u{1863}', '\u{1802}', '\u{1800}',
        ] {
            let text = format!("\u{186B}{mark}\u{1867}");
            assert_eq!(Z52.insert(&text), text, "{mark:?}");
            assert_eq!(Z52.strip(&text), text, "{mark:?}");
        }
    }

    #[test]
    fn a_gap_is_written_on_the_side_the_mark_takes() {
        assert_eq!(
            MENK_SHAPE.insert("\u{E2B5}\u{E236}\u{E271}"),
            format!("\u{E2B5}{MENK_GAP}\u{E236}\u{E271}")
        );
        assert_eq!(
            MENK_SHAPE.insert("\u{E2B5}\u{E254}\u{E271}"),
            format!("\u{E2B5}\u{E254}{MENK_GAP}\u{E271}")
        );
        assert_eq!(
            MENK_SHAPE.insert("\u{E2B5}\u{E243}\u{E271}"),
            format!("\u{E2B5}{MENK_GAP}\u{E243}{MENK_GAP}\u{E271}")
        );
        assert_eq!(
            Z52.insert("\u{186B}\u{1852}\u{1867}"),
            format!("\u{186B}{Z52_GAP}\u{1852}\u{1867}")
        );
        assert_eq!(
            Z52.insert("\u{186B}\u{1856}\u{1867}"),
            format!("\u{186B}\u{1856}{Z52_GAP}\u{1867}")
        );
        assert_eq!(
            Z52.insert("\u{186B}\u{184F}\u{1867}"),
            format!("\u{186B}{Z52_GAP}\u{184F}{Z52_GAP}\u{1867}")
        );
    }

    #[test]
    fn an_ordinary_space_is_replaced_not_doubled() {
        assert_eq!(
            MENK_SHAPE.insert("\u{E2B5} \u{E236}"),
            format!("\u{E2B5}{MENK_GAP}\u{E236}")
        );
        assert_eq!(
            MENK_SHAPE.insert("\u{E254} \u{E271}"),
            format!("\u{E254}{MENK_GAP}\u{E271}")
        );
        assert_eq!(
            Z52.insert("\u{186B} \u{1852}"),
            format!("\u{186B}{Z52_GAP}\u{1852}")
        );
        let once = MENK_SHAPE.insert("\u{E2B5}\u{E263}\u{E236}\u{E271}");
        assert_eq!(once, "\u{E2B5}\u{E263}\u{E236}\u{E271}");
        assert_eq!(MENK_SHAPE.insert(&once), once);
    }

    #[test]
    fn other_whitespace_already_separates_the_mark() {
        for space in ["\t", "\n", "\r\n"] {
            let text = format!("\u{E2B5}{space}\u{E236}");
            assert_eq!(MENK_SHAPE.insert(&text), text, "{space:?}");
        }
        assert_eq!(MENK_SHAPE.insert("\u{E236}\u{E271}"), "\u{E236}\u{E271}");
        assert_eq!(MENK_SHAPE.insert("\u{E2B5}\u{E254}"), "\u{E2B5}\u{E254}");
    }

    #[test]
    fn two_marks_side_by_side_share_one_gap() {
        // The middle dot takes it after, the semicolon before: one gap between them, not two.
        assert_eq!(
            Z52.insert("\u{186B}\u{184F}\u{1854}\u{1867}"),
            format!("\u{186B}{Z52_GAP}\u{184F}{Z52_GAP}\u{1854}\u{1867}")
        );
    }

    #[test]
    fn strip_undoes_insert() {
        for (gap, text) in [
            (&MENK_SHAPE, "\u{E2B5}\u{E236}\u{E271}"),
            (&MENK_SHAPE, "\u{E2B5}\u{E254}\u{E271}"),
            (&MENK_SHAPE, "\u{E2B5}\u{E243}\u{E271}"),
            (&MENK_SHAPE, "\u{E2B5}\u{E236}\u{E271}\u{E254}\u{E2B5}"),
            (&MENK_SHAPE, "\u{E2B5}\u{E234}\u{E271}"),
            (&MENK_SHAPE, "plain text"),
            (&MENK_SHAPE, ""),
            (&Z52, "\u{186B}\u{1852}\u{1867}"),
            (&Z52, "\u{186B}\u{1856}\u{1867}"),
            (&Z52, "\u{186B}\u{184F}\u{1854}\u{1867}"),
            (&Z52, "\u{186B}\u{1802}\u{1867}"),
        ] {
            assert_eq!(gap.strip(&gap.insert(text)), text, "{text:?}");
        }
    }

    #[test]
    fn a_space_the_text_meant_to_have_survives_the_round_trip() {
        assert_eq!(
            MENK_SHAPE.strip("\u{E2B5}\u{E263}\u{E271}"),
            "\u{E2B5}\u{E263}\u{E271}"
        );
        assert_eq!(
            MENK_SHAPE.strip("\u{E236}\u{E263}\u{E271}"),
            "\u{E236}\u{E263}\u{E271}"
        );
        assert_eq!(
            MENK_SHAPE.strip("\u{E2B5}\u{E263}\u{E254}"),
            "\u{E2B5}\u{E263}\u{E254}"
        );
        assert_eq!(
            Z52.strip("\u{186B}\u{202F}\u{1867}"),
            "\u{186B}\u{202F}\u{1867}"
        );
    }
}
