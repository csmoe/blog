//! Check all types for well-formedness with respect to the bounds of type variables.
//!
//! Allow bounds on formal type variable in structs and enums. Check these bounds are satisfied
//! wherever the struct or enum is used with actual type parameters.
//!
//! [RFC: 0034-bounded-type-parameters](https://github.com/rust-lang/rfcs/blob/master/text/0034-bounded-type-parameters.md)
