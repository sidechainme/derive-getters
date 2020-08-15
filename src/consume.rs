//! Consume internals.

use proc_macro2::{TokenStream};
use quote::quote;
use syn::FieldsNamed;

pub fn fields_tuple(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let field_names = fields.named
        .iter()
        .map(|field| {
            field.ident
                .as_ref()
                .expect("Fields must be named.")
        })
        .collect();

    quote!(
            #(field_names)*
    )
}

pub fn field_types_tuple(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let field_types = fields.named
        .iter()
        .map(|field| &field.ty)
        .collect();

    quote!(
            #(field_types)*
    )
}
