use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Paren;
use syn::{
    braced, bracketed, parenthesized, parse_macro_input, BinOp, Ident, LitBool, LitFloat, LitStr,
    Token, Type,
};

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

enum Statement {
    RowDef(RowDef),
    ValDef(ValDef),
    Cmd(Cmd),
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![@]) {
            Ok(Self::RowDef(input.parse::<RowDef>()?))
        } else if lookahead.peek(Ident) {
            Ok(Self::ValDef(input.parse::<ValDef>()?))
        } else if lookahead.peek(Token![#]) {
            Ok(Self::Cmd(input.parse::<Cmd>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct Setting {
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

struct Settings {
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

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
