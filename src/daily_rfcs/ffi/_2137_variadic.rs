//! Support defining C-compatible variadic functions in Rust.
//!
//! # Note
//!
//! *Variadic fucntion declaration:*
//! ```rust
//! pub unsafe extern "C" fn foo(arg: T, arg2: T2, args: ...);
//! ```
//! 1. The use of `...` as the type of `args` at the end of the
//! argument list declares the function `foo` as variadic function.
//! 2. `args: ...` must apears as the last argument of the function, and
//! there must be at least one argument before it.
//! 3. The function must be `extern "C"` and `unsafe`.
//! 4. The real type of `args` is `core::intrinsics::VaList<'a>`, the lifetme `'a` prevents the arguments from outliving the variadic function.
//!
//! [RFC: 2137-variadic](https://github.com/rust-lang/rfcs/blob/master/text/2137-variadic.md)

struct RFCs(i32);
