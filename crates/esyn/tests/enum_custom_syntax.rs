use esyn::*;

#[test]
fn test_named() {
    let config = r#"
fn main() {
    let test = m!(1);
}
"#;

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get_once::<Enum>(config)
        .unwrap();

    assert_eq!(test.get_ref(), &Enum::Named { a: 1 });
}

#[test]
fn test_unnamed() {
    let config = r#"
fn main() {
    let test = m!(2);
}
"#;

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get_once::<Enum>(config)
        .unwrap();

    assert_eq!(test.get_ref(), &Enum::Unnamed(2));
}

#[test]
fn test_unit() {
    let config = r#"
fn main() {
    let test = m!(3);
}
"#;

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get_once::<Enum>(config)
        .unwrap();

    assert_eq!(test.get_ref(), &Enum::Unit);
}

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
#[custom_syntax]
enum Enum {
    Named {
        a: u8,
    },

    Unnamed(u8),

    #[default]
    Unit,
}

impl CustomSyntax for Enum {
    fn parse_macro(ast: &syn::ExprMacro) -> Res<syn::Expr> {
        let v = ast.mac.tokens.to_string().parse::<u8>().unwrap();

        Ok(match v {
            1 => Self::Named { a: v },
            2 => Self::Unnamed(v),
            3 => Self::Unit,
            _ => unreachable!(),
        }
        .into_expr())
    }
}
