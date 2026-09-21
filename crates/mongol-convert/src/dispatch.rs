//! Rule dispatch — the Rust replacement for Java's `@Rule/@From/@To` annotation registry and the
//! Spring `RuleHolder`s. Compile-time exhaustive `match` over `CodeType`; rules are zero-sized
//! statics (they hold no data — the big tables are separate statics), so no `LazyLock` is needed.
//!
//! Letter rules (Delehi/MenkLetter) arrive in Step 6. Until then letter dispatch returns
//! `MissTranslateRule` (the router currently reports letter encodings as Unsupported anyway).

use crate::code_type::CodeType;
use crate::error::MongolConvertError;
use crate::letter::delehi::{DelehiFrom, DelehiTo};
use crate::letter::menk::{MenkLetterFrom, MenkLetterTo};
use crate::letter::rule::{LetterTranslateRuleFrom, LetterTranslateRuleTo};
use crate::shape::menk_from::MenkShapeFrom;
use crate::shape::menk_to::MenkShapeTo;
use crate::shape::rule::ShapeTranslateRule;
use crate::shape::z52_from::Z52From;
use crate::shape::z52_to::Z52To;

static Z52_FROM: Z52From = Z52From;
static Z52_TO: Z52To = Z52To;
static MENK_SHAPE_FROM: MenkShapeFrom = MenkShapeFrom;
static MENK_SHAPE_TO: MenkShapeTo = MenkShapeTo;
static DELEHI_FROM: DelehiFrom = DelehiFrom;
static DELEHI_TO: DelehiTo = DelehiTo;
static MENK_LETTER_FROM: MenkLetterFrom = MenkLetterFrom;
static MENK_LETTER_TO: MenkLetterTo = MenkLetterTo;

pub(crate) fn shape_from_rule(ct: CodeType) -> Result<&'static dyn ShapeTranslateRule, MongolConvertError> {
    match ct {
        CodeType::Z52 => Ok(&Z52_FROM),
        CodeType::MenkShape => Ok(&MENK_SHAPE_FROM),
        _ => Err(MongolConvertError::MissTranslateRule(ct)),
    }
}

pub(crate) fn shape_to_rule(ct: CodeType) -> Result<&'static dyn ShapeTranslateRule, MongolConvertError> {
    match ct {
        CodeType::Z52 => Ok(&Z52_TO),
        CodeType::MenkShape => Ok(&MENK_SHAPE_TO),
        _ => Err(MongolConvertError::MissTranslateRule(ct)),
    }
}

pub(crate) fn letter_from_rule(
    ct: CodeType,
) -> Result<&'static dyn LetterTranslateRuleFrom, MongolConvertError> {
    match ct {
        CodeType::Delehi => Ok(&DELEHI_FROM),
        CodeType::MenkLetter => Ok(&MENK_LETTER_FROM),
        // Oyun/Utn57 are handled (Unsupported) in the router before dispatch.
        _ => Err(MongolConvertError::MissTranslateRule(ct)),
    }
}

pub(crate) fn letter_to_rule(ct: CodeType) -> Result<&'static dyn LetterTranslateRuleTo, MongolConvertError> {
    match ct {
        CodeType::Delehi => Ok(&DELEHI_TO),
        CodeType::MenkLetter => Ok(&MENK_LETTER_TO),
        _ => Err(MongolConvertError::MissTranslateRule(ct)),
    }
}
