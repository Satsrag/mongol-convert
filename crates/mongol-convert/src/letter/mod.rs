//! LETTER subsystem: Delehi / Menk-letter <-> the Zvvnmod hub (Oyun + Utn57 are deferred).
//! The from-direction uses `LetterWord` (with Nature); the to-direction reuses the shape word model.

pub(crate) mod delehi;
pub(crate) mod from_translator;
pub(crate) mod menk;
pub(crate) mod rule;
pub(crate) mod to_translator;
