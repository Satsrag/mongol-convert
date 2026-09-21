//! Port of `shape/from/z52/Z52TranslateRuleFrom.java`. Z52 -> Zvvnmod. Keys on `get_locate_key`.

use crate::code_mapper::StaticMap;
use crate::shape::rule::ShapeTranslateRule;
use crate::tables::from_z52::FROM_Z52;
use crate::unicode::z52;
use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

static MAP: StaticMap = StaticMap::new(FROM_Z52);

pub(crate) struct Z52From;

impl ShapeTranslateRule for Z52From {
    fn is_translate_code_point(&self, c: char) -> bool {
        z52::is_z52_code(c) || z52::is_z52_code_punctuation(c)
    }

    fn contains(&self, fragment: &ShapeWordFragment) -> bool {
        MAP.contains_key(&fragment.get_locate_key())
    }

    fn get_mapper_code(&self, _pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str> {
        MAP.get(&fragment.get_locate_key())
    }

    fn get_char_type(&self, c: char) -> Option<CharType> {
        Some(if z52::is_z52_code(c) {
            CharType::Mongolian
        } else {
            CharType::Other
        })
    }
}
