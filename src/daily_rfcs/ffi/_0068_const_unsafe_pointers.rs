//! Rename `*T` to `*const T`.
//!
//! ## Note
//! |Rust    | C       |
//! |--------|---------|
//! |`*mut T`| `T*`    |
//! |`*const T`|`const T*`|
//!
//! ### Valid coercion
//! | |`&'a T` ==> `*const T`|`&'a mut T` ==> `*mut T`|
//! |--|---------------------|------------------------|
//! |`rustc`|the memory will remain valid for the lifetime `'a` and the memory will be immutable up to memory stored in `Unsafe<U>`| the memory will stay valid during 'a and that the memory will not be accessed during `'a`, and will consume the `&'a mut T` during the coercion|
//! |`code`|the pointer is only dereferenced in the lifetime `'a`|the unsafe pointer is only dereferenced in the lifetime `'a`,  and that the memory is "valid again" after `'a`|
//! ### Valid cast
//! |`*mut T` ==> &'a mut T|`*const T` ==> `&'a T`|
//! |----------------------|----------------------|
//! |the memory is initialized to start out with and that nobody will access the memory during `'a` except for the converted pointer|the memory is initialized to start out with and that nobody will write to the pointer during `'a` except for memory within `Unsafe<U>`|
//!
//! [RFC: 0086-const-unsafe-pointers](https://github.com/rust-lang/rfcs/blob/master/text/0068-const-unsafe-pointers.md)
struct RFCs(i32);
