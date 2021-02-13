## str-macro

<p>
  <a href="https://github.com/ammongit/str-macro/actions?query=workflow%3A%22Rust+CI%22">
    <img src="https://github.com/ammongit/str-macro/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>
</p>

Rust crate for the `str!()` macro, which makes the conveniences available from `vec![]` available for `String` as well.

Has no dependencies, and should work on any Rust release channel.

* Create an empty `String`

```rust
// Vec equivalent
let v = vec![];
assert_eq!(v.len(), 0);

// String
let s = str!();
assert_eq!(s, String::new());
```

* Create an owned `String` from a constant str reference.

```rust
// Vec equivalent
let v = vec!["alpha", "beta", "gamma"];
assert_eq!(v.len(), 3);

// String
let s = str!("alpha beta gamma");
assert_eq!(&s, "alpha beta gamma");
let _: String = s;
```

* Create an owned `String` from an object which implements `ToString`.

Note that this is automatically implemented for anything that implements `Display`.

```rust
let s = str!(2194);
assert_eq!(&s, "2194");

let s = str!(Ipv4Addr::new(127, 0, 0, 1));
assert_eq!(&s, "127.0.0.1");
```

----

Copyright (C) 2019-2021 Ammon Smith

Available under the MIT License.
