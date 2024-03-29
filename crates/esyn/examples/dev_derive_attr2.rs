use esyn::*;

fn main() {
    let config = r#"
fn main() {
    let key = m!(RCtrl-Alt-LShift-A);
}
"#;

    let key = &EsynBuilder::new()
        .set_fn("main")
        .set_let("key")
        .get_once::<Key>(config)
        .unwrap();
    dbg!(key);
}

fn test_enum_attr_custom_syntax() {
    #[derive(Debug, EsynDe, EsynSer, Default)]
    #[custom_syntax]
    enum Enum {
        Named {
            a: u8,
            b: u8,
        },

        #[default]
        Unknown,
    }

    let config = r#"
fn main() {
    let val = m!(1:2);
}
"#;

    let val = &EsynBuilder::new()
        .set_fn("main")
        .set_let("val")
        .get_once::<Enum>(config)
        .unwrap();
    dbg!(val);
}

#[derive(Debug, EsynDe, EsynSer, Default)]
#[custom_syntax]
pub struct Key<Opt = Option<Ch>> {
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
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Ctrl,
    LCtrl,
    RCtrl,
    //LRCtrl,
    Alt,
    LAlt,
    RAlt,
    //LRAlt,
    Shift,
    LShift,
    RShift,
    //LRShift,
    #[default]
    Unknown,
}

impl<Opt> CustomSyntax for Key<Opt> {
    fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
        use syn::*;

        struct Tmp {
            inner: Punctuated<Ident, Token![-]>,
        }

        impl Parse for Tmp {
            fn parse(input: parse::ParseStream) -> Result<Self> {
                Ok(Self {
                    inner: input.parse_terminated(Ident::parse, Token![-])?,
                })
            }
        }

        let input: Tmp = parse2(expr.mac.tokens.clone())?;

        let mut key = Key::default();
        for f in input.inner.iter() {
            match f.to_string().to_uppercase().as_str() {
                "CTRL" => key.ctrl = Some(Ch::Ctrl),
                "LCTRL" => key.ctrl = Some(Ch::LCtrl),
                "RCTRL" => key.ctrl = Some(Ch::RCtrl),

                "ALT" => key.alt = Some(Ch::Alt),
                "LALT" => key.alt = Some(Ch::LAlt),
                "RALT" => key.alt = Some(Ch::RAlt),

                "SHIFT" => key.shift = Some(Ch::Shift),
                "LSHIFT" => key.shift = Some(Ch::LShift),
                "RSHIFT" => key.shift = Some(Ch::RShift),

                "A" => key.ch = Ch::A,
                "B" => key.ch = Ch::B,
                "C" => key.ch = Ch::C,
                "D" => key.ch = Ch::D,
                "E" => key.ch = Ch::E,
                "F" => key.ch = Ch::F,
                "G" => key.ch = Ch::G,
                "H" => key.ch = Ch::H,
                "I" => key.ch = Ch::I,
                "J" => key.ch = Ch::J,
                "K" => key.ch = Ch::K,
                "L" => key.ch = Ch::L,
                "M" => key.ch = Ch::M,
                "N" => key.ch = Ch::N,
                "O" => key.ch = Ch::O,
                "P" => key.ch = Ch::P,
                "Q" => key.ch = Ch::Q,
                "R" => key.ch = Ch::R,
                "S" => key.ch = Ch::S,
                "T" => key.ch = Ch::T,
                "U" => key.ch = Ch::U,
                "V" => key.ch = Ch::V,
                "W" => key.ch = Ch::W,
                "X" => key.ch = Ch::X,
                "Y" => key.ch = Ch::Y,
                "Z" => key.ch = Ch::Z,

                _ => unreachable!(),
            }
        }

        Ok(key.into_expr())
    }
}
