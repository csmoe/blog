//! Remove `priv` keyword from rust-lang
//!
//! # Note
//! * This RFC demotes the identifier priv from being a keyword to being a reserved keyword.
//! * RFC also forbids visibility qualifiers in front of enum variants entirely, as they no longer serve any purpose.
//!
//! [RFC: 0026-remove-priv](https://github.com/rust-lang/rfcs/blob/master/text/0026-remove-priv.md)
struct RFCs(i32);
