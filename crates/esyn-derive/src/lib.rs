mod bound;
mod de;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EsynDe)]
pub fn derive_de(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    de::derive_de(input).unwrap().into()
}
