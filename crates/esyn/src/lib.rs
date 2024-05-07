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

pub use {
    auto::{
        custom_syntax::{CustomSyntax, WrapExpr},
        default::EsynDefault,
        ders::DeRs,
        mut_path::MutPath,
        ser::EsynSer,
        wrap::Wrap,
    },
    error::{MyErr, Res},
    ext::ByteStr,
    parser::{Esyn, EsynBuilder, FnBlock},
    utils::ExprHelper,
    {
        // macro
        esyn_derive::{EsynDe, EsynSer},
    },
};

// extern-crate
pub use {
    proc_macro2::{self, TokenStream},
    quote::{self as esyn_quote, quote, ToTokens},
    syn::{self, parse::Parse, punctuated::Punctuated, visit::Visit, Expr},
};
