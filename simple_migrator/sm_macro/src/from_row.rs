use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

pub fn from_row_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let struct_fields = match input.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => panic!("GetFields only work with struct type"),
    };

    let field_names_with_types: Vec<(String, String, Ident)> = struct_fields
        .iter()
        .map(|f| {
            let field = f.ident.clone().unwrap().to_string();
            let uppercase_field = field.replace('_', "").to_uppercase();

            let ident = Ident::new(&field, struct_name.span());

            (field, uppercase_field, ident)
        })
        .collect();

    let uppercase: Vec<_> = field_names_with_types.iter().map(|t| &t.1).collect();
    let idents: Vec<_> = field_names_with_types.iter().map(|t| &t.2).collect();

    let expanded = quote! {
        use rsfbclient::{Row, SqlType};
        use chrono::NaiveDateTime;
        use std::collections::HashMap;

        pub trait IntoValueTrait<T> {
            fn get_and_into(&self, key: String) -> T;
        }

        impl IntoValueTrait<bool> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> bool {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Boolean(bool_val) => *bool_val,
                    _ => panic!("key is not type of String"),
                }
            }
        }

        impl IntoValueTrait<String> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> String {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Text(text) => text.clone(),
                    _ => panic!("key is not type of String"),
                }
            }
        }

        impl IntoValueTrait<Option<String>> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> Option<String> {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Text(text) => Some(text.clone()),
                    SqlType::Null => None,
                    _ => panic!("key is not type of Option<String>"),
                }
            }
        }

        impl IntoValueTrait<Option<i64>> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> Option<i64> {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Integer(int_val) => Some(*int_val),
                    SqlType::Null => None,
                    _ => panic!("key is not type of Option<i64>"),
                }
            }
        }

        impl IntoValueTrait<i64> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> i64 {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Integer(int_val) => *int_val,
                    _ => panic!("key is not type of i64"),
                }
            }
        }

        impl IntoValueTrait<NaiveDateTime> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> NaiveDateTime {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Timestamp(timestamp) => *timestamp,
                    _ => panic!("key is not type of Option<i64>"),
                }
            }
        }

        impl IntoValueTrait<Option<NaiveDateTime>> for HashMap<String, SqlType> {
            fn get_and_into(&self, key: String) -> Option<NaiveDateTime> {
                let sql_val = self.get(&key).expect(&format!("fail to get value from key: {}", key));

                match sql_val {
                    SqlType::Timestamp(timestamp) => Some(*timestamp),
                    SqlType::Null => None,
                    _ => panic!("key is not type of Option<NaiveDateTime>"),
                }
            }
        }

        impl From<&Row> for #struct_name
        where
            Self: Default,
        {
            fn from(value: &Row) -> Self {

                let mut key_val: HashMap<String, SqlType> = value
                    .cols
                    .iter()
                    .map(|c| (c.name.to_uppercase(), c.value.clone()))
                    .collect();

                Self{
                    #(#idents : key_val.get_and_into(#uppercase .into())),*
                }

            }
        }

        impl From<Row> for #struct_name
        where
            Self: Default,
        {
            fn from(value: Row) -> Self {

                let mut key_val: HashMap<String, SqlType> = value
                    .cols
                    .iter()
                    .map(|c| (c.name.to_uppercase(), c.value.clone()))
                    .collect();

                Self{
                    #(#idents : key_val.get_and_into(#uppercase .into())),*
                }

            }
        }
    };

    TokenStream::from(expanded)
}
