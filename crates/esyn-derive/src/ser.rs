use crate::bound;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Index, Result};

pub fn derive_ser(input: &DeriveInput) -> Result<TokenStream> {
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
    let generics = bound::ser_trait_bounds_struct(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let struct_body_unnamed: TokenStream = r#"
quote! {
    #struct_ident(
        #(
            #iter,
        )*
    )
}
"#
    .parse()
    .unwrap();

    let struct_body_named: TokenStream = r#"
quote! {
    #struct_ident {
        #(
            #iter_name: #iter_field,
        )*
    }
}
"#
    .parse()
    .unwrap();

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
            //             #[rustfmt::skip]
            struct_impl.extend(quote! {
                impl #impl_generics
                    esyn::EsynSer
                for #struct_ident #ty_generics
                    #where_clause
                {
                    fn ser(&self) -> TokenStream {
                        let struct_ident = quote!{ #struct_ident };
                        let iter_name = [
                            #(
                                quote!{ #field_name },
                            )*
                        ];
                        let iter_field = [
                            #(
                                <#field_ty as esyn::EsynSer>::ser(&self.#field_name),
                            )*
                        ];

                        #struct_body_named
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

            // impl struct unamed
            //             #[rustfmt::skip]
            struct_impl.extend(quote! {
            impl #impl_generics
                esyn::EsynSer
            for #struct_ident #ty_generics
                #where_clause
            {
                fn ser(&self) -> TokenStream {
                    let struct_ident = quote!{ #struct_ident };
                    let iter: &[TokenStream] = &[
                        #(
                           <#field_ty as esyn::EsynSer>::ser(&self.#idx),
                        )*
                    ];

                    #struct_body_unnamed
                }
            }

            });
        }

        DataStruct {
            fields: Fields::Unit,
            ..
        } => {
            // impl struct unit
            //             #[rustfmt::skip]
            struct_impl.extend(quote! {
            impl #impl_generics
                esyn::EsynSer
            for #struct_ident #ty_generics
                #where_clause
            {
                fn ser(&self) -> TokenStream {
                    quote!{ #struct_ident }
                }
            }

            });
        }
    }

    Ok(struct_impl)
}

fn derive_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let enum_ident = &input.ident;
    let generics = bound::ser_trait_bounds_enum(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let enum_body_unnamed: TokenStream = r#"
quote! {
    #var_ident(
        #(
            #iter
        )*
    )

}
"#
    .parse()
    .unwrap();

    let enum_body_named: TokenStream = r#"
quote! {
    #var_ident {
        #(
            #iter_name: #iter,
        )*
    }
}
"#
    .parse()
    .unwrap();

    let mut enum_impl = TokenStream::new();
    let _enum_match = TokenStream::new();
    let _enum_default = TokenStream::new();
    let mut enum_ser = TokenStream::new();
    for var in data.variants.iter() {
        let var_ident = &var.ident;
        let fields = &var.fields;

        //let field_name: &Vec<_> = &fields.iter().map(|f| &f.ident).collect();
        let field_ty: &Vec<_> = &fields.iter().map(|f| &f.ty).collect();

        let _match_name = format!("{}::{}", enum_ident, var_ident);

        match fields {
            // impl enum unit
            Fields::Unit => {
                //                 #[rustfmt::skip]
                enum_ser.extend(quote! {
                // e.g. "Unit1" => Self::Unit1,
                #enum_ident::#var_ident => { quote!{ #enum_ident::#var_ident } },
                });
            }

            // impl enum unnamed
            Fields::Unnamed(_) => {
                // generate names for unnamed field
                // e.g.
                //     Enum(u8 , u16)
                //          __0, __1
                let mut named_idx: Vec<TokenStream> = Vec::with_capacity(fields.len());
                for n in 0..fields.len() {
                    named_idx.push(format!("__{n}").parse().unwrap());
                }

                //                 #[rustfmt::skip]
                enum_ser.extend(quote! {
                #enum_ident::#var_ident(
                    #(#named_idx ,)*
                ) => {
                    let var_ident = quote! {
                        #enum_ident::#var_ident
                    };
                    let iter = &[
                    #(
                        <#field_ty as esyn::EsynSer>::ser(&#named_idx),
                    )*
                    ];

                    quote! { #enum_body_unnamed }
                },

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

                //                 #[rustfmt::skip]
                enum_ser.extend(quote! {
                #enum_ident::#var_ident { #(#field_name ,)* } => {
                    let var_ident = quote!{ #enum_ident::#var_ident };
                    let iter_name = [
                        #(
                            quote!{ #field_name },
                        )*
                    ];
                    let iter = [
                        #(
                            <#field_ty as esyn::EsynSer>::ser(&#field_name),
                        )*
                    ];

                    quote!{ #enum_body_named }
                },
                });
            }
        }
    }

    // impl enum
    //     #[rustfmt::skip]
    enum_impl.extend(quote! {
        impl #impl_generics
            esyn::EsynSer
        for #enum_ident #ty_generics
            #where_clause
        {
            fn ser(&self) -> TokenStream {
                match self {
                    #enum_ser
                }
            }
        }
    });

    Ok(enum_impl)
}
