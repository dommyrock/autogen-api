use proc_macro::TokenStream;

//`proc-macro` crate types currently cannot export any items other than functions tagged with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
mod controllers;

//macro registry example
//https://github.com/cloudflare/foundations/tree/main/foundations-macros/src

#[proc_macro_attribute]
pub fn generate_controller(_args: TokenStream, item: TokenStream) -> TokenStream {
    controllers::expand(item)
}


