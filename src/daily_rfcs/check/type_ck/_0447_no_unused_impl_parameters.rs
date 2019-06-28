//! Disallow unconstrained type parameters from impls.
//!
//! # Rules
//! 1. appear in the trait reference of the impl, if any;
//! 2. appear in the self type of the impl; or,
//! 3. be bound as an associated type.
//!
//! # Details
//! ```non-rust
//! if T appears in the impl trait reference,
//!     then: T is contrained;
//! if T appears in the impl self type,
//!     then: T is constrained;
//! if <T0 as Trait<T1...Tn>::U == V appears in the impl predicates
//!     and T0, T1...Tn are constrained,
//!     and `T0 as Trait<T1...Tn>` is not the impl trait reference,
//!     then: V is constrained.
//! ```
//!
//! cc [E0207](https://doc.rust-lang.org/nightly/error-index.html#E0207)
//!
//! [RFC: 0047-no-unused-impl-parameters](https://github.com/rust-lang/rfcs/blob/master/text/0447-no-unused-impl-parameters.md)
