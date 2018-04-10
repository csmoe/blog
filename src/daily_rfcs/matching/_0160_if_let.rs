//! Introduce `if let PAT = EXPR { BODY }` construct.
//! ```rust
//! if let Some(x) = opt_val {
//!     do(x);
//! } else if cond() {
//!     do();
//! } else {
//!     do();
//! }
//! ```
//! desugar:
//! ```rust
//! match opt_val {
//!     Some(x) => {
//!         do(x);
//!     }
//!     _ if cond() => {}
//!     _ => {}
//! }
//! ```
//! # Note
//! * `else if` => `_ if guard => {}`
//! * `else` => `_ => {}`
//!
//! [RFC: 0160-if-let](https://github.com/rust-lang/rfcs/blob/master/text/0160-if-let.md)
struct RFCs(i32);
