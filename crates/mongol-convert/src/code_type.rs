//! Encoding type system. Port of `translate/enumeration/CodeType.java` + `CodeSeries.java`.
//!
//! Java `CodeType.get` matches `enumName.toLowerCase()`, e.g. `Menk_Shape` -> `"menk_shape"`.
//! That lowercased-enum-name form is the **canonical** string here; the PHP underscore-stripped
//! form (`"menkshape"`) is accepted as a compatibility alias (design decision #1).

use crate::error::MongolConvertError;
use std::str::FromStr;

/// Dispatch classification: a code is decoded/encoded either via the letter rules or the shape rules.
/// Port of `CodeSeries.java`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CodeSeries {
    Letter,
    Shape,
}

/// The supported Mongolian encodings. One variant per Java `CodeType` (in declaration order).
/// `Zvvnmod` is the intermediate hub. Port of `CodeType.java` lines 14-20.
///
/// `Oyun` and `Utn57` are parseable. `Oyun` remains unsupported in both directions; UTN #57
/// converts both ways through the in-process `zvvnmod-utn57` backend.
///
/// `Utn57Shape` has no Java counterpart: it is the written-unit spelling of a UTN #57 text
/// (ᠰᠠᠢᠨ as `SAIIA`), read and written through `Utn57`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CodeType {
    Zvvnmod,
    Delehi,
    MenkShape,
    MenkLetter,
    Oyun,
    Utn57,
    Z52,
    Utn57Shape,
}

impl CodeType {
    /// The code's series. Port of Java's stored `codeSeries` final field, as a `const fn`.
    pub const fn code_series(self) -> CodeSeries {
        match self {
            CodeType::Zvvnmod => CodeSeries::Shape,
            CodeType::Delehi => CodeSeries::Letter,
            CodeType::MenkShape => CodeSeries::Shape,
            CodeType::MenkLetter => CodeSeries::Letter,
            CodeType::Oyun => CodeSeries::Letter,
            CodeType::Utn57 => CodeSeries::Letter,
            CodeType::Z52 => CodeSeries::Shape,
            CodeType::Utn57Shape => CodeSeries::Letter,
        }
    }

    /// Canonical string form (the Java lowercased-enum-name form).
    pub const fn canonical_str(self) -> &'static str {
        match self {
            CodeType::Zvvnmod => "zvvnmod",
            CodeType::Delehi => "delehi",
            CodeType::MenkShape => "menk_shape",
            CodeType::MenkLetter => "menk_letter",
            CodeType::Oyun => "oyun",
            CodeType::Utn57 => "utn57",
            CodeType::Z52 => "z52",
            CodeType::Utn57Shape => "utn57_shape",
        }
    }

    /// Case-insensitive lookup, mirroring Java `CodeType.get`. Accepts the canonical Java form
    /// and the PHP underscore-stripped alias. Errors with [`MongolConvertError::UnsupportedEnumType`].
    pub fn get(value: &str) -> Result<CodeType, MongolConvertError> {
        value.parse()
    }
}

impl FromStr for CodeType {
    type Err = MongolConvertError;

    fn from_str(s: &str) -> Result<CodeType, MongolConvertError> {
        match s.to_ascii_lowercase().as_str() {
            "zvvnmod" => Ok(CodeType::Zvvnmod),
            "delehi" => Ok(CodeType::Delehi),
            "menk_shape" | "menkshape" => Ok(CodeType::MenkShape),
            "menk_letter" | "menkletter" => Ok(CodeType::MenkLetter),
            "oyun" => Ok(CodeType::Oyun),
            "utn57" => Ok(CodeType::Utn57),
            "z52" => Ok(CodeType::Z52),
            "utn57_shape" | "utn57shape" => Ok(CodeType::Utn57Shape),
            _ => Err(MongolConvertError::UnsupportedEnumType(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn series_matches_java() {
        assert_eq!(CodeType::Zvvnmod.code_series(), CodeSeries::Shape);
        assert_eq!(CodeType::Delehi.code_series(), CodeSeries::Letter);
        assert_eq!(CodeType::MenkShape.code_series(), CodeSeries::Shape);
        assert_eq!(CodeType::MenkLetter.code_series(), CodeSeries::Letter);
        assert_eq!(CodeType::Oyun.code_series(), CodeSeries::Letter);
        assert_eq!(CodeType::Utn57.code_series(), CodeSeries::Letter);
        assert_eq!(CodeType::Z52.code_series(), CodeSeries::Shape);
    }

    #[test]
    fn parse_canonical_java_form() {
        assert_eq!("menk_shape".parse::<CodeType>().unwrap(), CodeType::MenkShape);
        assert_eq!("Menk_Shape".parse::<CodeType>().unwrap(), CodeType::MenkShape);
        assert_eq!("ZVVNMOD".parse::<CodeType>().unwrap(), CodeType::Zvvnmod);
        assert_eq!("z52".parse::<CodeType>().unwrap(), CodeType::Z52);
    }

    #[test]
    fn parse_php_alias() {
        assert_eq!("menkshape".parse::<CodeType>().unwrap(), CodeType::MenkShape);
        assert_eq!("menkletter".parse::<CodeType>().unwrap(), CodeType::MenkLetter);
    }

    #[test]
    fn canonical_roundtrips() {
        for ct in [
            CodeType::Zvvnmod,
            CodeType::Delehi,
            CodeType::MenkShape,
            CodeType::MenkLetter,
            CodeType::Oyun,
            CodeType::Utn57,
            CodeType::Z52,
        ] {
            assert_eq!(ct.canonical_str().parse::<CodeType>().unwrap(), ct);
        }
    }

    #[test]
    fn unknown_errors() {
        assert!(matches!(
            "nope".parse::<CodeType>(),
            Err(MongolConvertError::UnsupportedEnumType(_))
        ));
        assert!(CodeType::get("").is_err());
    }

    #[test]
    fn utn57_shape_parses_and_prints_like_menk_shape() {
        assert_eq!(CodeType::get("utn57_shape").unwrap(), CodeType::Utn57Shape);
        assert_eq!(CodeType::get("UTN57SHAPE").unwrap(), CodeType::Utn57Shape);
        assert_eq!(CodeType::Utn57Shape.canonical_str(), "utn57_shape");
        assert_eq!(CodeType::Utn57Shape.code_series(), CodeSeries::Letter);
    }
}
