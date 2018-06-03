//! Allow `loop { ... }` expr returns a value via `break value`.
//!
//! ## Note
//! ### Syntax
//! Four forms of break will be supported:
//!
//! 1. `break`;
//! 2. `break 'label`;
//! 3. `break EXPR`;
//! 4. `break 'label EXPR`;
//!
//! where `'label` is the name of a `loop` and `EXPR` is an expression.
//! `break` and `break 'label` become equivalent to `break ()` and `break 'label ()` respectively.
//!
//! [RFC: 1624-loop-break-value](https://github.com/rust-lang/rfcs/blob/master/text/1624-loop-break-value.md)

struct RFCs(i32);
