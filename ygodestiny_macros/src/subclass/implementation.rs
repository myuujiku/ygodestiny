use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Ident, Token};

pub struct Implementation {
    pub target: Ident,
    pub content: TokenStream,
}

impl Implementation {
    pub fn parse_self(input: ParseStream, target: Ident) -> Result<Self> {
        input.parse::<Token![impl]>()?;
        input.parse::<Token![Self]>()?;
        let content;
        braced!(content in input);
        let content = content.parse::<TokenStream>()?;

        Ok(Self { target, content })
    }
}

impl Parse for Implementation {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![impl]>()?;
        let target = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let content = content.parse::<TokenStream>()?;

        Ok(Self { target, content })
    }
}
