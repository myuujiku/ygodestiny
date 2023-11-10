use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

use crate::{Cmd, RowDef, ValDef};

pub enum Statement {
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
