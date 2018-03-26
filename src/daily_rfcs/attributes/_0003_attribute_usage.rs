//! `libsyntax/ast.rs`
//! ```
//! pub struct Attribute {
//!     pub id: AttrId,
//!     pub style: AttrStyle,
//!     pub path: Path,
//!     pub tokens: TokenStream,
//!     pub is_sugared_doc: bool,
//!     pub span: Span,
//! }
//! ```
//!
//! # Note
//! * `syntax::ast::parse::ParseSess` use `AttrId` to generate a side table of used attributes
//!    *ONLY* during parsing and expanding.
//! * The `attribute-usage` lint would run at the end of compilation and warn on all attributes
//! whose ID does not appear in the side table.
//!
//! [RFC: 0003-attribute-usage](https://github.com/rust-lang/rfcs/blob/master/text/0003-attribute-usage.md)

struct RFCs(i32);
