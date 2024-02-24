// ? set_let
//   set_let_mut
//   alias_map: hashmap
//   FnBlock -> Scope
//   ders: gen_parten_match(att args)

mod auto;
mod error;
mod ext;
mod parser;
mod visit;

pub mod utils;
pub mod __quote {
    pub use quote::*;
}

pub use {
    // trait
    auto::*,

    // macro
    esyn_derive::{EsynDe, EsynSer},

    // struct/enum
    {
        error::{MyErr, Res},
        ext::ByteStr,
        parser::{Esyn, EsynBuilder, FnBlock},
        utils::ExprHelper,
    },
};

// extern-crate
pub use {
    proc_macro2::{self, TokenStream},
    quote::{quote, ToTokens},
    syn::{self, punctuated::Punctuated, visit::Visit},
};
