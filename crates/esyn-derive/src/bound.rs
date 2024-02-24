use syn::{parse_quote, GenericParam, Generics};

// impl ... for ... where $bounds
pub fn de_trait_bounds_struct(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(Sized));
        type_param.bounds.push(parse_quote!(EsynDefault));
        type_param.bounds.push(parse_quote!(MutPath));
        type_param.bounds.push(parse_quote!(EsynSer));
    }

    generics
}

// impl ... for ... where $bounds
pub fn de_trait_bounds_enum(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(Sized));
        type_param.bounds.push(parse_quote!(EsynDefault));
        type_param.bounds.push(parse_quote!(MutPath));
        type_param.bounds.push(parse_quote!(EsynSer));
    }

    generics
}

// impl ... for ... where $bounds
pub fn ser_trait_bounds_struct(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(EsynSer));
    }

    generics
}

// impl ... for ... where $bounds
pub fn ser_trait_bounds_enum(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(EsynSer));
    }

    generics
}

// impl ... for ... where $bounds
pub fn default_add_trait_bounds_struct(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(EsynDefault));
    }

    generics
}

// impl ... for ... where $bounds
pub fn default_add_trait_bounds_enum(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        let GenericParam::Type(ref mut type_param) = *param else {
            continue;
        };

        type_param.bounds.push(parse_quote!(EsynDefault));
    }

    generics
}
