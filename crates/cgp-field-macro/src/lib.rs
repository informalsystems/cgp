extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HasField)]
pub fn derive_fields(item: TokenStream) -> TokenStream {
    cgp_field_macro_lib::derive_fields(item.into()).into()
}

#[proc_macro]
pub fn symbol(body: TokenStream) -> TokenStream {
    cgp_field_macro_lib::make_symbol(body.into()).into()
}
