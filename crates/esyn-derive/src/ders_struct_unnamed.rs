use crate::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(input: &DeriveInput, fields: &FieldsUnnamed) -> Result<TokenStream> {
    let struct_ident = &input.ident;
    let struct_attrs = &input.attrs;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_ty: &Vec<_> = &fields.unnamed.iter().map(|f| &f.ty).collect();
    let idx: Vec<_> = {
        let mut res = vec![];
        for n in 0..fields.unnamed.len() {
            res.push(Index::from(n));
        }

        res
    };

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

            let Expr::Call(ExprCall {
                func, args,
                ..
            }) = ast
            else { unreachable!() };

            let Expr::Path(ExprPath {
                path: Path { segments, .. },
                ..
            }) = func.as_ref()
            else { unreachable!() };

            debug_assert_eq!(
                &segments[0].ident,
                stringify!(#struct_ident)
            );

            Ok(Self(
                #(
                    <#field_ty as DeRs<Expr>>::de(&args[#idx])? ,
                )*
            ))
        }
    }

    impl #impl_generics
        ::esyn::MutPath
    for #struct_ident #ty_generics
        #where_clause
    {
        fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
            #ts_mutpath_struct_attr_custom_syntax

            *self = <Self as DeRs<Expr>>::de(ast)?;

            Ok(())
        }
    }

    })
}
