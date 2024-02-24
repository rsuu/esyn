use crate::bound;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Index, Result};

pub fn derive_default(input: &DeriveInput) -> Result<TokenStream> {
    let ts = match &input.data {
        Data::Struct(data) => derive_struct(input, data)?,
        Data::Enum(data) => derive_enum(input, data)?,
        Data::Union(..) => return Err(Error::new(Span::call_site(), "Union is not supported")),
    };

    Ok(quote! {
        const _: () = {
            use {::esyn::syn::*, ::esyn::*};

            #ts
        };
    })
}

fn derive_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let struct_ident = &input.ident;
    let generics = bound::default_add_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut struct_impl = TokenStream::new();
    match data {
        DataStruct {
            fields: Fields::Named(fields),
            ..
        } => {
            let mut field_name = vec![];
            let mut field_ty = vec![];
            for f in fields.named.iter() {
                field_ty.push(&f.ty);
                field_name.push(&f.ident);
            }

            // impl struct named
            #[rustfmt::skip]
struct_impl.extend(quote! {
impl #impl_generics
    esyn::EsynDefault
for #struct_ident #ty_generics
    #where_clause
{
    fn esyn_default() -> Self {
        Self {
            #(
                #field_name: <#field_ty as EsynDefault>::esyn_default(),
            )*
        }
    }
}

});
        }

        DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        } => {
            let field_ty: &Vec<_> = &fields.unnamed.iter().map(|f| &f.ty).collect();
            let _idx: Vec<_> = {
                let mut res = vec![];
                for n in 0..fields.unnamed.len() {
                    res.push(Index::from(n));
                }

                res
            };

            // impl struct unamed
            #[rustfmt::skip]
struct_impl.extend(quote! {
impl #impl_generics
    esyn::EsynDefault
for #struct_ident #ty_generics
    #where_clause
{
    fn esyn_default() -> Self {
        Self(
            #(
                <#field_ty as EsynDefault>::esyn_default(),
            )*
        )
    }
}

});
        }

        DataStruct {
            fields: Fields::Unit,
            ..
        } => {
            // impl struct unit
            #[rustfmt::skip]
struct_impl.extend(quote! {
impl #impl_generics
    esyn::EsynDefault
for #struct_ident #ty_generics
    #where_clause
{
    fn esyn_default() -> Self {
        Self
    }
}

});
        }
    }

    Ok(struct_impl)
}

fn derive_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let enum_ident = &input.ident;
    let generics = bound::default_add_trait_bounds_enum(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut enum_impl = TokenStream::new();
    let _enum_match = TokenStream::new();
    let mut enum_default = TokenStream::new();
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
                #[rustfmt::skip]
                enum_default.extend(quote! {
return Self::#var_ident;
});
            }

            // impl enum unnamed
            Fields::Unnamed(_) => {
                #[rustfmt::skip]
                enum_default.extend(quote! {
return Self::#var_ident(
    #(
        <#field_ty as EsynDefault>::esyn_default(),
    )*
);

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

                #[rustfmt::skip]
                enum_default.extend(quote! {
return Self::#var_ident {
    #(
        #field_name: <#field_ty as EsynDefault>::esyn_default(),
    )*
};
});
            }
        }
    }

    // impl enum
    #[rustfmt::skip]
enum_impl.extend(quote! {
impl #impl_generics
    esyn::EsynDefault
for #enum_ident #ty_generics
    #where_clause
{
    fn esyn_default() ->Self {
        #enum_default
    }
}

});

    Ok(enum_impl)
}
