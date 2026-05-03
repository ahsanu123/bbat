use proc_macro::TokenStream;

mod from_row;
mod get_fields;

pub(crate) use from_row::*;
pub(crate) use get_fields::*;

#[proc_macro_derive(GetFields)]
pub fn get_fields(input: TokenStream) -> TokenStream {
    get_fields_macro(input)
}

#[proc_macro_derive(FromRow)]
pub fn from_row(input: TokenStream) -> TokenStream {
    from_row_macro(input)
}
