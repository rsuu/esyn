use crate::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(input: &DeriveInput) -> Result<TokenStream> {
    let struct_ident = &input.ident;
    let struct_attrs = &input.attrs;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (ts_impl_custom_syntax, ts_struct_attr_custom_syntax, ts_mutpath_struct_attr_custom_syntax) =
        ders::gen_attr_custom_syntax(struct_ident, struct_attrs, &impl_generics, &ty_generics)?;

    Ok(quote! {

    #ts_impl_custom_syntax

    impl #impl_generics
        esyn::DeRs<syn::Expr>
    for #struct_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
            #ts_struct_attr_custom_syntax

            let Expr::Path(ExprPath {
                path: Path { segments, .. },
                ..
            }) = ast
            else { unreachable!() };

            if segments[0].ident == stringify!(#struct_ident) {
                Ok(Self)
            } else {
                unreachable!()
            }
        }
    }

    impl #impl_generics
        esyn::MutPath
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
