use esyn::{proc_macro2::TokenTree, *};

fn main() {
    let config = r#"
fn main() {
    let key = Key {
        inner: parse_key!(RCtrl-Alt-LShift-A)
    };
}
"#;

    let key = &EsynBuilder::new()
        .set_fn("main")
        .set_let("key")
        .get_once::<Key>(config)
        .unwrap();
    dbg!(key);
}

fn parse_key(expr: &syn::ExprMacro) -> Res<syn::Expr> {
    let ts = expr.mac.tokens.clone().into_iter();

    let mut inner = KeyInner::default();
    for f in ts {
        match f {
            TokenTree::Ident(ident) => match ident.to_string().as_str() {
                "A" => inner.ch = Ch::A,
                "B" => inner.ch = Ch::B,
                "RCtrl" => inner.ctrl = Some(Ch::LCtrl),
                "Alt" => inner.alt = Some(Ch::Alt),
                "LShift" => inner.shift = Some(Ch::LShift),
                _ => {}
            },
            TokenTree::Punct(..) => {}
            _ => {}
        }
    }

    let inner = Wrap(inner);
    let expr = syn::parse_quote!( #inner );

    Ok(expr)
}

#[derive(Debug, EsynDe, EsynSer, Default)]
pub struct Key {
    #[parse(
        Expr::Macro => parse_key(),
    )]
    inner: KeyInner,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
pub struct KeyInner<Opt = Option<Ch>> {
    ch: Ch,
    ctrl: Opt,
    alt: Opt,
    shift: Opt,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
pub enum Ch {
    A,
    B,
    C,

    Ctrl,
    LCtrl,
    RCtrl,

    Alt,
    LAlt,
    RAlt,

    Shift,
    LShift,
    RShift,

    #[default]
    Unknown,
}
