//! Dissolve internals
use std::{
    iter::Extend,
    convert::TryFrom,
};

use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{
    Data,
    DataStruct,
    Fields,
    DeriveInput,
    FieldsNamed,
    Type,
    Ident,
    Result,
    Error,
    TypeTuple,
    token::Paren,
    punctuated::Punctuated,
};

use crate::faultmsg::{StructIs, Problem};

pub fn extract_fields<'a>(structure: &'a DataStruct) -> Result<&'a FieldsNamed> {
    match structure.fields {
        Fields::Named(ref fields) => Ok(fields),
        Fields::Unnamed(_) | Fields::Unit => Err(
            Error::new(Span::call_site(), Problem::UnnamedField)
        ),
    }
}

pub fn extract_struct<'a>(node: &'a DeriveInput) -> Result<&'a DataStruct> {
    match node.data {
        Data::Struct(ref structure) => Ok(structure),
        Data::Enum(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Enum))
        ),
        Data::Union(_) => Err(
            Error::new_spanned(node, Problem::NotNamedStruct(StructIs::Union))
        ),
    }
}

pub struct Field {
    ty: Type,    
    name: Ident,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Self> {
        let name: Ident =  field.ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;
        
        Ok(Field {
            ty: field.ty.clone(),
            name: name,
        })
    }
    
    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named.named
            .iter()
            .map(|field| Field::from_field(field))
            .collect()
    }
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
}

impl<'a> NamedStruct<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) = self.original.generics
            .split_for_impl();        
        let struct_name = &self.name;

        let types: Punctuated<Type, syn::Token![,]> = self.fields
            .iter()
            .fold(Punctuated::new(), |mut p, field| {
                p.push(field.ty.clone());
                p
            });

        let type_tuple = TypeTuple {
            paren_token: Paren { span: Span::call_site() },
            elems: types,
        };

        let fields: TokenStream = self.fields
            .iter()
            .enumerate()
            .fold(TokenStream::new(), |mut ts, (count, field)| {
                if count > 0 {
                    ts.extend(quote!(,))
                }
                
                let field_name = &field.name;
                let field_expr = quote!(
                    self.#field_name
                );

                ts.extend(field_expr);

                ts
            });
        
        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                pub fn dissolve(self) -> #type_tuple {
                    (
                        #fields
                    )
                }
            }
        )        
    }
}

impl<'a> TryFrom<&'a DeriveInput> for NamedStruct<'a> {
    type Error = Error;
    
    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = extract_struct(node)?;
        let named_fields = extract_fields(struct_data)?;
        let fields = Field::from_fields_named(named_fields)?;

        Ok(NamedStruct {
            original: node,
            name: node.ident.clone(),
            fields,
        })
    }
}
