use crate::bound;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields,
    Ident, Index, Result,
};

pub fn derive_de(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => derive_struct(&input, data),
        Data::Enum(data) => derive_enum(&input, data),

        Data::Union(..) => Err(Error::new(Span::call_site(), "Union is not supported")),
    }
}

fn derive_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let struct_ident = &input.ident;
    let generics = bound::add_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut ts_res = TokenStream::new();

    ts_res.extend(quote! {
        impl #impl_generics esyn::TypeInfo
         for #struct_ident #ty_generics
             #where_clause
        {
            fn name() -> &'static str {
                stringify!(#struct_ident)
            }
        }
    });

    match data {
        DataStruct {
            fields: Fields::Named(fields),
            ..
        } => {
            let name: &Vec<_> = &fields.named.iter().map(|f| &f.ident).collect();
            let ty: &Vec<_> = &fields.named.iter().map(|f| &f.ty).collect();

            ts_res.extend(quote! {
            impl #impl_generics esyn::Ast
             for #struct_ident #ty_generics
                 #where_clause
            {
                fn ast() -> String {
                    let mut tmp = "".to_string();

                    #(
                    tmp.push_str(
                        &format!(
                            "{}:{},",
                            stringify!(#name),
                            <#ty as esyn::Ast>::ast()
                        )
                    );
                    )*

                    format!(
                        "{} {{ {} }}",
                        stringify!(#struct_ident),
                        tmp
                    )
                }
            }

            impl #impl_generics esyn::Bytes
             for #struct_ident #ty_generics
                 #where_clause
                {
                fn from_bytes(mut buf: impl esyn::ParseBytes) -> esyn::Res<Self> {
                    let mut res = Self::default();

                    #(
                    res.#name = <#ty as esyn::Bytes>::from_bytes(&mut buf)?;
                    )*

                    Ok(res)
                }

            }
            });
        }

        DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        } => {
            let ty: &Vec<_> = &fields.unnamed.iter().map(|f| &f.ty).collect();
            let idx: Vec<usize> = (0..ty.len()).collect();

            ts_res.extend(quote! {
            impl #impl_generics esyn::Ast
             for #struct_ident #ty_generics
                 #where_clause
            {
                fn ast() -> String {
                    let mut tmp = "".to_string();

                    #(
                    tmp.push_str(
                        &format!(
                            "{},",
                            <#ty as esyn::Ast>::ast()
                        )
                    );
                    )*

                    format!(
                        "{} ( {} )",
                        stringify!(#struct_ident),
                        tmp
                    )
                }
            }

            impl #impl_generics esyn::Bytes
             for #struct_ident #ty_generics
                 #where_clause
                {
                fn from_bytes(mut buf: impl esyn::ParseBytes) -> esyn::Res<Self> {
                    let mut res = Self::default();

                    #(
                    res.#idx = <#ty as esyn::Bytes>::from_bytes(&mut buf)?;
                    )*

                    Ok(res)
                }

            }

            });
        }

        DataStruct {
            fields: Fields::Unit,
            ..
        } => {
            todo!()
        }
    }

    Ok(ts_res)
}

pub fn derive_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let enum_ident = &input.ident;
    let generics = bound::add_trait_bounds_enum(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut ts_de = TokenStream::new();
    let mut ts_bytes = TokenStream::new();
    let mut ts_ast = TokenStream::new();
    for var in data.variants.iter() {
        let ident = &var.ident;
        let fields = &var.fields;

        let name: &Vec<_> = &fields.iter().map(|f| &f.ident).collect();
        let ty: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();

        let match_name = format!("{}::{}", enum_ident.to_string(), ident.to_string());

        match fields {
            Fields::Unit => {
                ts_de.extend(quote! {
                    stringify!(#ident) => {
                        return Ok(Self::#ident);
                    },
                });

                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#ident;
                    },
                });
            }

            Fields::Unnamed(_) => {
                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#ident (
                        #(
                        <#ty as esyn::Bytes>::from_bytes(&mut buf)?,
                        )*
                        );
                    },
                });
            }

            Fields::Named(_) => {
                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#ident {
                        #(
                        #name: <#ty as esyn::Bytes>::from_bytes(&mut buf)?,
                        )*
                        };
                    },
                });
            }
        }
    }

    Ok(quote! {
        impl #impl_generics esyn::Ast
         for #enum_ident #ty_generics
             #where_clause
        {
            fn ast() -> String {
                format!("*\"Enum_{}\"", stringify!(#enum_ident))
            }
        }

        impl #impl_generics esyn::TypeInfo
         for #enum_ident #ty_generics
             #where_clause
        {
            fn name() -> &'static str {
                stringify!(#enum_ident)
            }
        }

        impl #impl_generics esyn::Bytes
         for #enum_ident #ty_generics
             #where_clause
        {
            fn from_bytes(mut buf: impl esyn::ParseBytes) -> esyn::Res<Self> {
                let mut res = Self::default();
                if !buf.read_bool()? {
                    return Ok(res);
                }

                let name = buf.read_string()?;
                match name.as_str() {
                    #ts_bytes
                    _ => unreachable!()
                }

                Ok(res)
            }
        }

    }
    .into())
}
