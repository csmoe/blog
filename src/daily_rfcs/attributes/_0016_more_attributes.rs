//! Allow attributes on more places inside functions, such as statements, blocks and expressions.
//!
//! # Note
//! * attributes bind tighter than any operator, that is `#[attr] x op y` is always parsed as `(#[attr] x) op y`.
//! * Attributes would be disallowed on `if`.
//!
//! [RFC: 0016-more-attributes](https://github.com/rust-lang/rfcs/blob/master/text/0016-more-attributes.md)
struct RFCs(i32);
