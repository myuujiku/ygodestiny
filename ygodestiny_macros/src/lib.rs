mod subclass;

use proc_macro::TokenStream;

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn object_subclass(input: TokenStream) -> TokenStream {
    subclass::object_subclass(input)
}
