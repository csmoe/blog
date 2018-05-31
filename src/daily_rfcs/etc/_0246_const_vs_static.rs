//! Divide global declarations into two categories(const-value/global-variable)
//!
//! ## Note
//!
//! - constants declare constant values. These represent a value, not a memory address.
//! - statics declare global variables. These represent a memory address.
//!
//! [RFC: 0246-const-vs-static](https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md)

struct RFCs(i32);
