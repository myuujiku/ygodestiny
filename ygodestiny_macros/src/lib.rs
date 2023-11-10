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

struct ValDef {
    name: Ident,
    val: Val,
}

impl Parse for ValDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;

        Ok(Self {
            name,
            val: input.parse::<Val>()?,
        })
    }
}

enum CmdLink {
    Adjustment {
        linked: Ident,
        origin: Ident,
        modifier: Option<(BinOp, LitFloat)>,
    },
}

impl Parse for CmdLink {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::link>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::adjustment) {
            input.parse::<kw::adjustment>()?;
            Ok(Self::Adjustment {
                linked: input.parse::<Ident>()?,
                origin: {
                    input.parse::<kw::to>()?;
                    input.parse::<Ident>()?
                },
                modifier: {
                    match input.peek(Paren) {
                        true => {
                            let content;
                            parenthesized!(content in input);
                            content.parse::<kw::val>()?;
                            Some((content.parse::<BinOp>()?, content.parse::<LitFloat>()?))
                        }
                        false => None,
                    }
                },
            })
        } else {
            Err(lookahead.error())
        }
    }
}

enum Cmd {
    Link(CmdLink),
}

impl Parse for Cmd {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![#]>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::link) {
            Ok(Self::Link(input.parse::<CmdLink>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

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
