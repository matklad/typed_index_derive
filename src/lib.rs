//! # typed_index_derive
//!
//! A frequent pattern in Rust is to store objects in a vector and use integer indexes
//! as handlers to them. While using `usize` works, it could become confusing if there
//! are several flavors of indexes. To make the meaning of each index clear the newtype
//! wrappers like `FooIdx(usize)` are useful, but require a fair amount of boilerplate.
//! This crate derives the boilerplate for you:
//!
//!
//! ```rust
//! #[macro_use]
//! extern crate typed_index_derive;
//!
//! struct Spam(String);
//!
//! #[derive(
//!     // Usual derives for plain old data
//!     Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash,
//!     // this crate
//!     TypedIndex
//! )]
//! #[typed_index(Spam)] // index into `&[Spam]`
//! struct SpamIdx(usize); // could be `u32` instead of `usize`
//!
//! fn main() {
//!     let spams = vec![Spam("foo".into()), Spam("bar".into()), Spam("baz".into())];
//!
//!     // Conversions between `usize` and `SpamIdx`
//!     let idx: SpamIdx = 1.into();
//!     assert_eq!(usize::from(idx), 1);
//!
//!     // We can index `Vec<Spam>` with `SpamIdx`
//!     assert_eq!(&spams[idx].0, "bar");
//!
//!     // However, we can't index `Vec<usize>`
//!     // vec![1, 2, 3][idx]
//!     // error: slice indices are of type `usize` or ranges of `usize`
//!
//!     // Similarly to `<[Spam]>::get`, `SpamIdx::get`/`SpamIdx::get_mut`
//!     // returns `None` if it's out of bounds. Note that the receiver and
//!     // argument are flipped.
//!     let oob: SpamIdx = 92.into();
//!     assert!(oob.get(&spams).is_none());
//!
//!     // You can add/subtract `usize` from an index
//!     assert_eq!(&spams[idx - 1].0, "foo");
//!
//!     // The difference between two indices is `usize`
//!     assert_eq!(idx - idx, 0usize);
//! }
//! ```

#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;


#[proc_macro_derive(TypedIndex, attributes(typed_index))]
pub fn derive_typed_index(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;
    let ty_name = get_type_name(&input.attrs);
    let index_ty = get_index_ty(&input.data);

    let expanded = quote! {
        impl #name {
            pub fn get(self, xs: &[#ty_name]) -> Option<&#ty_name> {
                xs.get(self.0 as usize)
            }

            pub fn get_mut(self, xs: &mut [#ty_name]) -> Option<&mut #ty_name> {
                xs.get_mut(self.0 as usize)
            }
        }

        impl ::std::ops::Index<#name> for [#ty_name] {
            type Output = #ty_name;

            fn index(&self, index: #name) -> &#ty_name {
                &self[index.0 as usize]
            }
        }

        impl ::std::ops::IndexMut<#name> for [#ty_name] {
            fn index_mut(&mut self, index: #name) -> &mut #ty_name {
                &mut self[index.0 as usize]
            }
        }

        impl ::std::ops::Index<#name> for Vec<#ty_name> {
            type Output = #ty_name;

            fn index(&self, index: #name) -> &#ty_name {
                &self.as_slice()[index]
            }
        }

        impl ::std::ops::IndexMut<#name> for Vec<#ty_name> {
            fn index_mut(&mut self, index: #name) -> &mut #ty_name {
                &mut self.as_mut_slice()[index]
            }
        }

        impl From<#index_ty> for #name {
            fn from(val: #index_ty) -> #name { #name(val) }
        }

        impl From<#name> for #index_ty {
            fn from(val: #name) -> #index_ty { val.0 }
        }

        impl ::std::ops::Add<#index_ty> for #name {
            type Output = #name;

            fn add(self, rhs: #index_ty) -> #name {
                #name(self.0 + rhs)
            }
        }

        impl ::std::ops::AddAssign<#index_ty> for #name {
            fn add_assign(&mut self, rhs: #index_ty) {
                self.0 += rhs
            }
        }

        impl ::std::ops::Sub<#index_ty> for #name {
            type Output = #name;

            fn sub(self, rhs: #index_ty) -> #name {
                #name(self.0 - rhs)
            }
        }

        impl ::std::ops::SubAssign<#index_ty> for #name {
            fn sub_assign(&mut self, rhs: #index_ty) {
                self.0 -= rhs
            }
        }

        impl ::std::ops::Sub<#name> for #name {
            type Output = #index_ty;

            fn sub(self, rhs: #name) -> #index_ty {
                self.0 - rhs.0
            }
        }
    };

    expanded.into()
}

fn get_type_name(attrs: &[syn::Attribute]) -> syn::Ident {
    match attrs.iter().filter_map(get_typed_index_meta_items).next() {
        Some(ref mut metas) if metas.len() == 1 => {
            let meta = metas.pop().unwrap();
            match meta {
                syn::NestedMeta::Meta(syn::Meta::Word(word)) => return word,
                _ => (),
            }
        }
        _ => (),
    };
    panic!("`#[typed_index(YourType)]` attribute is mandatory")
}

fn get_index_ty(data: &syn::Data) -> syn::Type {
    match data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    let field = fields.unnamed.first().unwrap().into_value();
                    return field.ty.clone();
                }
                _ => (),
            }
        }
        _ => (),
    };

    panic!("typed_index works only for newtype structs")
}

// Copy-pasted from serde
fn get_typed_index_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "typed_index" {
        match attr.interpret_meta() {
            Some(syn::Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => {
                // TODO: produce an error
                None
            }
        }
    } else {
        None
    }
}

