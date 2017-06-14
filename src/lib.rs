#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![deny(fat_ptr_transmutes,
        missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        variant_size_differences)]

//! Contains a lexer for lexing the WebIDL grammar.

/// Contains lexer related structures and functions for lexing the WebIDL grammar.
pub mod lexer;

pub use lexer::Lexer;
