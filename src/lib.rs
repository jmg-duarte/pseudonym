//! # pseudonym
//! Declare aliases for Rust constructs with ease!
//! Currently, `pseudonym::alias` supports the following items:
//! * Functions
//! * Structures
//!
//! ## Usage
//! The following example will generate a function
//! named `short_name` equal to `very_long_function_name`.
//! ```rust
//! # use pseudonym::alias;
//! #[alias(short_name)]
//! fn very_long_function_name() {
//!     // ...
//! }
//!
//! fn main () {
//!     short_name(); // use the alias!
//! }
//! ```

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Item, Token, ItemFn, ItemStruct, ItemImpl, Type, spanned::Spanned, TypePath,
};

struct Aliases {
    names: Punctuated<Ident, Token![,]>,
}

impl Parse for Aliases {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            names: input.call(Punctuated::parse_separated_nonempty)?,
        })
    }
}

impl IntoIterator for Aliases {
    type Item = Ident;

    type IntoIter = syn::punctuated::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

/// Declare an aliased for the following item.
///
/// Example:
/// ```rust
/// # use pseudonym::alias;
/// #[alias(short_name)]
/// fn very_long_function_name() {
///     // ...
/// }
///
/// fn main () {
///     short_name(); // use the alias!
/// }
/// ```
#[proc_macro_attribute]
pub fn alias(args: TokenStream, input: TokenStream) -> TokenStream {
    let aliases = parse_macro_input!(args as Aliases);
    let parsed_input = parse_macro_input!(input as Item);
    return match parsed_input {
        Item::Fn(item_fn) => expand_fn(item_fn, aliases),
        Item::Struct(item_struct) => expand_struct(item_struct, aliases),
        Item::Impl(item_impl) => expand_impl(item_impl, aliases),
        _ => syn::Error::new(parsed_input.span(), "unsupported item")
            .to_compile_error()
            .into(),
    };
}

/// Expand [`syn::ItemFn`] aliases.
fn expand_fn(item_fn: ItemFn, aliases: Aliases) -> TokenStream {
    let item_fn_aliases = aliases.into_iter().map(|alias| {
        let mut item_fn_alias = item_fn.clone();
        item_fn_alias.sig.ident = alias;
        item_fn_alias
    });

    quote::quote!(
        #item_fn
        #(#item_fn_aliases)*
    )
    .into()
}

/// Expand [`syn::ItemStruct`] aliases.
fn expand_struct(item_struct: ItemStruct, aliases: Aliases) -> TokenStream {
    let item_struct_aliases = aliases.into_iter().map(|alias| {
        let mut item_struct_alias = item_struct.clone();
        item_struct_alias.ident = alias;
        item_struct_alias
    });

    quote::quote!(
        #item_struct
        #(#item_struct_aliases)*
    )
    .into()
}

/// Expand [`syn::ItemImpl`] aliases.
fn expand_impl(item_impl: ItemImpl, aliases: Aliases) -> TokenStream {
    let item_impl_aliases = aliases.into_iter().map(|ident| {
        let mut item_impl_alias = item_impl.clone();
        if let Type::Path(TypePath {ref mut path, ..}) = item_impl_alias.self_ty.as_mut() {
            let mut first_path_segment = path.segments.first_mut().unwrap();
            first_path_segment.ident = ident;
        }
        item_impl_alias
    });

    quote::quote!(
        #item_impl
        #(#item_impl_aliases)*
    )
    .into()
}