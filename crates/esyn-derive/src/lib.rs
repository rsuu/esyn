mod de;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Esyn)]
pub fn derive_esyn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    de::derive(input)
}

#[proc_macro_derive(AutoDefault)]
pub fn derive_auto_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    de::derive_auto_default(input)
}
