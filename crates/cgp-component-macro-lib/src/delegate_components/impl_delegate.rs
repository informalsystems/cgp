use syn::{parse_quote, Generics, ImplItem, ImplItemType, ItemImpl, Path, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};
use crate::delegate_components::merge_generics::merge_generics;

pub fn impl_delegate_components(
    target_type: &Type,
    target_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> Vec<ItemImpl> {
    delegate_entries
        .entries
        .iter()
        .flat_map(|entry| {
            let source = &entry.source;

            entry.components.iter().map(|component| {
                impl_delegate_component(target_type, target_generics, component, source)
            })
        })
        .collect()
}

pub fn impl_delegate_component(
    target_type: &Type,
    target_generics: &Generics,
    component: &ComponentAst,
    source: &Type,
) -> ItemImpl {
    let component_type = &component.component_type;

    let trait_path: Path = parse_quote!(DelegateComponent < #component_type >);

    let delegate_type: ImplItemType = parse_quote!(type Delegate = #source;);

    let generics = merge_generics(target_generics, &component.component_generics);

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics,
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    }
}
