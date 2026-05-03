use proc_macro::{Span, TokenStream};
use quote::{ToTokens as _, quote};
use syn::{Data, DeriveInput, Ident, Type, parse::Parse, parse_macro_input, spanned::Spanned};

pub fn from_row_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let struct_fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("GetFields only work with struct type"),
    };

    let field_names: Vec<Ident> = struct_fields
        .iter()
        .map(|f| {
            let field = f.ident.clone().unwrap().to_string();

            Ident::new(&field, struct_name.span())
        })
        .collect();

    let field_names_with_types: Vec<(String, Ident, Type)> = struct_fields
        .iter()
        .map(|f| {
            let field = f.ident.clone().unwrap().to_string();

            let ident = Ident::new(&field, struct_name.span());

            let ty = f.ty.clone();

            (field, ident, ty)
        })
        .collect();

    let types: Vec<Type> = field_names_with_types.iter().map(|f| f.2.clone()).collect();

    for ty in &types {
        println!("{}", ty.to_token_stream());
    }

    let expanded = quote! {
        use rsfbclient::{Row, SqlType};

        impl From<Row> for #struct_name
        where
            Self: Default,
        {
            fn from(value: Row) -> Self {
                let mut default_value = Self::default();

                let mut key_val: HashMap<String, SqlType> = value
                    .cols
                    .iter()
                    .map(|c| (c.name.to_uppercase(), c.value.clone()))
                    .collect();

                #(default_value.#field_names = key_val.);*

                default_value
            }
        }
    };

    TokenStream::from(expanded)
}
