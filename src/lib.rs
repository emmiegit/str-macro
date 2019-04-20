/*
 * lib.rs
 *
 * str-macro - A convenience macro for strings in Rust.
 * Copyright (c) 2019 Ammon Smith
 *
 * str-macro is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations)]

//! Creates a [`String`] with the given contents.
//!
//! `str!` allows you to conveniently create owned [`String`]s.
//!
//! - Create an empty [`String`]
//! ```
//! # #[macro_use] extern crate str_macro;
//! let s = str!();
//! assert_eq!(s, String::new());
//! ```
//!
//! - Create an owned [`String`] from a constant [`str`] reference.
//! ```
//! # #[macro_use] extern crate str_macro;
//! const CONST_STR: &str = "alpha beta gamma";
//!
//! let s = str!(CONST_STR);
//! assert_eq!(&s, CONST_STR);
//! ```
//!
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html

// Definition //

/// Create an owned `String`.
///
/// See the [crate-level documentation] for usage examples.
///
/// [crate-level documentation]: index.html
#[macro_export]
macro_rules! str {
    () => ( String::new() );
    ($x:expr) => ( ToString::to_string(&$x) );
}

// Tests //

#[cfg(test)]
mod test {
    #[test]
    fn test_simple() {
        let s: String = str!();
        assert_eq!(s, "");

        let s: String = str!("test message");
        assert_eq!(s, "test message");

        let s: String = str!(String::from("second message"));
        assert_eq!(s, "second message");
    }

    #[derive(Debug)]
    struct S;

    impl ToString for S {
        fn to_string(&self) -> String {
            String::from("from struct")
        }
    }

    #[test]
    fn test_impl() {
        let s: String = str!(S);
        assert_eq!(s, "from struct");
    }
}
