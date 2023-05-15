pub mod auto;
pub mod bytes;
pub mod error;
pub mod ext;
pub mod parser;

// macro
pub use esyn_derive::EsynDe;

// trait
pub use {
    auto::{Ast, TypeInfo},
    bytes::{read_int, Bytes, FromEsyn, Null, ParseBytes, ParseExpr},
};

pub use {
    error::{MyErr, Res},
    parser::Esyn,
};
