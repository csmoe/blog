//! `Self` is not `Sized` by default.
//!
//! ## Note
//!
//! - Remove the `Sized` default for the implicitly declared `Self` parameter on traits.
//! - Make it "object unsafe" for a trait to inherit from `Sized`.
//!
//! [RFC: 0546-Self-not-sized-by-default](https://github.com/rust-lang/rfcs/blob/master/text/0546-Self-not-sized-by-default.md)

struct RFCs(i32);
