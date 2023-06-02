use crate::bound;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Index, Result};

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
            let field_name: &Vec<_> = &fields.named.iter().map(|f| &f.ident).collect();
            let field_ty: &Vec<_> = &fields.named.iter().map(|f| &f.ty).collect();

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
                            stringify!(#field_name),
                            <#field_ty as esyn::Ast>::ast()
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
                fn from_bytes<W: esyn::ParseBytes>(buf: &mut W) -> esyn::Res<Self> {
                    let mut res = Self::default();
                    // Struct{}
                    if !buf.read_bool()? {
                        return Ok(Self::default());
                    }

                    #(
                    res.#field_name = <#field_ty as esyn::Bytes>::from_bytes(buf)?;
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
            let field_ty: &Vec<_> = &fields.unnamed.iter().map(|f| &f.ty).collect();
            let idx: Vec<_> = {
                let mut res = vec![];
                for n in 0..fields.unnamed.len() {
                    res.push(Index::from(n));
                }

                res
            };

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
                            <#field_ty as esyn::Ast>::ast()
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
                fn from_bytes<W: esyn::ParseBytes>(buf: &mut W) -> esyn::Res<Self> {
                    let mut res = Self::default();

                    // Struct()
                    if !buf.read_bool()? {
                        return Ok(Self::default());
                    }

                    #(
                    res.#idx = <#field_ty as esyn::Bytes>::from_bytes(buf)?;
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
            ts_res.extend(quote! {
            impl #impl_generics esyn::Ast
             for #struct_ident #ty_generics
                 #where_clause
            {
                fn ast() -> String {
                    stringify!(#struct_ident).to_string()
                }
            }

            impl #impl_generics esyn::Bytes
             for #struct_ident #ty_generics
                 #where_clause
            {
                fn from_bytes<W: esyn::ParseBytes>(buf: &mut W) -> esyn::Res<Self> {
                    // ?
                    buf.read_bool()?;

                    Ok(Self::default())

                }
            }

            });
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
    for var in data.variants.iter() {
        let var_ident = &var.ident;
        let fields = &var.fields;

        let field_name: &Vec<_> = &fields.iter().map(|f| &f.ident).collect();
        let field_ty: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();

        let match_name = format!("{}::{}", enum_ident.to_string(), var_ident.to_string());

        match fields {
            Fields::Unit => {
                ts_de.extend(quote! {
                    stringify!(#var_ident) => {
                        return Ok(Self::#var_ident);
                    },
                });

                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#var_ident;
                    },
                });
            }

            Fields::Unnamed(_) => {
                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#var_ident (
                        #(
                        <#field_ty as esyn::Bytes>::from_bytes(buf)?,
                        )*
                        );
                    },
                });
            }

            Fields::Named(_) => {
                ts_bytes.extend(quote! {
                    #match_name => {
                        res = Self::#var_ident {
                        #(
                        #field_name: <#field_ty as esyn::Bytes>::from_bytes(buf)?,
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
            fn from_bytes<W: esyn::ParseBytes>(buf: &mut W) -> esyn::Res<Self> {
                let mut res = Self::default();

                // not Struct
                if !buf.read_bool()? {
                    return Ok(res);
                }

                let name = buf.read_string()?;
                if name.is_empty() {
                    return Ok(res);
                }

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
