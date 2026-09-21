//! Port of `shape/from/menk/MenkShapeTranslateRuleFrom.java`. Menk-shape -> Zvvnmod.
//! Keys on `get_locate_key`. Input gate excludes `notSupportSet` then accepts two PUA ranges.

use crate::code_mapper::StaticMap;
use crate::shape::rule::ShapeTranslateRule;
use crate::tables::zvvnmod_birga::menk_shape_birga_to_zvvnmod;
use crate::tables::from_menk_shape::FROM_MENK_SHAPE;
use crate::tables::menk_shape_block::MENK_SHAPE_NOT_SUPPORT;
use crate::unicode::sorted_contains;
use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

static MAP: StaticMap = StaticMap::new(FROM_MENK_SHAPE);

pub(crate) struct MenkShapeFrom;

impl ShapeTranslateRule for MenkShapeFrom {
    fn is_translate_code_point(&self, c: char) -> bool {
        if sorted_contains(MENK_SHAPE_NOT_SUPPORT, c) {
            return false;
        }
        ('\u{e263}'..='\u{e34a}').contains(&c) || ('\u{e234}'..='\u{e261}').contains(&c)
    }

    fn contains(&self, fragment: &ShapeWordFragment) -> bool {
        MAP.contains_key(&fragment.get_locate_key())
    }

    fn get_mapper_code(&self, _pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str> {
        if let [character] = fragment.content() {
            if let Some(zvvnmod) = menk_shape_birga_to_zvvnmod(*character) {
                return Some(zvvnmod);
            }
        }
        MAP.get(&fragment.get_locate_key())
    }

    fn get_char_type(&self, c: char) -> Option<CharType> {
        // Java isWordCodePoint: E264..=E34F
        Some(if ('\u{e264}'..='\u{e34f}').contains(&c) {
            CharType::Mongolian
        } else {
            CharType::Other
        })
    }
}
