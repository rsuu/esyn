pub mod auto;
pub mod bytes;
pub mod error;
pub mod ext;
pub mod func;
pub mod parser;

// macro
pub use esyn_derive::EsynDe;

// trait
pub use {
    auto::{Ast, TypeInfo},
    bytes::{read_int, Bytes, ParseBytes, ParseExpr},
};

pub use {
    error::{MyErr, Res},
    func::{Func, FuncMap},
    parser::Esyn,
};
