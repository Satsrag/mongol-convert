//! Zvvnmod -> strict Z52. Keys on `get_key` (unpadded). Rust extends the four active historical
//! Java target mappings to the complete 21-position Z52 punctuation repertoire.

use crate::code_mapper::StaticMap;
use crate::shape::rule::ShapeTranslateRule;
use crate::tables::to_z52::TO_Z52;
use crate::tables::z52_punctuation;
use crate::unicode::zvvnmod;
use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

static MAP: StaticMap = StaticMap::new(TO_Z52);

pub(crate) struct Z52To;

impl ShapeTranslateRule for Z52To {
    fn is_translate_code_point(&self, c: char) -> bool {
        zvvnmod::is_zvvnmod_code(c) || z52_punctuation::contains(c)
    }

    fn contains(&self, fragment: &ShapeWordFragment) -> bool {
        z52_punctuation::get(&fragment.get_key()).is_some() || MAP.contains_key(&fragment.get_key())
    }

    fn get_mapper_code(&self, _pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str> {
        z52_punctuation::get(&fragment.get_key()).or_else(|| MAP.get(&fragment.get_key()))
    }

    fn get_char_type(&self, _c: char) -> Option<CharType> {
        None
    }
}
