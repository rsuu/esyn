use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(var_ident: &Ident, fields: &Fields) -> Result<TokenStream> {
    let mut field_name = vec![];
    let mut field_ty = vec![];
    for f in fields.iter() {
        field_ty.push(&f.ty);
        field_name.push(&f.ident);
    }

    Ok(quote! {

        stringify!(#var_ident) => {
            let Expr::Call(ExprCall {
                args, ..

            }) = ast
            else { unreachable!() };

            let mut iter = args.iter();
            Self::#var_ident(
                #(
                    <#field_ty as DeRs<Expr>>::de( &iter.next().unwrap() ).unwrap(),
                )*
            )
        }

    })
}
