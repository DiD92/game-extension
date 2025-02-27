use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{spanned::Spanned, Data, DataStruct};

pub(crate) fn quote_from_data(data: &Data) -> Result<TokenStream2, syn::Error> {
    match data {
        Data::Struct(data) => Ok(struct_to_dict(data)),
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span(),
            "expected `struct` found `enum`",
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            "expected `struct` found `union`",
        )),
    }
}

fn struct_to_dict(data_struct: &DataStruct) -> TokenStream2 {
    let fields = data_struct
        .fields
        .iter()
        .enumerate()
        .map(|(field_idx, field)| {
            if let Some(ident) = &field.ident {
                let key = ident.to_string().trim_start_matches("r#").to_owned();

                quote! {
                    #key: self.#ident.to_variant()
                }
            } else {
                let key = format!("{}", field_idx);

                quote! {
                    #key: self.#field_idx.to_variant()
                }
            }
        });

    quote! {
        godot::prelude::dict! {
            #(#fields),*
        }
    }
}
