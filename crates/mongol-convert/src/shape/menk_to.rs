//! Port of `shape/to/menk/MenkShapeTranslateRuleTo.java`. Zvvnmod -> Menk-shape. Keys on `get_key`.
//! The only stateful shape rule: `reslove_tsatslaga` overrides the key "" to "" when the
//! immediately preceding fragment ended in a Zvvnmod tail code (depends on prior fragment content).

use crate::code_mapper::StaticMap;
use crate::shape::rule::ShapeTranslateRule;
use crate::tables::zvvnmod_birga::zvvnmod_birga_to_menk_shape;
use crate::tables::to_menk_shape::TO_MENK_SHAPE;
use crate::unicode::zvvnmod;
use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

static MAP: StaticMap = StaticMap::new(TO_MENK_SHAPE);

pub(crate) struct MenkShapeTo;

impl ShapeTranslateRule for MenkShapeTo {
    fn is_translate_code_point(&self, c: char) -> bool {
        zvvnmod::is_zvvnmod_code(c) || zvvnmod::is_zvvnmod_punctuation(c)
    }

    fn contains(&self, fragment: &ShapeWordFragment) -> bool {
        let key = fragment.get_key();
        zvvnmod_birga_to_menk_shape(&key).is_some() || MAP.contains_key(&key)
    }

    fn get_mapper_code(&self, pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str> {
        let key = fragment.get_key();
        if let Some(menk_shape) = zvvnmod_birga_to_menk_shape(&key) {
            return Some(menk_shape);
        }
        if let Some(r) = reslove_tsatslaga(pre, &key) {
            return Some(r);
        }
        MAP.get(&key)
    }

    fn get_char_type(&self, _c: char) -> Option<CharType> {
        None
    }
}

fn reslove_tsatslaga(pre: &[char], key: &str) -> Option<&'static str> {
    if key != "\u{e00d}" || pre.is_empty() {
        return None;
    }
    let last = *pre.last().unwrap();
    if zvvnmod::is_zvvnmod_tail_code(last) {
        Some("\u{e26a}")
    } else {
        None
    }
}
