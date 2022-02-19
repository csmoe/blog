/// # `derive(Copy)` vs `impl Copy`
///
/// As the [std] said:
/// > If a type is Copy then its Clone implementation only needs to return *self.
///
/// that is, `clone()` will be optimized to `memcpy` as `Copy` does.
///
/// **BUT**, not always: If a local type is derived with `#[derive(Clone)]`,
/// and without **derived** `Copy`. The compiler cannot optimize the `clone()` to `memcpy`
/// anymore.
///
/// # Example[2]
/// Compile this snippet with `rustc example.rs --crate-type lib -Clink-dead-code -Copt-level=3 --emit=asm`
/// ```rust,no_run
/// fn foo() {
///     let a = A { i: 1u8 };
///     a.clone();
/// }
///
/// #[derive(Clone)]
/// struct A { i: u8 }
///
/// impl Copy for A {}
/// ```
/// ```asm
/// core::clone::Clone::clone_from:
///     mov     al, byte ptr [rsi]
///     mov     byte ptr [rdi], al
///     ret

/// core::clone::impls::<impl core::clone::Clone for u8>::clone:
///     mov     al, byte ptr [rdi]
///     ret

/// example::foo:
///     ret

/// <example::A as core::clone::Clone>::clone:
///     mov     al, byte ptr [rdi]
///     ret
/// ```
/// The `clone()` method invokes the `clone()` implementation of inner field, instead of just
/// `memcpy`ing on `struct S` as derived Copy does:
///
/// Compares to,
/// ```git
/// - #[derive(Clone)]
/// + #[derive(Clone, Copy)]
/// struct S { i: u8 }
/// - impl Copy for S {}
/// ```
/// ```asm
/// core::clone::Clone::clone_from:
///     mov     al, byte ptr [rsi]
///     mov     byte ptr [rdi], al
///     ret

/// example::foo:
///     ret

/// <example::A as core::clone::Clone>::clone:
///     mov     al, byte ptr [rdi]
///     ret
/// ```
/// # Why?
/// Derived `Clone` is kind of special as its implemented during macro expansion phase, the rustc
/// do not know the `impl Copy`ed type is `Copy` at that time[1].
///
/// [std]: https://doc.rust-lang.org/nightly/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone
/// [1]: https://github.com/rust-lang/rust/issues/47796
/// [2]: https://rust.godbolt.org/z/jao4WsPGd
pub type DeriveImplCopy = ();
