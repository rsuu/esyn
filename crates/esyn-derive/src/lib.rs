mod attr;
mod bound;
mod default;
mod ders;
mod ders_struct_named;
mod ders_struct_unit;
mod ders_struct_unnamed;
mod ser;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EsynDe, attributes(parse))]
pub fn derive_de(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut res = TokenStream::new();

    res.extend(ders::derive_de(&input).unwrap());
    res.extend(default::derive_default(&input).unwrap());

    res.into()
}

#[proc_macro_derive(EsynSer)]
pub fn derive_ser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ser::derive_ser(&input).unwrap().into()
}
