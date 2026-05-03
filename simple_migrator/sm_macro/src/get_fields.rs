use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

pub fn get_fields_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let struct_fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("GetFields only work with struct type"),
    };

    let uppercase_field_name: Vec<String> = struct_fields
        .iter()
        .map(|f| {
            f.ident
                .clone()
                .unwrap()
                .to_string()
                .replace('_', "")
                .to_uppercase()
        })
        .collect();

    let expanded = quote! {
        impl #struct_name {
            pub fn get_fields() -> Vec<String>{
                let fields = Vec::<String>::from([
                    #(#uppercase_field_name.into()),*
                ]);

                fields
            }
        }
    };

    TokenStream::from(expanded)
}
