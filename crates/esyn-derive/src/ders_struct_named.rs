use crate::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(
    struct_impl: &mut TokenStream,
    input: &DeriveInput,
    fields: &FieldsNamed,
) -> Result<()> {
    let struct_ident = &input.ident;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_name = vec![];
    let mut field_ty = vec![];
    let mut field_attr_parse = vec![];
    for f in fields.named.iter() {
        field_name.push(&f.ident);
        field_ty.push(&f.ty);

        if let Some(attr_parse) = attr::attr_parse(&f.attrs)? {
            field_attr_parse.push(attr_parse.gen_code());
        } else {
            field_attr_parse.push(quote! {});
        }
    }

    struct_impl.extend(quote! {

    impl #impl_generics
        esyn::DeRs<syn::Expr>
    for #struct_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
            //dbg!(ast);
            let Expr::Struct(ExprStruct {
                fields, ..
            }) = ast
            else { unreachable!() };


            Ok(Self {
                // TODO:
                // expand:
                #(
                    #field_name: {
                        if let Ok(tmp) = ExprHelper::get_named_field_expr(
                            fields,
                            stringify!(#field_name)
                        ) {
                            let expr: Option<Res<Expr>> = {
                                match tmp {
                                    #field_attr_parse
                                    _ => Some(Ok(tmp.clone())),
                                }
                            };


                            <#field_ty as DeRs<Expr>>::de(&expr.unwrap()?)?
                        } else {
                            EsynDefault::esyn_default()
                        }
                    },
                )*
            })
        }
    }

    impl #impl_generics
        esyn::MutPath
    for #struct_ident #ty_generics
        #where_clause
    {
        fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> esyn::Res<()> {
            if let Some(i) = iter.next() {
                match i.to_string().as_str() {
                    // expand:
                    //   "_u8" => self._u8.mut_path(iter, ast)?,
                    #(
                        stringify!(#field_name) => {
                            let expr: Option<Res<Expr>> = {
                                let tmp = match ast {
                                    #field_attr_parse
                                    _ => None,
                                };

                                tmp
                            };

                            if let Some(expr) = expr {
                               self.#field_name = <#field_ty as DeRs<Expr>>::de(&expr?)?;
                            } else {
                                self.#field_name.mut_path(iter, ast)?;
                            }
                        },
                    )*

                    _ => unreachable!("{i:#?}")
                }
            } else {
                *self = <Self as DeRs<Expr>>::de(ast)?;
            }

            Ok(())
        }
    }

    });

    Ok(())
}
