mod declaration;
mod implementation;
mod imports;
mod state;

use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, LitStr};

use declaration::Declaration;
use implementation::Implementation;
use imports::Imports;
use state::State;

use proc_macro::TokenStream;

struct ObjectSubclass {
    declaration: Declaration,
    imports: Imports,
    state: Option<State>,
    implementations: Vec<Implementation>,
}

impl Parse for ObjectSubclass {
    fn parse(input: ParseStream) -> Result<Self> {
        let declaration = input.parse::<Declaration>()?;

        let imports = input.parse::<Imports>()?;

        let state = if input.peek(state::kw::State) {
            Some(input.parse::<State>()?)
        } else {
            None
        };

        let mut implementations = vec![];

        while !input.is_empty() {
            implementations.push(input.parse::<Implementation>()?);
        }

        Ok(Self {
            declaration,
            imports,
            state,
            implementations,
        })
    }
}

pub fn object_subclass(input: TokenStream) -> TokenStream {
    let ObjectSubclass {
        declaration,
        imports,
        state,
        mut implementations,
    } = parse_macro_input!(input as ObjectSubclass);

    let Declaration {
        class_name,
        parent,
        extensions,
        implements,
    } = declaration;

    let implementations = {
        let mut imps = vec!["Object".to_string(), parent.object.to_string()];
        imps.extend(extensions.iter().map(|x| x.object.to_string()));

        let imps: Vec<(Ident, proc_macro2::TokenStream)> = imps
            .iter()
            .map(|a| {
                (
                    Ident::new(&format!("{a}Impl"), Span::call_site()),
                    if let Some(i) = implementations
                        .iter()
                        .position(|b| &b.target.to_string() == a)
                    {
                        implementations.swap_remove(i).content
                    } else {
                        proc_macro2::TokenStream::new()
                    },
                )
            })
            .collect();

        if !implementations.is_empty() {
            return implementations
                .iter()
                .fold(
                    proc_macro2::TokenStream::new(),
                    |mut r, x: &Implementation| {
                        r.extend(
                            syn::Error::new(
                                x.target.span(),
                                format!("{} is not a member of {class_name}", x.target),
                            )
                            .to_compile_error(),
                        );
                        r
                    },
                )
                .into();
        }

        let mut tokens = proc_macro2::TokenStream::new();
        for (imp, content) in imps {
            tokens.extend(quote!(impl #imp for #class_name { #content }));
        }

        tokens
    };

    let imports = {
        let mut tokens = proc_macro2::TokenStream::new();
        for import in imports.items {
            tokens.extend(quote!(use #import;));
        }

        tokens
    };

    let imp = {
        let state = if let Some(State { fields }) = state {
            quote!(#fields)
        } else {
            quote!(;)
        };

        let obj_id = LitStr::new(
            &format!(
                "__subclassed_{}{}_{class_name}",
                parent.prefix, parent.object
            ),
            Span::call_site(),
        );

        quote! {
            mod imp {
                use adw::prelude::*;
                use adw::subclass::prelude::*;
                use gtk::glib;

                #imports

                #[derive(Default)]
                pub struct #class_name #state

                #[glib::object_subclass]
                impl ObjectSubclass for #class_name {
                    const NAME: &'static str = #obj_id;
                    type Type = super::#class_name;
                    type ParentType = #parent;
                }

                #implementations
            }
        }
    };

    let expanded = {
        let has_impl = if !implements.is_empty() {
            Some(quote!(, @implements))
        } else {
            None
        };

        quote! {
            #imp

            gtk::glib::wrapper! {
                pub struct #class_name(ObjectSubclass<imp::#class_name>)
                    @extends #parent #(, #extensions)*
                    #has_impl #(#implements),*;
            }
        }
    };

    TokenStream::from(expanded)
}
