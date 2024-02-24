use crate::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

pub fn parse(struct_impl: &mut TokenStream, input: &DeriveInput) -> Result<()> {
    let struct_ident = &input.ident;
    let generics = bound::de_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    struct_impl.extend(quote! {

    impl #impl_generics
        esyn::DeRs<syn::Expr>
    for #struct_ident #ty_generics
        #where_clause
    {
        fn de(ast: &syn::Expr) -> esyn::Res<Self> {
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
            *self = <Self as DeRs<Expr>>::de(ast)?;

            Ok(())
        }
    }

    });

    Ok(())
}
