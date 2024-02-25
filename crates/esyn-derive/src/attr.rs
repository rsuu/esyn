// TODO:
//  - #[parse]
//    struct {unnamed}
//    enum {named, unnamed}

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

pub fn attr_custom_syntax(attrs: &[Attribute]) -> Result<Option<AttrParse>> {
    for attr in attrs {
        let ts = attr.into_token_stream();
        let attr = syn::parse2::<AttrParse>(ts)?;

        if attr.ident.to_string().as_str() == "custom_syntax" {
            return Ok(Some(attr));
        }
    }

    Ok(None)
}

// e.g.
//   #[parse]
#[derive(Debug)]
pub struct AttrParse {
    _ts0: Token![#],

    _ts1: token::Bracket,
    ident: Ident,
    //_ts2: token::Paren,
    //args: punctuated::Punctuated<Path, Token![,]>,
}

impl Parse for AttrParse {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg1;
        //let arg2;

        Ok(Self {
            _ts0: input.parse()?,

            _ts1: bracketed!(arg1 in input),
            ident: arg1.parse()?,
            //_ts2: parenthesized!(arg2 in arg1),
            //args: arg2.parse_terminated(Path::parse, Token![,])?,
        })
    }
}
