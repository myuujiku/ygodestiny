use syn::parse::{Parse, ParseStream, Result};
use syn::token::Paren;
use syn::{parenthesized, BinOp, Ident, LitFloat, Token};

use crate::kw;

pub enum CmdLink {
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

pub enum Cmd {
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
