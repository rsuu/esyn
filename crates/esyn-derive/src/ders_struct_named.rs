use crate::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    let struct_ident = &input.ident;
    let struct_attrs = &input.attrs;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_name = vec![];
    let mut field_ty = vec![];
    for f in fields.named.iter() {
        field_name.push(&f.ident);
        field_ty.push(&f.ty);
    }

    let (ts_impl_custom_syntax, ts_struct_attr_custom_syntax, ts_mutpath_struct_attr_custom_syntax) =
        ders::gen_attr_custom_syntax(struct_ident, struct_attrs, &impl_generics, &ty_generics)?;

    Ok(quote! {

    #ts_impl_custom_syntax

    impl #impl_generics
        ::esyn::DeRs<syn::Expr>
    for #struct_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
            #ts_struct_attr_custom_syntax

            //dbg!(ast);
            let Expr::Struct(ExprStruct {
                fields, ..
            }) = ast
            else { unreachable!() };

            Ok(Self {
                #(
                #field_name: {
                    if let Ok(expr) = ExprHelper::get_named_field_expr(
                        fields,
                        stringify!(#field_name)
                    ) {
                        <#field_ty as DeRs<Expr>>::de(expr)?
                    } else {
                        EsynDefault::esyn_default()
                    }
                },
                )*
            })
        }
    }

    impl #impl_generics
        ::esyn::MutPath
    for #struct_ident #ty_generics
        #where_clause
    {
        fn mut_path(
            &mut self,
            iter: &mut std::slice::Iter<&Ident>,
            ast: &syn::Expr
        ) -> esyn::Res<()> {
            #ts_mutpath_struct_attr_custom_syntax

            if let Some(i) = iter.next() {
                match i.to_string().as_str() {
                    // expand:
                    //   "_u8" => self._u8.mut_path(iter, ast)?,
                    #(
                    stringify!(#field_name) => {
                        self.#field_name.mut_path(iter, ast)?;
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

    })
}
