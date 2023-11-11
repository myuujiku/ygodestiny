mod kw {
    syn::custom_keyword!(root);
    syn::custom_keyword!(expander);
    syn::custom_keyword!(spin);
    syn::custom_keyword!(switch);
    syn::custom_keyword!(adjustment);
    syn::custom_keyword!(from);
    syn::custom_keyword!(to);
    syn::custom_keyword!(link);
    syn::custom_keyword!(val);
}

mod cmd;
mod row;
mod row_def;
mod settings;
mod statement;
mod subclass;
mod val;
mod val_def;

use proc_macro::TokenStream;

use cmd::Cmd;
use row::Row;
use row_def::RowDef;
use settings::Settings;
use statement::Statement;
use val::Val;
use val_def::ValDef;

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn object_subclass(input: TokenStream) -> TokenStream {
    subclass::object_subclass(input)
}
