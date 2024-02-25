use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(var_ident: &Ident) -> Result<TokenStream> {
    Ok(quote! {

    // expand:
    //   "Unit1" => Self::Unit1,
    stringify!(#var_ident) => Self::#var_ident,

    })
}
