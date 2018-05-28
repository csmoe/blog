//! Bounds on trait objects should be separated with `+`
//!
//! ## Note
//! `&Trait:Share + Send` => `&Trait + Share + Send`
//!
//! (`+` will not be permitted in as without parentheses.)
//!
//! [RFC: 0087-trait-bounds-with-plus](https://github.com/rust-lang/rfcs/blob/master/text/0087-trait-bounds-with-plus.md)

struct RFCs(i32);
