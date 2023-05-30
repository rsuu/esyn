use syn::{parse_quote, GenericParam, Generics};

pub fn add_trait_bounds_struct(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Sized));
            type_param.bounds.push(parse_quote!(Default));
            type_param.bounds.push(parse_quote!(esyn::Bytes));
            type_param.bounds.push(parse_quote!(esyn::Ast));
        }
    }
    generics
}

pub fn add_trait_bounds_enum(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Sized));
            type_param.bounds.push(parse_quote!(Default));
            type_param.bounds.push(parse_quote!(esyn::Bytes));
            type_param.bounds.push(parse_quote!(esyn::EnumName));
            type_param.bounds.push(parse_quote!(esyn::Ast));
        }
    }

    generics
}
