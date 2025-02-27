use derive::quote_from_data;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod derive;

/// Implements `GodotConvert` and `ToGodot` for the given struct using a `Dictionary`
/// Converts all struct attributes to Variants
#[proc_macro_derive(ToGodotDictionary)]
pub fn derive_to_dictionary(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let identifier = derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let data = quote_from_data(&derive_input.data).unwrap();

    let expanded = quote! {
        impl #impl_generics godot::prelude::GodotConvert for #identifier #ty_generics #where_clause {
            type Via = godot::prelude::Dictionary;
        }

        impl #impl_generics godot::prelude::ToGodot for #identifier #ty_generics #where_clause {
            type ToVia<'v> = godot::prelude::Dictionary;

            fn to_godot(&self) -> Self::Via {
                #data
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
