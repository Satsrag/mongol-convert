//! Vowel-harmony nature of a Mongolian letter. Port of `word/Nature.java`.
//!
//! Only the enum is needed by the unicode classifier so far. The one-way `SAARMAG -> X`
//! latch behaviour used by the word model is added in Step 3 when `LetterWord`/`ShapeWord` land.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Nature {
    /// 阳性 (masculine)
    Chagh,
    /// 阴性 (feminine)
    Hundii,
    /// 中性 (neutral) — the default that latches to the first non-neutral nature seen.
    Saarmag,
}
