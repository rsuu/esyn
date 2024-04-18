use crate::*;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, ImplGenerics, Result,
    TypeGenerics,
};

pub fn derive_de(input: &DeriveInput) -> Result<TokenStream> {
    let ts = match &input.data {
        Data::Struct(data) => derive_struct(input, data)?,
        Data::Enum(data) => derive_enum(input, data)?,
        Data::Union(..) => return Err(Error::new(Span::call_site(), "Union is not supported")),
    };

    Ok(quote! {
        const _: () = {
            use { ::esyn::*, ::esyn::syn::* };

            #ts
        };
    })
}

fn derive_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let fields = &data.fields;

    match fields {
        Fields::Unit => ders_struct_unit::parse(input),
        Fields::Unnamed(v) => ders_struct_unnamed::parse(input, v),
        Fields::Named(v) => ders_struct_named::parse(input, v),
    }
}

fn derive_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    use syn::*;

    let enum_ident = &input.ident;
    let enum_attrs = &input.attrs;
    let generics = bound::de_trait_bounds_enum(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut enum_match = TokenStream::new();
    for var in data.variants.iter() {
        let var_ident = &var.ident;
        let fields = &var.fields;

        let ts = match fields {
            Fields::Unit => ders_enum_unit::parse(var_ident)?,
            Fields::Unnamed(..) => ders_enum_unnamed::parse(var_ident, fields)?,
            Fields::Named(..) => ders_enum_named::parse(input, var_ident, fields)?,
        };

        enum_match.extend(ts);
    }

    let (ts_impl_custom_syntax, ts_enum_attr_custom_syntax, ts_mutpath_enum_attr_custom_syntax) =
        gen_attr_custom_syntax(enum_ident, enum_attrs, &impl_generics, &ty_generics)?;

    Ok(quote! {

    #ts_impl_custom_syntax

    impl #impl_generics
        ::esyn::DeRs<syn::Expr>
    for #enum_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
            #ts_enum_attr_custom_syntax

            let var = match ast {
                // Unit
                Expr::Path(ExprPath {
                    path: Path { segments, .. },
                    ..
                })
                // Named
                | Expr::Struct(ExprStruct {
                    path: Path { segments, .. },
                    ..
                }) => {
                    debug_assert_eq!(segments.len(), 2);

                    let PathSegment { ident, .. } = &segments[1];

                    ident.to_string()
                }

                // Unnamed
                Expr::Call(ExprCall { func, .. }) => {
                    let Expr::Path(ExprPath { path:Path {  segments ,..},.. }) = func.as_ref()
                    else {unreachable!()};

                    debug_assert_eq!(segments.len(), 2);

                    let PathSegment { ident, .. } = &segments[1];

                    ident.to_string()
                }

                _ => unreachable!(),
            };

            Ok(match var.as_str() {
                #enum_match
                _ => unreachable!(),
            })
        }
    }

    impl #impl_generics
        ::esyn::MutPath
    for #enum_ident #ty_generics
        #where_clause
    {
        fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
            #ts_mutpath_enum_attr_custom_syntax

            *self = <Self as DeRs<Expr>>::de(ast)?;

            Ok(())
        }
    }

    })
}

pub fn gen_attr_custom_syntax(
    ident: &Ident,
    attrs: &[Attribute],
    impl_generics: &ImplGenerics<'_>,
    ty_generics: &TypeGenerics<'_>,
) -> Result<(TokenStream, TokenStream, TokenStream)> {
    let (ts_attr_custom_syntax, ts_mutpath_attr_custom_syntax) = 's: {
        // if `Self` has `#[custom_syntax]`
        //   generate code
        // else
        //   skip

        if attr::attr_custom_syntax(attrs).is_none() {
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

        if ts_attr_custom_syntax.is_empty() && ts_mutpath_attr_custom_syntax.is_empty() {
            quote! {

                impl #impl_generics
                    ::esyn::CustomSyntax
                for #ident #ty_generics
                {}

            }
        } else {
            quote! {}
        }
    };

    Ok((
        ts_impl_custom_syntax,
        ts_attr_custom_syntax,
        ts_mutpath_attr_custom_syntax,
    ))
}
