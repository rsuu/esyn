use crate::*;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result};

pub fn derive_de(input: &DeriveInput) -> Result<TokenStream> {
    let ts = match &input.data {
        Data::Struct(data) => derive_struct(input, data)?,
        Data::Enum(data) => derive_enum(input, data)?,
        Data::Union(..) => return Err(Error::new(Span::call_site(), "Union is not supported")),
    };

    Ok(quote! {
        const _: () = {
            use {::esyn::*, ::esyn::syn::*};

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
    let enum_ident = &input.ident;
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

    Ok(quote! {

    impl #impl_generics
        ::esyn::DeRs<syn::Expr>
    for #enum_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
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
            *self = <Self as DeRs<Expr>>::de(ast)?;
            Ok(())
        }
    }

    })
}
