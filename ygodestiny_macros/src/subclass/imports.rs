use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, Token, UseTree};

mod kw {
    syn::custom_keyword!(imports);
}

pub struct Imports {
    pub items: Vec<UseTree>,
}

impl Parse for Imports {
    fn parse(input: ParseStream) -> Result<Self> {
        let items = if input.peek(kw::imports) {
            input.parse::<kw::imports>()?;
            input.parse::<Token![=]>()?;
            let content;
            bracketed!(content in input);
            let content = content.parse_terminated(UseTree::parse, Token![;])?;
            input.parse::<Token![;]>()?;

            content.into_iter().collect()
        } else {
            vec![]
        };

        Ok(Self { items })
    }
}
