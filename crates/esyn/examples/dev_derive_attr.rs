use esyn::*;

fn main() {
    let config = r#"
fn main() {
    let a = Test {
        _u8: 8,       // Rust's syntax
        _u16: parse_call_u16(),    // Function
        _u32: parse_macro!(32), // Macro
    };
    a.inner = Inner {
        _u16: parse_call_u16_inner(),    // Function
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let v = &EsynBuilder::new()
        .set_fn("main")
        .set_let("a")
        .get::<Test>(&esyn)
        .unwrap();
    dbg!(v);
}

#[derive(Debug, EsynDe, EsynSer, Default)]
struct Test {
    // parse:
    //   123
    _u8: u8,

    // parse:
    //   123  |  f()
    #[parse(
        Expr::Call => parse_call_u16(),
    )]
    _u16: u16,

    // parse:
    //   123  |  f()  |  m!(32)  |  m![32]  |  m!{32}
    #[parse(
        // case 1
        Expr::Call => parse_call_u32(),
        // case 2
        Expr::Macro => parse_macro(),
    )]
    _u32: u32,

    inner: Inner,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
struct Inner {
    #[parse(
        Expr::Call => parse_call_u16_inner(),
    )]
    _u16: u16,
}

// u16

// fn
pub fn parse_call_u16(expr: &syn::ExprCall) -> Res<syn::Expr> {
    Ok(syn::parse2(quote! {16})?)
}

pub fn parse_call_u16_inner(expr: &syn::ExprCall) -> Res<syn::Expr> {
    Ok(syn::parse2(quote! {16})?)
}

// u32

// fn
pub fn parse_call_u32(expr: &syn::ExprCall) -> Res<syn::Expr> {
    Ok(syn::parse2(quote! {32})?)
}

// macro
// `m!{ 32 }` to u32
//      ^^ LitInt
pub fn parse_macro(expr: &syn::ExprMacro) -> Res<syn::Expr> {
    dbg!(&expr);
    let syn::ExprMacro { mac, attrs } = expr else {
        unreachable!()
    };

    Ok(syn::parse2(mac.tokens.to_owned())?)
}
