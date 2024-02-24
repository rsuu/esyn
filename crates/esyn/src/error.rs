use thiserror::Error;

#[macro_export]
macro_rules! err {
    (NotFound: $a:expr) => {
        MyErr::NotFound(format!($a))
    };
    (Panic: $a:expr) => {
        MyErr::Panic(format!($a))
    };
    (Expected: $a:expr, $b:expr) => {
        MyErr::Expected {
            l: format!("{}", $a),
            r: format!("{}", $b),
        }
    };

    (NotFound $a:expr) => {
        Err(err!(NotFound: $a))
    };
    (Panic $a:expr) => {
        Err(err!(Panic: $a))
    };
    (Expected $a:expr, $b:expr) => {
        Err(err!(Expected: $a, $b))
    };
}

pub type Res<T> = Result<T, MyErr>;

// TODO:
#[derive(Debug, Error)]
pub enum MyErr {
    #[error("todo")]
    Todo,

    #[error("")]
    Str(&'static str),

    #[error("")]
    Debug(String),

    #[error("not found `{0}`")]
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

    #[error("{0:?}")]
    Panic(String),

    #[error("expected `{l}`, found `{r}`")]
    Expected { l: String, r: String },

    #[error("UnknownField")]
    UnknownField,
}
