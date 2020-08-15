//! Getters internals

use std::{
    convert::TryFrom,
    iter::Extend,
};

use proc_macro2::{TokenTree, TokenStream, Delimiter, Span};
use quote::quote;
use syn::{
    Data,
    DataStruct,
    Fields,
    DeriveInput,
    FieldsNamed,
    Type,
    AttrStyle,
    Ident,
    LitStr,
    Result,
    Error,
    Attribute,
    parse_str,
    parse::{Parse, ParseStream, Nothing},
};

use crate::faultmsg::{StructIs, Problem};

static INVALID_STRUCT: &str = "Struct must be a named struct. Not unnamed or unit.";
static INVALID_VARIANT: &str = "Variant must be a struct. Not an enum or union.";
static VALID_ATTR: &str = "Either #[getter(skip)] or #[getter(rename=\"name\")].";

enum Action {    
    Skip,
    Rename(Ident),
}

impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(skip);
        syn::custom_keyword!(rename);
        
        if input.peek(skip) {
            let _ = input.parse::<skip>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowSkip))
            } else {
                Ok(Action::Skip)
            }
        } else if input.peek(rename) {
            let _ = input.parse::<rename>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowNewName))
            } else {
                Ok(Action::Rename(Ident::new(name.value().as_str(), Span::call_site())))
            }
        } else {
            Err(Error::new(Span::call_site(), Problem::InvalidAttribute))
        }
    }
}

fn get_action_from(attributes: &[Attribute]) -> Result<Option<Action>> {
    let mut current: Option<Action> = None;
    
    for attr in attributes {
        if attr.style != AttrStyle::Outer { continue; }
        
        if attr.path.is_ident("getter") {
            current = Some(attr.parse_args::<Action>()?);
        }
    }
    
    Ok(current)
}

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
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        match get_action_from(field.attrs.as_slice())? {
            Some(Action::Skip) => return Ok(None),
            Some(Action::Rename(ident)) => Ok(Some(Field {
                ty: field.ty.clone(),
                name: ident,
            })),
            None => Ok(Some(Field {
                ty: field.ty.clone(),
                name: field.ident
                    .clone()
                    .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?,
            })),
        }
    }
    
    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named.named
            .iter()
            .try_fold(Vec::new(), |mut fields, field| {
                if let Some(field) = Field::from_field(field)? {
                    fields.push(field);
                }

                Ok(fields)
            })
    }
}

pub struct NamedStruct<'a> {
    original: &'a DeriveInput,
    name: Ident,
    fields: Vec<Field>,
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

/*
pub fn expand(ast: &DeriveInput) -> Result<proc_macro::TokenStream> {
    let struct_name = &ast.ident;

    /*
    let (impl_generics, struct_generics, where_clause) = ast.generics.split_for_impl();
    
    let fields = isolate_named_fields(&ast)?;
    let methods = getters_from_fields(fields)?;
    */
    
    Ok(
        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #(#methods)*
            }
        ).into()
    )
}
*/
