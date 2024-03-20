use esyn::*;

fn main() {
    let config = r#"
fn main() {
    let test = Test {
        rust: Rust { a: 0 }, // Rust's syntax
        func: f(1),          // Function
        macros: m!(2),       // Macro
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let test = &EsynBuilder::new()
        .set_fn("main")
        .set_let("test")
        .get::<Test>(&esyn)
        .unwrap();
    dbg!(test);
}

// follows code are all valid now:
//   123  |  f()  |  m!(123)  |  m![123]  |  m!{123}
#[derive(Debug, EsynDe, EsynSer, Default)]
struct Test {
    rust: Rust,
    func: Func,
    macros: Macros,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
struct Rust {
    a: u8,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
#[custom_syntax]
struct Func {
    a: u8,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
#[custom_syntax]
struct Macros {
    a: u8,
}

impl CustomSyntax for Func {
    fn parse_call(expr: &syn::ExprCall) -> Res<syn::Expr> {
        let v = expr
            .args
            .first()
            .unwrap()
            .into_token_stream()
            .to_string()
            .parse::<u8>()?;

        Ok(syn::parse_quote!(Func { a: #v }))
    }
}

impl CustomSyntax for Macros {
    fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
        let v = expr.mac.tokens.to_string().parse::<u8>()?;

        Ok(syn::parse_quote!(Macros { a: #v }))
    }
}
