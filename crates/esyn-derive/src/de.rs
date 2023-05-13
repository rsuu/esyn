use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

// #[derive(Esyn)]
// struct Test {
//     field1: i32,
//     field2: String,
// }
//
// impl Test {
//     fn init_default_values(&mut self) {
//         self.field1 = i32::default();
//         self.field2 = String::default();
//     }
// }
pub fn derive(input: DeriveInput) -> TokenStream {
    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    else {
        panic!("MyMacro only works on structs with named fields");
    };

    let ident = input.ident;
    let name: &Vec<_> = &fields.named.iter().map(|f| &f.ident).collect();
    let ty: &Vec<_> = &fields.named.iter().map(|f| &f.ty).collect();
    let len = name.len();

    //    let mut tmp = vec![];
    //    for f in fields.named.iter() {
    //        tmp.push((f, f.ident.clone().unwrap().to_string()));
    //    }
    //    tmp.sort_by_key(|(_, s)| s.to_string());
    //
    //    let mut name = {
    //        let mut res = vec![];
    //        for (f, ..) in tmp.iter() {
    //            res.push(&f.ident);
    //        }
    //
    //        res
    //    };
    //
    //    let mut ty = {
    //        let mut res = vec![];
    //        for (f, ..) in tmp.iter() {
    //            res.push(&f.ty);
    //        }
    //
    //        res
    //    };

    // iter fields and do `type::default()`
    let res = quote! {
        impl FromEsyn for #ident
        {
            fn from_esyn(
                mut buf: impl ParseBytes,
                fields: &mut Vec<String>
            ) -> Res<Self> {
                if fields.len() < #len {
                    return Err(MyErr::UnEqFields)
                }

                struct Tmp {
                    #( #name: Option<#ty>, )*
                }

                impl Default for Tmp {
                    fn default() -> Self {

                        Self {
                        #( #name: None, )*
                        }
                    }
                }
                let mut tmp = Tmp::default();

                // NOTE: Make sure bytes order as same as fields order.
                for i in 0..fields.len() {
                    #(
                    //dbg!(stringify!(#name), fields[i].as_str());
                    if stringify!(#name) == fields[i].as_str()
                    {
                        tmp.#name = Some(<#ty as EsynBytes>::from_bytes(&mut buf)?);

                        // This mean bytes has been parse.
                        fields[i] = "".to_string();
                    }
                    )*
                }

                // TODO: check if depth > 1
                let res = Self {
                    #(
                    #name: {
                        if let Some(v)
                            = tmp.#name { v }
                        else {
                            return Err(MyErr::Missed( stringify!(#name).to_string() ));
                        }
                    },
                    )*
                };


                Ok(res)
            }
            fn from_esyn_default(
                mut buf: impl ParseBytes,
                fields: &mut Vec<String>
            ) -> Res<Self> {
                let mut res = Self::default();

                // NOTE: The byte order must be the same as the field order.
                for i in 0..fields.len() {
                    #(
                    //dbg!(stringify!(#name), fields[i].as_str());
                    if stringify!(#name) == fields[i].as_str()
                    {
                        res.#name = <#ty as EsynBytes>::from_bytes(&mut buf)?;

                        // NOTE: This means that the bytes have been parsed.
                        fields[i] = "".to_string();
                    }
                    )*
                }


                Ok(res)
            }

            unsafe fn from_esyn_uninit(
                mut buf: impl ParseBytes,
                fields: &mut Vec<String>
            ) -> Res<Self> {
                let mut uninit = std::mem::MaybeUninit::<Self>::uninit();
                let ptr = uninit.as_mut_ptr();


                for i in 0..fields.len() {
                    #(
                    //dbg!(stringify!(#name), fields[i].as_str());
                    if stringify!(#name) == fields[i].as_str()
                    {
                        std::ptr::addr_of_mut!((*ptr).#name)
                            .write(<#ty as EsynBytes>::from_bytes(&mut buf)?);

                        fields[i] = "".to_string();
                    }
                    )*
                }

                let res = uninit.assume_init();

                Ok(res)
            }
        }


        impl EsynBytes for #ident
        {
            fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
                Ok(Self {
                    #(
                      #name: <#ty as EsynBytes>::from_bytes(&mut buf)?,
                    )*
                })
            }
        }

    };

    res.into()
}

pub fn derive_auto_default(input: DeriveInput) -> TokenStream {
    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &input.data
    else {
        panic!("MyMacro only works on structs with named fields");
    };

    let ident = input.ident;
    let mut name: &Vec<_> = &fields.named.iter().map(|f| &f.ident).collect();
    let ty: &Vec<_> = &fields.named.iter().map(|f| &f.ty).collect();

    let res = quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    #(
                      #name: <#ty as Default>::default(),
                    )*
                }
            }
        }

    };

    res.into()
}
