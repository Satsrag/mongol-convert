//! Static, sorted, binary-searchable `&str -> &str` map — the Rust analog of the Java `CodeMapper`
//! (lookup side only). The duplicate-key guard that Java enforces at build time lives in the
//! table-gen dump step, so by the time a table reaches here it is unique and sorted.

pub struct StaticMap {
    entries: &'static [(&'static str, &'static str)],
}

impl StaticMap {
    pub const fn new(entries: &'static [(&'static str, &'static str)]) -> Self {
        Self { entries }
    }

    /// Java `CodeMapper.get`: the mapped value, or `None` if absent.
    pub fn get(&self, key: &str) -> Option<&'static str> {
        self.entries
            .binary_search_by(|(k, _)| (*k).cmp(key))
            .ok()
            .map(|i| self.entries[i].1)
    }

    /// Java `CodeMapper.containsKey`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.binary_search_by(|(k, _)| (*k).cmp(key)).is_ok()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::from_z52::FROM_Z52;

    #[test]
    fn lookup_against_generated_from_z52() {
        let m = StaticMap::new(FROM_Z52);
        assert_eq!(m.len(), 440);
        // From the dumped table: padded punctuation key U+184F -> U+00B7 (middle dot).
        assert_eq!(m.get(" \u{184f}"), Some("\u{b7}"));
        assert!(m.contains_key(" \u{184f} "));
        assert_eq!(m.get("definitely-not-a-key"), None);
        assert!(!m.contains_key(""));
    }
}
