#[macro_use]
extern crate typed_index_derive;

#[derive(Debug, Eq, PartialEq)]
struct Spam(i32);

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, TypedIndex)]
#[typed_index(Spam)]
struct SpamIdx(usize);

#[test]
fn indexing() {
    let idx = SpamIdx(0);

    let xs: &[Spam] = &[Spam(10), Spam(20)];
    let spam: &Spam = &xs[idx];
    assert_eq!(spam, &Spam(10));

    let xs: &mut [Spam] = &mut [Spam(10), Spam(20)];
    let spam: &mut Spam = &mut xs[idx];
    assert_eq!(spam, &Spam(10));

    let xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    let spam: &Spam = &xs[idx];
    assert_eq!(spam, &Spam(10));

    let mut xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    let spam: &mut Spam = &mut xs[idx];
    assert_eq!(spam, &Spam(10));
}

#[test]
fn get() {
    let idx = SpamIdx(0);

    let xs: &[Spam] = &[Spam(10), Spam(20)];
    assert_eq!(idx.get(xs), Some(&Spam(10)));

    let xs: &mut [Spam] = &mut [Spam(10), Spam(20)];
    assert_eq!(idx.get_mut(xs), Some(&mut Spam(10)));

    let xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    assert_eq!(idx.get(&xs), Some(&Spam(10)));

    let mut xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    assert_eq!(idx.get_mut(&mut xs), Some(&mut Spam(10)));

    let out_of_bounds = SpamIdx(92);
    let mut xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    assert_eq!(out_of_bounds.get(&xs), None);
    assert_eq!(out_of_bounds.get_mut(&mut xs), None);
}


#[test]
fn arithmetic() {
    let a = SpamIdx(0);
    let b = SpamIdx(10);
    let c: usize = b - a;
    assert_eq!(c, 10);
    assert_eq!(a + c, b);
    assert_eq!(a, b - c);
    let mut d = a;
    d += c;
    assert_eq!(d, b);
    d -= c;
    assert_eq!(d, a);
}

#[test]
fn conversion() {
    let a = SpamIdx(10);
    let b: usize = a.into();
    assert_eq!(b, 10usize);

    let a: usize = 10 ;
    let b: SpamIdx = a.into();
    assert_eq!(b, SpamIdx(10));
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, TypedIndex)]
#[typed_index(Spam)]
struct SmolIdx(u32);


#[test]
fn indexing_smol() {
    let idx = SmolIdx(0);

    let xs: &[Spam] = &[Spam(10), Spam(20)];
    let spam: &Spam = &xs[idx];
    assert_eq!(spam, &Spam(10));

    let xs: &mut [Spam] = &mut [Spam(10), Spam(20)];
    let spam: &mut Spam = &mut xs[idx];
    assert_eq!(spam, &Spam(10));

    let xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    let spam: &Spam = &xs[idx];
    assert_eq!(spam, &Spam(10));

    let mut xs: Vec<Spam> = vec![Spam(10), Spam(20)];
    let spam: &mut Spam = &mut xs[idx];
    assert_eq!(spam, &Spam(10));
}


#[test]
fn arithmetic_smol() {
    let a = SmolIdx(0);
    let b = SmolIdx(10);
    let c: u32 = b - a;
    assert_eq!(c, 10);
    assert_eq!(a + c, b);
    assert_eq!(a, b - c);
    let mut d = a;
    d += c;
    assert_eq!(d, b);
    d -= c;
    assert_eq!(d, a);
}
