use crate::{attr, bound};

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(input: &DeriveInput, var_ident: &Ident, fields: &Fields) -> Result<TokenStream> {
    let generics = bound::de_trait_bounds_struct(input.generics.clone());

    let len = fields.len();
    let mut field_name = Vec::with_capacity(len);
    let mut field_ty = Vec::with_capacity(len);
    for f in fields.iter() {
        field_ty.push(&f.ty);
        field_name.push(&f.ident);
    }

    let enum_match = quote! {

        // expand:
        //   "NamedField" => { ... }
        stringify!(#var_ident) => {
            let Expr::Struct(ExprStruct {
                fields, ..
            }) = ast
            else { unreachable!() };

            Self::#var_ident {
                #(
                #field_name: {
                    let expr = ExprHelper::get_named_field_expr(
                        fields,
                        stringify!(#field_name)
                    );

                    if let Ok(expr) = expr {
                        <#field_ty as DeRs<Expr>>::de(expr)?
                    } else {
                        EsynDefault::esyn_default()
                    }
                },
                )*
            }
        }

    };

    Ok(enum_match)
}
