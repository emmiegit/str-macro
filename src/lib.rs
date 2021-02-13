/*
 * lib.rs
 *
 * str-macro - A convenience macro for strings in Rust.
 * Copyright (c) 2019-2021 Ammon Smith
 *
 * str-macro is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]

//! Creates a [`String`] with the given contents.
//!
//! `str!` allows you to conveniently create owned [`String`]s,
//! similar to the [`vec!`] macro found in the Rust stdlib.
//!
//! - Create an empty [`String`]
//! ```
//! # #[macro_use] extern crate str_macro;
//! // Vec equivalent
//! let v = vec![];
//! assert_eq!(v, Vec::new());
//! assert!(v.is_empty());
//!
//! # let _: Vec<()> = v;
//! // String
//! let s = str!();
//! assert_eq!(s, String::new());
//! assert!(s.is_empty());
//! ```
//!
//! - Create an owned [`String`] from a constant [`str`] reference.
//! ```
//! # #[macro_use] extern crate str_macro;
//! // Vec equivalent
//! let v = vec!["alpha", "beta", "gamma"];
//! assert_eq!(&v, &["alpha", "beta", "gamma"]);
//! assert_eq!(v.len(), 3);
//!
//! // String
//! let s = str!("alpha beta gamma");
//! assert_eq!(&s, "alpha beta gamma");
//! let _: String = s;
//! ```
//!
//! - Create an owned [`String`] from an object which implements [`ToString`].
//!
//! Note that this is automatically implemented for anything that implements [`Display`].
//! ```
//! # #[macro_use] extern crate str_macro;
//! # use std::net::Ipv4Addr;
//! let s = str!(2194);
//! assert_eq!(&s, "2194");
//!
//! let s = str!(Ipv4Addr::new(127, 0, 0, 1));
//! assert_eq!(&s, "127.0.0.1");
//! ```
//!
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`vec!`]: https://doc.rust-lang.org/std/macro.vec.html

// Definition //

/// Create an owned `String`.
///
/// See the [crate-level documentation] for usage examples.
///
/// [crate-level documentation]: index.html
#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr) => {
        ToString::to_string(&$x)
    };
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
