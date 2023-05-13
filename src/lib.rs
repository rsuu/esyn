pub mod auto;
pub mod error;
pub mod parse;

// trait
pub use {
    auto::Zeroed,
    parse::{EsynBytes, FromEsyn, ParseBytes, ParseExpr},
};

// macro
pub use esyn_derive::{AutoDefault, Esyn};

pub use error::{MyErr, Res};
