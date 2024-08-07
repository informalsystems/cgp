use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse_quote, Ident};

use crate::delegate_components::ast::DefineComponentsAst;
use crate::delegate_components::define_struct::define_struct;
use crate::delegate_components::delegates_to::define_delegates_to_trait;
use crate::delegate_components::impl_delegate::impl_delegate_components;
use crate::delegate_components::substitution_macro::define_substitution_macro;
use crate::derive_component::snake_case::to_snake_case_str;

pub fn define_components(body: TokenStream) -> TokenStream {
    let ast: DefineComponentsAst = syn::parse2(body).unwrap();

    let components_type = {
        let components_ident = &ast.components_ident;
        let type_generics = ast.components_generics.split_for_impl().1;
        parse_quote!( #components_ident #type_generics )
    };

    let impl_items = impl_delegate_components(
        &components_type,
        &ast.components_generics,
        &ast.delegate_entries,
    );

    let item_struct = define_struct(&ast.components_ident, &ast.components_generics);

    let mut output = TokenStream::new();

    output.extend(item_struct.to_token_stream());

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    {
        let delegates_to_trait_name = format!("DelegatesTo{}", ast.components_ident);

        let (delegates_to_trait, delegates_to_impl) = define_delegates_to_trait(
            &Ident::new(&delegates_to_trait_name, Span::call_site()),
            &components_type,
            &ast.components_generics,
            &ast.delegate_entries,
        );

        output.extend(delegates_to_trait.to_token_stream());
        output.extend(delegates_to_impl.to_token_stream());
    }

    {
        let with_components_macro_name = format!(
            "with_{}",
            to_snake_case_str(&ast.components_ident.to_string())
        );

        let with_components_macro = define_substitution_macro(
            &Ident::new(&with_components_macro_name, Span::call_site()),
            &ast.components_ident,
            &ast.delegate_entries.all_components().to_token_stream(),
        );

        output.extend(with_components_macro);
    }

    output
}
