use crate::*;

use syn::*;

pub struct ExprHelper {}

impl ExprHelper {
    pub fn get_call_name(syn::ExprCall { func, .. }: &syn::ExprCall) -> &syn::Ident {
        let syn::Expr::Path(v) = func.as_ref() else {
            unreachable!()
        };

        v.path.get_ident().unwrap()
    }

    pub fn get_macro_name(syn::ExprMacro { mac, .. }: &syn::ExprMacro) -> &syn::Ident {
        let v = &mac.path;

        &v.segments[0].ident
    }

    pub fn get_named_field_expr<'a>(
        fields: &'a syn::punctuated::Punctuated<FieldValue, Token![,]>,
        name: &'static str,
    ) -> Res<&'a Expr> {
        for FieldValue { expr, member, .. } in fields.iter() {
            let Member::Named(ident) = member else {
                continue;
            };
            //dbg!(&expr);

            if ident == name {
                return Ok(expr);
            } else {
                continue;
            }
        }

        err!(NotFound "{name}")
    }
}
