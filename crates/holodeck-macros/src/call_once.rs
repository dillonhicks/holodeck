use proc_macro2::TokenStream;
use quote::{
    quote,
    quote_spanned,
};
use syn::spanned::Spanned as _;

use crate::context::Context;


pub struct CallOnce {
    input: syn::Item,
}

impl syn::parse::Parse for CallOnce {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: input.parse()?,
        })
    }
}

impl CallOnce {
    pub(super) fn expand(
        self,
        ctx: Context,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        let mut expander = Expander { ctx };

        expander
            .expand_item(&self.input)
            .ok_or_else(|| expander.ctx.errors)
    }
}

struct Expander {
    ctx: Context,
}

impl Expander {
    fn expand_item(
        &mut self,
        item: &syn::Item,
    ) -> Option<TokenStream> {
        match item {
            syn::Item::Fn(item_fn) => self.expand_function(item_fn),
            _ => {
                self.ctx
                    .errors
                    .push(syn::Error::new_spanned(item, "only function are supported"));
                None
            }
        }
    }

    fn expand_function(
        &mut self,
        input: &syn::ItemFn,
    ) -> Option<TokenStream> {
        let vis = &input.vis;
        let sig = &input.sig;
        let block = &input.block;

        let once_ident = {
            let name = format!("HOLODECK_CALL_ONCE_{}", &sig.ident.to_string().to_uppercase());
            syn::Ident::new(&name, input.span())
        };

        let call_once_impl = quote_spanned! { input.span() =>

            static #once_ident: ::std::sync::Once = ::std::sync::Once::new();

            #vis #sig {
                #once_ident.call_once(move ||
                    #block
                )
            }
        };

        Some(quote_spanned! { input.span() => #call_once_impl})
    }
}
