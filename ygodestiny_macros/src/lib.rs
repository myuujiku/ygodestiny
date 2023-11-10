use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
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

struct RowAttrs {
    load_fn: Ident,
    collect_fn: Ident,
    convert_to: Option<Type>,
    load_value: Option<TS2>,
}

enum Row {
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
    fn fill_tokens(&self, build_content: &mut TS2) -> RowAttrs {
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

struct RowDef {
    root: Ident,
    name: Ident,
    row: Row,
}

impl RowDef {

}

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

enum Val {
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
