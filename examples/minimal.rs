#[macro_use]
extern crate typed_index_derive;

struct Spam(String);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, TypedIndex)]
#[typed_index(Spam)]
struct SpamIdx(usize); // could be `u32` to save memory

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
