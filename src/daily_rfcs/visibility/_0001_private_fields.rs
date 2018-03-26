//! all struct fields are private by default
//!
//! # Note
//! * Private fields cannot be bound in patterns (both in irrefutable and refutable contexts, i.e. `let` and `match` statements).
//! * Private fields cannot be specified outside of the defining module when constructing a tuple struct.
//!
//! [RFC: 0001-private-fields](https://github.com/rust-lang/rfcs/blob/master/text/0001-private-fields.md)

struct RFCs(i32);
