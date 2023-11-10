use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{bracketed, Ident, LitBool, LitStr, Token, Type};

use crate::kw;

pub struct RowAttrs {
    load_fn: Ident,
    collect_fn: Ident,
    convert_to: Option<Type>,
    load_value: Option<TokenStream>,
}

pub enum Row {
    Expander {
        default: Option<LitBool>,
        title: LitStr,
        subtitle: Option<LitStr>,
    },
    Spin {
        convert_to: Option<Type>,
        adjustment: Ident,
        title: LitStr,
        subtitle: Option<LitStr>,
    },
    Switch {
        default: Option<LitBool>,
        title: LitStr,
        subtitle: Option<LitStr>,
    },
}

impl Row {
    pub fn fill_tokens(&self, build_content: &mut TokenStream) -> RowAttrs {
        match self {
            Row::Expander {
                default,
                title,
                subtitle,
            } => {
                build_content.extend(quote! {
                    adw::ExpanderRow::builder()
                        .show_enable_switch(true)
                        .expanded(false)
                        .title(#title)
                });
                if let Some(subtitle) = subtitle {
                    build_content.extend(quote!(.subtitle(#subtitle)));
                }
                if let Some(default) = default {
                    build_content.extend(quote!(.value(#default)));
                }
                build_content.extend(quote!(.build();));

                RowAttrs {
                    load_fn: Ident::new("set_enable_expansion", Span::call_site()),
                    collect_fn: Ident::new("enables_expansion", Span::call_site()),
                    convert_to: None,
                    load_value: Some(quote!(true)),
                }
            }
            Row::Spin {
                convert_to,
                adjustment,
                title,
                subtitle,
            } => {
                build_content.extend(quote! {
                    adw::SpinRow::builder()
                        .adjustment(&#adjustment)
                        .title(#title)
                });
                if let Some(subtitle) = subtitle {
                    build_content.extend(quote!(.subtitle(#subtitle)));
                }
                build_content.extend(quote!(.build();));

                RowAttrs {
                    load_fn: Ident::new("value", Span::call_site()),
                    collect_fn: Ident::new("set_value", Span::call_site()),
                    convert_to: convert_to.clone(),
                    load_value: None,
                }
            }
            Row::Switch {
                default,
                title,
                subtitle,
            } => {
                build_content.extend(quote! {
                    adw::SwitchRow::builder()
                        .title(#title)
                });
                if let Some(subtitle) = subtitle {
                    build_content.extend(quote!(.subtitle(#subtitle)));
                }
                if let Some(default) = default {
                    build_content.extend(quote!(.value(#default)));
                }
                build_content.extend(quote!(.build();));

                RowAttrs {
                    load_fn: Ident::new("is_active", Span::call_site()),
                    collect_fn: Ident::new("set_active", Span::call_site()),
                    convert_to: None,
                    load_value: None,
                }
            }
        }
    }
}

impl Parse for Row {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::expander) {
            input.parse::<kw::expander>()?;
            Ok(Self::Expander {
                default: match input.peek(Token![=]) {
                    true => {
                        input.parse::<Token![=]>()?;
                        Some(input.parse::<LitBool>()?)
                    }
                    false => None,
                },
                title: input.parse()?,
                subtitle: match input.peek(LitStr) {
                    true => Some(input.parse::<LitStr>()?),
                    false => None,
                },
            })
        } else if lookahead.peek(kw::spin) {
            input.parse::<kw::spin>()?;
            Ok(Self::Spin {
                convert_to: {
                    if input.peek(Token![->]) {
                        input.parse::<Token![->]>()?;
                        Some(input.parse::<Type>()?)
                    } else {
                        None
                    }
                },
                adjustment: {
                    let content;
                    bracketed!(content in input);
                    content.parse::<Token![@]>()?;
                    content.parse::<Ident>()?
                },
                title: input.parse()?,
                subtitle: match input.peek(LitStr) {
                    true => Some(input.parse::<LitStr>()?),
                    false => None,
                },
            })
        } else if lookahead.peek(kw::switch) {
            input.parse::<kw::switch>()?;
            Ok(Self::Switch {
                default: match input.peek(Token![=]) {
                    true => {
                        input.parse::<Token![=]>()?;
                        Some(input.parse::<LitBool>()?)
                    }
                    false => None,
                },
                title: input.parse()?,
                subtitle: match input.peek(LitStr) {
                    true => Some(input.parse::<LitStr>()?),
                    false => None,
                },
            })
        } else {
            Err(lookahead.error())
        }
    }
}
