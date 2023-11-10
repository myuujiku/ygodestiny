use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

use crate::Val;

pub struct ValDef {
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
