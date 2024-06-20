use esyn::*;

// valid syntax:
//   123  |  f()  |  m!(123)  |  m![123]  |  m!{123}
#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
struct Named {
    rust: Rust,
    func: Func,
    macros: Macros,
}

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
struct Rust {
    a: u8,
}

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
#[custom_syntax]
struct Func {
    a: u8,
}

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
#[custom_syntax]
struct Macros {
    a: u8,
}

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
#[custom_syntax]
struct Unnamed(u8);

#[derive(Debug, EsynDe, EsynSer, Default, PartialEq)]
#[custom_syntax]
struct Unit;

#[test]
fn test_named() {
    let config = r#"
fn main() {
    let test = Named {
        rust: Rust { a: 0 }, // Rust's syntax
        func: f(1),          // Function
        macros: m!(2),       // Macro
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get::<Named>(&esyn)
        .unwrap();

    assert_eq!(
        test.get_ref(),
        &Named {
            rust: Rust { a: 0 },
            func: Func { a: 1 },
            macros: Macros { a: 2 }
        }
    )
}

#[test]
fn test_unnamed() {
    let config = r#"
fn main() {
    let test = m!( 123 );
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get::<Unnamed>(&esyn)
        .unwrap();

    assert_eq!(test.get_ref(), &Unnamed(123));
}

#[test]
fn test_unit() {
    let config = r#"
fn main() {
    let test = m!( );
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let test = &Esyn::builder()
        .set_fn("main")
        .set_let("test")
        .get::<Unit>(&esyn)
        .unwrap();

    assert_eq!(test.get_ref(), &Unit);
}

impl CustomSyntax for Func {
    fn parse_call(expr: &syn::ExprCall) -> Res<syn::Expr> {
        let a = expr
            .args
            .first()
            .unwrap()
            .into_token_stream()
            .to_string()
            .parse::<u8>()?;

        Ok(Func { a }.into_expr())
    }
}

impl CustomSyntax for Macros {
    fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
        let a = expr.mac.tokens.to_string().parse::<u8>()?;

        Ok(Self { a }.into_expr())
    }
}

impl CustomSyntax for Unnamed {
    fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
        let a = expr.mac.tokens.to_string().parse::<u8>()?;

        Ok(Self(a).into_expr())
    }
}

impl CustomSyntax for Unit {
    fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
        assert!(expr.mac.tokens.to_string().is_empty());

        Ok(Self.into_expr())
    }
}
