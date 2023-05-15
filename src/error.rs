use std::string::FromUtf8Error;
use thiserror::Error;

pub type Res<T> = Result<T, MyErr>;

#[derive(Debug, Error)]
pub enum MyErr {
    #[error("todo")]
    Todo,

    #[error("")]
    Str(&'static str),

    #[error("")]
    Debug(String),

    #[error("parse int")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("parse int")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("")]
    Io(#[from] std::io::Error),

    #[error("")]
    Utf8(#[from] FromUtf8Error),

    #[error("")]
    UnEqFields,

    #[error("")]
    Syn(#[from] syn::Error),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
}
