use proc_macro2::{
    Span,
    TokenStream,
};
use quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    spanned::Spanned as _,
    Meta::*,
    MetaNameValue,
    NestedMeta::*,
};

use crate::internals::*;

#[derive(Default)]
pub(crate) struct Attrs {
    /// `#[holodeck(call_once)]`
    pub(crate) call_once: Option<Span>,
}


pub(crate) struct Context {
    pub(crate) errors:     Vec<syn::Error>,
    pub(crate) attributes: Attrs,
}


impl Context {
    /// Construct a new context.
    pub(crate) fn new() -> Self {
        Self::with_module(&quote!(crate))
    }

    /// Construct a new context.
    pub(crate) fn with_module<M>(module: M) -> Self
    where
        M: Copy + ToTokens,
    {
        Self {
            errors:     Vec::new(),
            attributes: Default::default(),
        }
    }

    /// Parse the `meta` of a `#[holodeck(<meta>)]` attribute
    pub(crate) fn parse_meta(
        &mut self,
        meta: &syn::Meta,
    ) -> Option<Attrs> {
        let mut attrs = Attrs::default();

        match &meta {
            // Parse `#[holodeck(call_once)]`
            Path(word) if *word == CALL_ONCE => {
                attrs.call_once = Some(meta.span());
            }
            meta => {
                self.errors
                    .push(syn::Error::new_spanned(meta, "unsupported attribute"));

                return None;
            }
        }


        Some(attrs)
    }
}
