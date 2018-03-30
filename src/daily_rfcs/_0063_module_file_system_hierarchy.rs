//! `mod foo;` can *only* be placed in a crate root and in mod.rs files.
//!
//! # Note
//!
//! Valid `mod foo;`:
//! * The file under parsing is the crate root, or
//!
//! * The file under parsing is a `mod.rs`, or
//!
//! * `#[path]` is specified, e.g. `#[path = "foo.rs"] mod foo;`.
//!
//! [RFC: 0063-module-file-system-hierarchy](https://github.com/rust-lang/rfcs/blob/master/text/0063-module-file-system-hierarchy.md)

