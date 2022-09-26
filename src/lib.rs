//! # pseudonym
//! Declare aliases for Rust constructs with ease!
//! Currently, `pseudonym::alias` supports the following items:
//! * Functions
//! * Structures
//! * Traits
//! * Implementations
//!
//! ## Usage
//! In the following examples the items to which the macro is attached to
//! get generated as aliases using the passed identifier.
//!
//! ### Functions
//!
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
//!
//! ### Structures
//!
//! ```rust
//! # use pseudonym::alias;
//! #[alias(StructAlias)]
//! struct S (i32);
//! ```
//!
//! ### Traits
//!
//! ```rust
//! # use pseudonym::alias;
//! #[alias(TraitAlias)]
//! trait T {}
//! ```
//!
//! ### Implementations
//!
//! ```rust
//! # use pseudonym::alias;
//! #[alias(StructAlias)]
//! struct S;
//!
//! #[alias(StructAlias)]
//! impl S {
//!     fn new() -> Self { S }
//! }
//! ```
use proc_macro::TokenStream;
use std::fmt::Debug;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    Ident, Item, ItemFn, ItemImpl, ItemStruct, ItemTrait, LitStr, Token, Type, TypePath, ItemConst,
};

/// [`syn::Ident`] extension functions.
trait IdentExt {
    /// Return the ident representing `Self`.
    fn get_ident(&self) -> Ident;
}

/// Structure representing the `deprecated` attribute.
#[derive(Debug)]
struct Deprecated {
    name: Ident,
    since: Option<String>,
    note: Option<String>,
}

impl Parse for Deprecated {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let deprecated = input.parse::<Ident>()?;
        if deprecated != "deprecated" {
            return Err(syn::Error::new(
                deprecated.span(),
                "invalid argument, only `deprecated` is supported",
            ));
        }
        let content;
        let _ = parenthesized!(content in input);

        let name = content.parse::<Ident>()?;
        let mut since = None;
        let mut note = None;

        // parse a possible trailing comma
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }

        // If/While the stream is not empty, try to parse the remaining key values
        // allows overwriting values
        while !content.is_empty() {
            // Parse key value pairs e.g.
            // note = "deprecation note"
            if content.peek(Ident) && content.peek2(Token![=]) && content.peek3(LitStr) {
                let parsed_ident = content.parse::<Ident>()?;
                content.parse::<Token![=]>()?;
                let parsed_lit_str = content.parse::<LitStr>()?;
                // Only `since` and `version` are supported by deprecated
                if parsed_ident == "since" {
                    since = Some(parsed_lit_str.value());
                } else if parsed_ident == "note" {
                    note = Some(parsed_lit_str.value());
                } else {
                    return Err(syn::Error::new(
                        parsed_ident.span(),
                        "invalid argument, only `since` and `note` are supported",
                    ));
                }
                // if a trailing comma exists consume it
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            } else {
                return Err(syn::Error::new(content.span(), "expected a key-value pair"));
            }
        }

        Ok(Self {
            name,
            since,
            note,
        })
    }
}

#[derive(Debug)]
enum Alias {
    Deprecated(Deprecated),
    Name(Ident),
}

impl Parse for Alias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // expecting `deprecated(...)`
        let res = if input.peek(Ident) && input.peek2(Paren) {
            Self::Deprecated(input.parse::<Deprecated>()?)
        } else {
            Self::Name(input.parse::<Ident>()?)
        };
        Ok(res)
    }
}

impl IdentExt for Alias {
    fn get_ident(&self) -> Ident {
        match self {
            Alias::Deprecated(Deprecated { name, .. }) => name.clone(),
            Alias::Name(name) => name.clone(),
        }
    }
}

#[derive(Debug)]
struct Aliases(Punctuated<Alias, Token![,]>);

impl Parse for Aliases {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(Punctuated::parse_separated_nonempty)?))
    }
}

impl IntoIterator for Aliases {
    type Item = Alias;

    type IntoIter = syn::punctuated::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
        Item::Trait(item_trait) => expand_trait(item_trait, aliases),
        Item::Const(item_const) => expand_const(item_const, aliases),
        _ => syn::Error::new(parsed_input.span(), "unsupported item")
            .to_compile_error()
            .into(),
    };
}

#[doc(hidden)]
macro_rules! match_deprecated {
    ($item_ident:ident, $matched:ident) => {
        if let Alias::Deprecated(Deprecated { since, note, .. }) = $matched {
            match (since, note) {
                (None, None) => parse_quote!(
                    #[deprecated]
                    #$item_ident
                ),
                (None, Some(note)) => parse_quote!(
                    #[deprecated(note = #note)]
                    #$item_ident
                ),
                (Some(since), None) => parse_quote!(
                    #[deprecated(since = #since)]
                    #$item_ident
                ),
                (Some(since), Some(note)) => parse_quote!(
                    #[deprecated(since = #since, note = #note)]
                    #$item_ident
                ),
            }
        } else {
            $item_ident
        }
    };
}

/// Expand [`syn::ItemFn`] aliases.
fn expand_fn(item_fn: ItemFn, aliases: Aliases) -> TokenStream {
    let item_fn_aliases = aliases.into_iter().map(|alias| {
        let mut item_fn_alias = item_fn.clone();
        item_fn_alias.sig.ident = alias.get_ident();
        match_deprecated!(item_fn_alias, alias)
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
        item_struct_alias.ident = alias.get_ident();
        match_deprecated!(item_struct_alias, alias)
    });

    quote::quote!(
        #item_struct
        #(#item_struct_aliases)*
    )
    .into()
}

/// Expand [`syn::ItemImpl`] aliases.
fn expand_impl(item_impl: ItemImpl, aliases: Aliases) -> TokenStream {
    let item_impl_aliases = aliases.into_iter().map(|alias| {
        let mut item_impl_alias = item_impl.clone();
        if let Type::Path(TypePath { ref mut path, .. }) = item_impl_alias.self_ty.as_mut() {
            let mut first_path_segment = path.segments.first_mut().unwrap();
            first_path_segment.ident = alias.get_ident();
        }
        match_deprecated!(item_impl_alias, alias)
    });

    quote::quote!(
        #item_impl
        #(#item_impl_aliases)*
    )
    .into()
}

/// Expand [`syn::ItemTrait`] aliases.
fn expand_trait(item_trait: ItemTrait, aliases: Aliases) -> TokenStream {
    let item_trait_aliases = aliases.into_iter().map(|alias| {
        let mut item_trait_alias = item_trait.clone();
        item_trait_alias.ident = alias.get_ident();
        match_deprecated!(item_trait_alias, alias)
    });

    quote::quote!(
        #item_trait
        #(#item_trait_aliases)*
    )
    .into()
}

/// Expand [`syn::ItemConst`] aliases.
fn expand_const(item_const: ItemConst, aliases: Aliases) -> TokenStream {
    let item_const_aliases = aliases.into_iter().map(|alias| {
        let mut item_const_alias = item_const.clone();
        item_const_alias.ident = alias.get_ident();
        match_deprecated!(item_const_alias, alias)

    });
    quote::quote!(
        #item_const
        #(#item_const_aliases)*
    ).into()
}
