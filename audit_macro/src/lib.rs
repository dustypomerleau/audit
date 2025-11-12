use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

// TODO: you probably want an attribute macro, since it's not possible to make helper attributes
// mandatory, and you always want to supply the range at a minimum.
#[proc_macro_attribute]
pub fn range_bounded(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let output = quote! {};

    output.into()
}