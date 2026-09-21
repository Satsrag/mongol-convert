//! Lookup tables. The files under `generated/` are produced by `tools/table-gen/TableDumper.java`
//! (dumped from the live Java maps) and are committed verbatim — DO NOT edit them by hand; re-run
//! the dumper instead. Each is a sorted, binary-searchable static.

/// Rust-owned ZVVNMOD Birga rules that intentionally override historical Java behavior.
pub mod zvvnmod_birga;
/// Rust-owned strict Z52 punctuation encoding backed by first-party font and IME evidence.
pub mod z52_punctuation;

#[path = "generated/from_z52.rs"]
pub mod from_z52;
#[path = "generated/to_z52.rs"]
pub mod to_z52;
#[path = "generated/from_menk_shape.rs"]
pub mod from_menk_shape;
#[path = "generated/to_menk_shape.rs"]
pub mod to_menk_shape;
#[path = "generated/z52_block.rs"]
pub mod z52_block;
#[path = "generated/zvvnmod_block.rs"]
pub mod zvvnmod_block;
#[path = "generated/menk_shape_block.rs"]
pub mod menk_shape_block;
#[path = "generated/from_delehi.rs"]
pub mod from_delehi;
#[path = "generated/to_delehi.rs"]
pub mod to_delehi;
#[path = "generated/from_menk_letter.rs"]
pub mod from_menk_letter;
#[path = "generated/to_menk_letter.rs"]
pub mod to_menk_letter;

#[cfg(test)]
mod tests {
    //! Guards that every generated table is ascending-sorted by Rust ordering, so the binary
    //! searches in `code_mapper` / `unicode` are valid. (The Java dumper sorts by code point;
    //! for BMP that equals Rust's UTF-8/`char` order — these tests prove it for the real data.)

    fn map_sorted_unique(m: &[(&str, &str)]) -> bool {
        m.windows(2).all(|w| w[0].0 < w[1].0)
    }
    fn chars_sorted_unique(s: &[char]) -> bool {
        s.windows(2).all(|w| w[0] < w[1])
    }

    #[test]
    fn mappers_sorted_unique() {
        assert!(map_sorted_unique(super::from_z52::FROM_Z52));
        assert!(map_sorted_unique(super::to_z52::TO_Z52));
        assert!(map_sorted_unique(super::from_menk_shape::FROM_MENK_SHAPE));
        assert!(map_sorted_unique(super::to_menk_shape::TO_MENK_SHAPE));
    }

    #[test]
    fn char_sets_sorted_unique() {
        assert!(chars_sorted_unique(super::z52_block::Z52_CODES));
        assert!(chars_sorted_unique(super::z52_block::Z52_CODE_PUNCTUATIONS));
        assert!(chars_sorted_unique(super::zvvnmod_block::ZVVNMOD_CODES));
        assert!(chars_sorted_unique(super::zvvnmod_block::ZVVNMOD_TAIL_CODES));
        assert!(chars_sorted_unique(super::zvvnmod_block::ZVVNMOD_PUNCTUATIONS));
        assert!(chars_sorted_unique(super::zvvnmod_block::TO_Z52_PUNCTUATIONS));
        assert!(chars_sorted_unique(super::menk_shape_block::MENK_SHAPE_NOT_SUPPORT));
    }

    #[test]
    fn expected_counts() {
        assert_eq!(super::from_z52::FROM_Z52.len(), 440);
        assert_eq!(super::to_z52::TO_Z52.len(), 144);
        assert_eq!(super::z52_punctuation::UNICODE_TO_Z52_PUNCTUATION.len(), 21);
        assert!(super::z52_punctuation::UNICODE_TO_Z52_PUNCTUATION
            .windows(2)
            .all(|window| window[0].0 < window[1].0));
        assert_eq!(super::from_menk_shape::FROM_MENK_SHAPE.len(), 2738);
        assert_eq!(super::to_menk_shape::TO_MENK_SHAPE.len(), 223);
    }

    #[test]
    fn letter_mappers_sorted_unique() {
        use super::*;
        for m in [
            from_delehi::FROM_DELEHI,
            from_delehi::FROM_DELEHI_CHAGH,
            from_delehi::FROM_DELEHI_HUNDII,
            from_delehi::FROM_DELEHI_SAARMAG,
            to_delehi::TO_DELEHI,
            from_menk_letter::FROM_MENK_LETTER,
            from_menk_letter::FROM_MENK_LETTER_CHAGH,
            from_menk_letter::FROM_MENK_LETTER_HUNDII,
            from_menk_letter::FROM_MENK_LETTER_SAARMAG,
            from_menk_letter::FROM_MENK_LETTER_W_WITH_EHSHIG,
            to_menk_letter::TO_MENK_LETTER,
            to_menk_letter::TO_MENK_LETTER_HUNDII,
            to_menk_letter::TO_MENK_LETTER_CHAGH,
        ] {
            assert!(map_sorted_unique(m));
        }
    }
}
