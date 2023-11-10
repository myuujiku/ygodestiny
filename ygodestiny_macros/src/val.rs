use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, LitFloat, Token};

use crate::kw;

pub enum Val {
    Adjustment {
        min: LitFloat,
        max: LitFloat,
        step: LitFloat,
        default: Option<LitFloat>,
    },
    AdjustmentFrom {
        source: Ident,
    },
}

impl Parse for Val {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::adjustment) {
            input.parse::<kw::adjustment>()?;

            Ok(if input.peek(kw::from) {
                input.parse::<kw::from>()?;
                Self::AdjustmentFrom {
                    source: input.parse::<Ident>()?,
                }
            } else {
                Self::Adjustment {
                    min: input.parse::<LitFloat>()?,
                    max: {
                        input.parse::<Token![..]>()?;
                        input.parse::<LitFloat>()?
                    },
                    step: {
                        input.parse::<Token![/]>()?;
                        input.parse::<LitFloat>()?
                    },
                    default: match input.peek(Token![=]) {
                        true => {
                            input.parse::<Token![=]>()?;
                            Some(input.parse::<LitFloat>()?)
                        }
                        false => None,
                    },
                }
            })
        } else {
            Err(lookahead.error())
        }
    }
}
