use syn::*;

pub fn attr_custom_syntax(attrs: &[Attribute]) -> Option<&Attribute> {
    for attr in attrs {
        let Some(ident) = attr.meta.path().get_ident() else {
            continue;
        };

        if ident == "custom_syntax" {
            return Some(attr);
        }
    }

    None
}
