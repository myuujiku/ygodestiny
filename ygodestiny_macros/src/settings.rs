use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, Ident, Token};

use crate::{kw, Row, Statement};

pub struct Setting {
    name: Ident,
    root: Row,
    statements: Vec<Statement>,
}

impl Parse for Setting {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        content.parse::<Token![#]>()?;
        content.parse::<kw::root>()?;
        content.parse::<Token![:]>()?;

        let root = content.parse::<Row>()?;
        content.parse::<Token![;]>()?;

        let statements = Punctuated::<Statement, Token![;]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Self {
            name,
            root,
            statements,
        })
    }
}

pub struct Settings {
    settings: Vec<Setting>,
}

impl Parse for Settings {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut settings = Vec::new();

        while !input.is_empty() {
            settings.push(input.parse::<Setting>()?);
        }

        Ok(Self { settings })
    }
}
