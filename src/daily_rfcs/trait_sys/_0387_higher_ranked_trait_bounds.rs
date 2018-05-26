//! Add the ability to have trait bounds that are polymorphic over lifetimes.
//!
//! ## Note
//!
//! ### syntax
//! ```rust
//! for<lifetimes> Trait<T1, ..., Tn>
//! for<lifetimes> Trait(T1, ..., tn) -> Tr
//! ```
//!
//! - Legal: in `where` clauses and types.
//! - Illegal: `impl` and qualified path `<T as Trait>`
//!
//! Implicit binders are introduced for omitted lifetimes when using parentheses notation(`Trait(T1, ..., Tn)`) and in fn types(`for<'a> fn(&'a T)`).
//!
//! ### Early_Bound vs Late_Bound lifetime in `impl`s
//! - Early:
//!    - the `self` type of the impl;
//!    - a `where` clause associated with the impl.
//! - Late:
//!    - all other lifetimes.
//!
//! ```rust
//! // Here 'late does not appear in any where clause nor in the self type,
//! // and hence it is late-bound. Thus this impl is considered to provide:
//! //
//! //     SomeType : for<'late> FnMut<(&'late Foo,),()>
//! impl<'late> FnMut(&'late Foo) -> Bar for SomeType { ... }
//!
//! // Here 'early appears in the self type and hence it is early bound.
//! // This impl thus provides:
//! //
//! //     SomeOtherType<'early> : FnMut<(&'early Foo,),()>
//! impl<'early> FnMut(&'early Foo) -> Bar for SomeOtherType<'early> { ... }
//! ```
//!
//! ### Instantiating late-bound lifetimes in a trait reference
//! - Accessing an associated constant, once those are implemented.
//! - Accessing an associated type.
//! - Accessing an associated item from a trait reference.
//!
//! [RFC: 0387-higher-ranked-trait-bounds](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md)

struct RFCs(i32);

