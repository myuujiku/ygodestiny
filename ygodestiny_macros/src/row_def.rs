use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

use crate::Row;

pub struct RowDef {
    root: Ident,
    name: Ident,
    row: Row,
}

impl RowDef {}

impl Parse for RowDef {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            root: {
                input.parse::<Token![@]>()?;
                input.parse::<Ident>()?
            },
            name: input.parse::<Ident>()?,
            row: {
                input.parse::<Token![:]>()?;
                input.parse::<Row>()?
            },
        })
    }
}
