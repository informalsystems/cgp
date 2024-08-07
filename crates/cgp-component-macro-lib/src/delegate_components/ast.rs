use core::iter;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Colon, Comma, Lt};
use syn::{braced, bracketed, Generics, Ident, Token, Type};

pub struct DelegateComponentsAst {
    pub target_type: Type,
    pub target_generics: Generics,
    pub delegate_entries: DelegateEntriesAst,
}

pub struct DefineComponentsAst {
    pub components_ident: Ident,
    pub components_generics: Generics,
    pub delegate_entries: DelegateEntriesAst,
}

pub struct DelegateEntriesAst {
    pub entries: Punctuated<DelegateEntryAst, Comma>,
}

pub struct DelegateEntryAst {
    pub components: Punctuated<ComponentAst, Comma>,
    pub source: Type,
}

#[derive(Clone)]
pub struct ComponentAst {
    pub component_type: Type,
    pub component_generics: Generics,
}

impl DelegateEntriesAst {
    pub fn all_components(&self) -> Punctuated<ComponentAst, Comma> {
        self.entries
            .iter()
            .flat_map(|entry| entry.components.clone().into_iter())
            .collect()
    }
}

impl Parse for DelegateComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let target_type: Type = input.parse()?;

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            target_type,
            target_generics,
            delegate_entries,
        })
    }
}

impl Parse for DefineComponentsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components_ident: Ident = input.parse()?;

        let components_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let delegate_entries: DelegateEntriesAst = input.parse()?;

        Ok(Self {
            components_ident,
            components_generics,
            delegate_entries,
        })
    }
}

impl Parse for DelegateEntriesAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entries = {
            let entries_body;
            braced!(entries_body in input);
            entries_body.parse_terminated(DelegateEntryAst::parse, Comma)?
        };

        Ok(Self { entries })
    }
}

impl Parse for DelegateEntryAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = if input.peek(Bracket) {
            let components_body;
            bracketed!(components_body in input);
            components_body.parse_terminated(ComponentAst::parse, Token![,])?
        } else {
            let component: ComponentAst = input.parse()?;
            Punctuated::from_iter(iter::once(component))
        };

        let _: Colon = input.parse()?;

        let source: Type = input.parse()?;

        Ok(Self { components, source })
    }
}

impl Parse for ComponentAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let component_type: Type = input.parse()?;

        Ok(Self {
            component_type,
            component_generics,
        })
    }
}

impl ToTokens for ComponentAst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.component_generics.to_token_stream());
        tokens.extend(self.component_type.to_token_stream());
    }
}
