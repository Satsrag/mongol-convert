//! Unicode-block code-point classifiers.
//!
//! `mgl` (small, fully-specified sets/ranges) is hand-ported. The large membership sets of the
//! Z52 and Zvvnmod blocks are **generated** by `tools/table-gen` (dumped from the live Java sets)
//! and looked up here via binary search.

pub mod mgl;
pub mod z52;
pub mod zvvnmod;

/// Membership test on an ascending-sorted `char` slice. The generated tables are sorted, and
/// `tables::tests` guards that invariant, so binary search is valid.
#[inline]
pub(crate) fn sorted_contains(slice: &[char], c: char) -> bool {
    slice.binary_search(&c).is_ok()
}
