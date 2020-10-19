use proc_macro2::TokenStream;
use syn::parse::{
    Parse,
    Parser,
};

use crate::context::{
    Attrs,
    Context,
};

pub(crate) struct HolodeckAttribute {
    input: syn::Meta,
}

impl syn::parse::Parse for HolodeckAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            input: input.parse()?,
        })
    }
}



impl HolodeckAttribute {
    pub(crate) fn expand(
        mut self,
        input: proc_macro::TokenStream,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        let HolodeckAttribute { input: attr } = self;

        let mut ctx = Context::new();
        let attrs = ctx.parse_meta(&attr).unwrap_or_default();

        // #[holodeck(call_once)]
        let stream = if let Some(_) = attrs.call_once.as_ref() {
            expand_call_once(ctx, input)?
        } else {
            ctx.errors
                .push(syn::Error::new_spanned(attr, "unsupported attribute {}"));
            return Err(ctx.errors);
        };

        Ok(stream)
    }
}


fn expand_call_once(
    mut ctx: Context,
    input: proc_macro::TokenStream,
) -> Result<TokenStream, Vec<syn::Error>> {
    use crate::call_once::CallOnce;
    let expander = match CallOnce::parse.parse(input) {
        Ok(call_once) => call_once,
        Err(err) => {
            ctx.errors.push(err);
            return Err(ctx.errors);
        }
    };

    expander.expand(ctx)
}
