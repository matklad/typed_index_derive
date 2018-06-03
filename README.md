# typed_index_derive

[![Build Status](https://travis-ci.org/matklad/typed_index_derive.svg?branch=master)](https://travis-ci.org/matklad/typed_index_derive)
[![Crates.io](https://img.shields.io/crates/v/typed_index_derive.svg)](https://crates.io/crates/typed_index_derive)
[![API reference](https://docs.rs/typed_index_derive/badge.svg)](https://docs.rs/typed_index_derive/)

Custom derive to easily create newtype index types.

A frequent pattern in Rust is to store objects in a vector and use integer indexes
as handlers to them. While using `usize` works, it could become confusing if there
are several flavors of indexes. To make the meaning of each index clear the newtype
wrappers like `FooIdx(usize)` are useful, but require a fair amount of boilerplate.
This crate derives the boilerplate for you:


```rust
#[macro_use]
extern crate typed_index_derive;

struct Spam(String);

#[derive(
    // Usual derives for plain old data
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash,
    // this crate
    TypedIndex
)]
#[typed_index(Spam)] // index into `&[Spam]`
struct SpamIdx(usize); // could be `u32` instead of `usize`

fn main() {
    let spams = vec![Spam("foo".into()), Spam("bar".into()), Spam("baz".into())];

    // Conversions between `usize` and `SpamIdx`
    let idx: SpamIdx = 1.into();
    assert_eq!(usize::from(idx), 1);

    // We can index `Vec<Spam>` with SpamIdx
    assert_eq!(&spams[idx].0, "bar");

    // However, we can't index `Vec<usize>`
    // vec![1, 2, 3][idx]
    // error: slice indices are of type `usize` or ranges of `usize`

    // You can add/subtract `usize` from an index
    assert_eq!(&spams[idx - 1].0, "foo");

    // The difference between two indices is `usize`
    assert_eq!(idx - idx, 0usize);
}
```
