mod subclass;

use proc_macro::TokenStream;

#[proc_macro]
pub fn object_subclass(input: TokenStream) -> TokenStream {
    subclass::object_subclass(input)
}
