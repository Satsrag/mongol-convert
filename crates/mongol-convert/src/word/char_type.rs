//! Port of `letter/from/CharType.java`. Classifies a code point as Mongolian or not; drives the
//! space-padding in `ShapeWordFragment::get_locate_key` (head/tail context).

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum CharType {
    Mongolian,
    Other,
}
