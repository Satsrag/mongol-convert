//! Port of the `ShapeTranslateRule` Java interface.

use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

pub(crate) trait ShapeTranslateRule: Sync {
    /// Whether `c` participates in this encoding's word shaping (vs a passthrough char).
    fn is_translate_code_point(&self, c: char) -> bool;

    /// Whether the fragment's lookup key is present in this rule's mapper.
    fn contains(&self, fragment: &ShapeWordFragment) -> bool;

    /// The mapped output for a committed fragment, or `None` if its key is absent.
    /// `pre` is the concatenated content of all preceding fragments in the word (for stateful rules).
    fn get_mapper_code(&self, pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str>;

    /// CharType of a code point (used to set fragment head/tail). `None` mirrors Java returning null
    /// for the to-shape rules, which key on `get_key` (unpadded) and so ignore head/tail.
    fn get_char_type(&self, c: char) -> Option<CharType>;
}
