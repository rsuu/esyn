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
    let mut struct_impl = TokenStream::new();
    match data {
        DataStruct {
            fields: Fields::Named(fields),
            ..
        } => {
            ders_struct_named::parse(&mut struct_impl, input, fields)?;
        }

        DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        } => {
            ders_struct_unnamed::parse(&mut struct_impl, input, fields)?;
        }

        DataStruct {
            fields: Fields::Unit,
            ..
        } => {
            ders_struct_unit::parse(&mut struct_impl, input)?;
        }
    }

    Ok(struct_impl)
}

fn derive_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let enum_ident = &input.ident;
    let generics = bound::de_trait_bounds_enum(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut enum_impl = TokenStream::new();
    let mut enum_match = TokenStream::new();
    let _enum_default = TokenStream::new();
    let _enum_to_tokens = TokenStream::new();
    for var in data.variants.iter() {
        let var_ident = &var.ident;
        let fields = &var.fields;

        //let field_name: &Vec<_> = &fields.iter().map(|f| &f.ident).collect();
        let field_ty: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();

        let _match_name = format!("{}::{}", enum_ident, var_ident);

        match fields {
            // impl enum unit
            Fields::Unit => {
                enum_match.extend(quote! {
                // expand:
                //   "Unit1" => Self::Unit1,
                stringify!(#var_ident) => Self::#var_ident,
                });
            }

            // impl enum unnamed
            Fields::Unnamed(_) => {
                enum_match.extend(quote! {
                stringify!(#var_ident) => {
                    let Expr::Call(ExprCall {
                        args, ..
                    }
                    ) = ast
                    else { unreachable!() };

                    let mut iter = args.iter();
                    Self::#var_ident(
                        #(
                            <#field_ty as DeRs<Expr>>::de( &iter.next().unwrap() ).unwrap(),
                        )*
                    )
                }

                });
            }

            // impl enum named
            Fields::Named(_) => {
                let mut field_name = vec![];
                let mut field_ty = vec![];
                for f in fields.iter() {
                    field_ty.push(&f.ty);
                    field_name.push(&f.ident);
                }

                enum_match.extend(quote! {
                // expand:
                //   "NamedField" => { ... }
                stringify!(#var_ident) => {
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

                });
            }
        }
    }

    // impl enum
    enum_impl.extend(quote! {
    impl #impl_generics
        esyn::DeRs<syn::Expr>
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
        esyn::MutPath
    for #enum_ident #ty_generics
        #where_clause
    {
        fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
            *self = <Self as DeRs<Expr>>::de(ast)?;
            Ok(())
        }
    }

    });

    Ok(enum_impl)
}
