//! Allow attrubutes in `match` arms.
//!
//! # Note
//! ```rust
//! match x {
//!    #[attr]
//!    Thing => {}
//!
//!    #[attr]
//!    Foo | Bar => {}
//!
//!    #[attr]
//!    _ => {}
//! }
//! ```
//!
//! [RFC: 0049-match-arm-attributes](https://github.com/nox/rust-rfcs/blob/master/text/0049-match-arm-attributes.md)
