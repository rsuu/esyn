use crate::{attr, bound};

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(input: &DeriveInput, var_ident: &Ident, fields: &Fields) -> Result<TokenStream> {
    let enum_ident = &input.ident;
    let enum_attrs = &input.attrs;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let len = fields.len();
    let mut field_name = Vec::with_capacity(len);
    let mut field_ty = Vec::with_capacity(len);
    for f in fields.iter() {
        field_ty.push(&f.ty);
        field_name.push(&f.ident);
    }

    let (ts_impl_custom_syntax, ts_enum_attr_custom_syntax, ts_mutpath_enum_attr_custom_syntax) =
        gen_attr_custom_syntax(enum_ident, enum_attrs, &impl_generics, &ty_generics)?;

    let enum_match = quote! {

            // expand:
            //   "NamedField" => { ... }
            stringify!(#var_ident) => {
    //            // TODO: auto impl
    //            let ast = {
    //                if let Ok(expr) = <Self as ::esyn::CustomSyntax>::parse(ast) {
    //                    return Ok(<Self as DeRs<Expr>>::de(&expr)?);
    //                } else {
    //                    ast
    //                }
    //            };

                //dbg!(ast);
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

fn gen_attr_custom_syntax(
    enum_ident: &Ident,
    enum_attrs: &[Attribute],
    impl_generics: &ImplGenerics<'_>,
    ty_generics: &TypeGenerics<'_>,
) -> Result<(TokenStream, TokenStream, TokenStream)> {
    let (ts_enum_attr_custom_syntax, ts_mutpath_enum_attr_custom_syntax) = 's: {
        // if `Self` has `#[custom_syntax]`
        //   generate code
        // else
        //   skip

        if attr::attr_custom_syntax(enum_attrs)?.is_none() {
            break 's (quote! {}, quote! {});
        }

        let return_val = quote! {

            let ast = {
                if let Ok(expr) = <Self as ::esyn::CustomSyntax>::parse(ast) {
                    return Ok(<Self as DeRs<Expr>>::de(&expr)?);
                } else {
                    ast
                }
            };

        };
        let set_val = quote! {

            let ast = {
                if let Ok(expr) = <Self as ::esyn::CustomSyntax>::parse(ast) {
                    *self = <Self as DeRs<Expr>>::de(&expr)?;

                    return Ok(());
                } else {
                    ast
                }
            };

        };

        (return_val, set_val)
    };
    let ts_impl_custom_syntax = {
        // if not-impl `CustomSyntax` for `Self`
        //   auto impl
        // else
        //   skip

        if ts_enum_attr_custom_syntax.is_empty() && ts_mutpath_enum_attr_custom_syntax.is_empty() {
            quote! {

                impl #impl_generics
                    ::esyn::CustomSyntax
                for #enum_ident #ty_generics
                {}

            }
        } else {
            quote! {}
        }
    };

    Ok((
        ts_impl_custom_syntax,
        ts_enum_attr_custom_syntax,
        ts_mutpath_enum_attr_custom_syntax,
    ))
}
