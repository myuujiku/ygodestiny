use syn::parse::{Parse, ParseStream, Result};
use syn::FieldsNamed;

pub mod kw {
    syn::custom_keyword!(State);
}

pub struct State {
    pub fields: FieldsNamed,
}

impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::State>()?;
        let fields = input.parse::<FieldsNamed>()?;

        Ok(Self { fields })
    }
}
