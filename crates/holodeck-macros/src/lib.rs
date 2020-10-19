extern crate proc_macro;


mod call_once;
mod context;
mod holodeck;
mod internals;



/// entrypoint for non-derive `#[holodeck(...)]` macros
#[proc_macro_attribute]
#[doc(hidden)]
pub fn holodeck(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = syn::parse_macro_input! { attr as holodeck::HolodeckAttribute };

    attr.expand(input).unwrap_or_else(to_compile_errors).into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    ::quote::quote!(#(#compile_errors)*)
}
