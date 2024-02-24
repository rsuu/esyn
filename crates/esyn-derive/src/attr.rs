// TODO:
//  - #[parse]
//    struct {unnamed}
//    enum {named, unnamed}

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    *,
};

pub fn attr_parse(attrs: &[Attribute]) -> Result<Option<AttrParse>> {
    for attr in attrs {
        let ts = attr.into_token_stream();
        let attr = syn::parse2::<AttrParse>(ts)?;

        if attr.ident.to_string().as_str() == "parse" {
            return Ok(Some(attr));
        }
    }

    Ok(None)
}

// e.g.
//   #[parse(Expr::Macro => parse_macro())]
#[derive(Debug)]
pub struct AttrParse {
    _ts0: Token![#],

    _ts1: token::Bracket,
    ident: Ident,

    _ts2: token::Paren,
    args: punctuated::Punctuated<AttrParseArg, Token![,]>,
}

// e.g.
//   Expr::Macro => parse_macro(),
#[derive(Debug)]
struct AttrParseArg {
    path: Path,
    _ts3: Token![=>],
    func: Path,
    _ts4: token::Paren,
}

impl AttrParse {
    pub fn gen_code(&self) -> TokenStream {
        let mut ts = TokenStream::new();

        for AttrParseArg { path, func, .. } in self.args.iter() {
            let ty = path.segments.last().unwrap();
            let fty = {
                match ty.ident.to_string().as_str() {
                    "Macro" => quote! { get_macro_name },
                    "Call" => quote! { get_call_name },
                    _ => unimplemented!(),
                }
            };

            ts.extend(quote! {
                #path(expr) => {
                    if ExprHelper::#fty(expr) != stringify!(#func) {
                        panic!()
                    }

                    Some(#func(expr))
                },
            });
        }

        ts
    }
}

impl Parse for AttrParse {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg1;
        let arg2;

        Ok(Self {
            _ts0: input.parse()?,

            _ts1: bracketed!(arg1 in input),
            ident: arg1.parse()?,

            _ts2: parenthesized!(arg2 in arg1),
            args: arg2.parse_terminated(AttrParseArg::parse, Token![,])?,
        })
    }
}

impl Parse for AttrParseArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let _end;

        Ok(Self {
            path: input.parse()?,
            _ts3: input.parse()?,
            func: input.parse()?,
            _ts4: parenthesized!(_end in input),
        })
    }
}
