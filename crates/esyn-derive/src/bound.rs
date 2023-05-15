use proc_macro2::TokenStream;
use syn::{parse_quote, GenericParam, Generics, WhereClause, WherePredicate};

pub fn where_clause_with_bound(generics: &Generics, bound: TokenStream) -> WhereClause {
    let new_predicates = generics.type_params().map::<WherePredicate, _>(|param| {
        let param = &param.ident;
        parse_quote!(#param : #bound)
    });

    let mut generics = generics.clone();
    generics
        .make_where_clause()
        .predicates
        .extend(new_predicates);
    generics.where_clause.unwrap()
}

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
