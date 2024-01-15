extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn rustexify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;

    let latex_code = format!("\\text{{Function: }} {}", fn_name);

    let output = quote! {
        #input_fn
    }

    TokenStream::from(output)
}
