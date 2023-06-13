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

    #[error("not found {0}")]
    NotFound(String),

    #[error("parse int")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("parse int")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("unsupported")]
    Unsupported,

    #[error("{0:?} is unsupported")]
    UnType(String),

    #[error("")]
    Io(#[from] std::io::Error),

    #[error("")]
    UnEqFields,

    #[error("")]
    Syn(#[from] syn::Error),
}
