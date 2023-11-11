use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, Ident, Token};

mod kw {
    syn::custom_keyword!(adw);
}

pub struct GtkIdent {
    pub libadwaita: bool,
    pub object: Ident,
}

impl Parse for GtkIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let libadwaita = input.peek(kw::adw);

        if libadwaita {
            input.parse::<kw::adw>()?;
        }

        let object = input.parse::<Ident>()?;

        Ok(Self { libadwaita, object })
    }
}

impl ToTokens for GtkIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { libadwaita, object } = self;
        if *libadwaita {
            tokens.extend(quote!(adw::#object))
        } else {
            tokens.extend(quote!(gtk::#object))
        }
    }
}

pub struct Declaration {
    pub class_name: Ident,
    pub parent: GtkIdent,
    pub extensions: Vec<GtkIdent>,
    pub implements: Vec<GtkIdent>,
}

impl Parse for Declaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let class_name = input.parse::<Ident>()?;

        let content;
        parenthesized!(content in input);
        let parent = content.parse::<GtkIdent>()?;

        let extensions = if content.peek(Token![+]) {
            content.parse::<Token![+]>()?;
            content
                .parse_terminated(GtkIdent::parse, Token![,])?
                .into_iter()
                .collect()
        } else {
            vec![]
        };

        let implements = if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            let content;
            parenthesized!(content in input);
            content
                .parse_terminated(GtkIdent::parse, Token![,])?
                .into_iter()
                .collect()
        } else {
            vec![]
        };

        input.parse::<Token![;]>()?;

        Ok(Self {
            class_name,
            parent,
            extensions,
            implements,
        })
    }
}
