use proc_macro::TokenStream;

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

mod row;
use row::Row;

mod row_def;
use row_def::RowDef;

mod val;
use val::Val;

mod val_def;
use val_def::ValDef;

mod cmd;
use cmd::Cmd;

mod statement;
use statement::Statement;

mod settings;
use settings::Settings;

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
