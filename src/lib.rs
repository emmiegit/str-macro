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

// Definition //

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
