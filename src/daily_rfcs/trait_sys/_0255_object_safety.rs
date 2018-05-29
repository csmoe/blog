//! Restrict which traits can be used to make trait objects
//!
//! # Note
//! ## Object-safe method
//!  1. require `Self : Sized`; **OR**
//!  2. meet all of the following conditions:
//!       - must not have any type parameters; **AND**
//!       - must have a receiver that has type `Self` or which dereferences to the `Self` type;
//!            - for now, this means `self`, `&self`, `&mut self`, or `self: Box<Self>`, but eventually this should be extended to custom types like self: Rc<Self> and so forth.
//!       - must not use `Self` (in the future, where we allow arbitrary types for the receiver, Self may only be used for the type of the receiver and only where we allow Sized? types).
//!
//!  # Object-safe trait
//!  - all of its methos are object safe.
//!  - the trait does not require that `Self : Sized`
//!
//! # How to bypass object-safety rules
//!  - split trait into object-safe and non-object-safe parts.
//!  - add `where Self: Sized` to non-object-safe methods.(derive from Object-safe method rule 1)
//!
//! [RFC: 0255-object-safety](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)

struct RFCs(i32);
