//! Add type ascription to expressions.
//!
//! ```rust
//! // Current.
//! let x: T = {
//!     let temp: U<_> = foo();
//!     temp
//! };
//!
//! // With type ascription.
//! let x: T = foo(): U<_>;
//! ```
//! # Note
//! Type ascription expressions inherit their 'lvalue-ness' from their underlying expressions. I.e., `e: T` is an lvalue if `e` is an lvalue, and an rvalue otherwise.
//!
//! [RFC: 0803-type-ascription](https://github.com/rust-lang/rfcs/blob/master/text/0803-type-ascription.md)
