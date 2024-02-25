use crate::*;

pub trait CustomSyntax {
    fn parse(expr: &syn::Expr) -> Res<syn::Expr> {
        match expr {
            syn::Expr::Macro(expr) => Self::parse_macro(expr),
            syn::Expr::Call(expr) => Self::parse_call(expr),
            _ => Err(MyErr::Unimplemented),
        }
    }

    fn parse_macro(_: &syn::ExprMacro) -> Res<syn::Expr> {
        Err(MyErr::Unimplemented)
    }

    fn parse_call(_: &syn::ExprCall) -> Res<syn::Expr> {
        Err(MyErr::Unimplemented)
    }
}
