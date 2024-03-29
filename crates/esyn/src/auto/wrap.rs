use crate::*;
use std::fmt::Debug;
use syn::*;

// REFS: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
#[repr(transparent)]
pub struct Wrap<T>(T);

impl<T> Wrap<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }

    pub fn get(self) -> T {
        self.0
    }

    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_expr(self) -> syn::Expr
    where
        T: EsynSer,
    {
        syn::parse_quote!( #self )
    }
}

impl<T> ToTokens for Wrap<T>
where
    T: EsynSer,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(<T as EsynSer>::ser(&self.0));
    }
}

impl<T> parse::Parse for Wrap<T>
where
    T: DeRs<Expr>,
{
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;

        Ok(Self(T::de(&expr).unwrap()))
    }
}

impl<T: Debug> Debug for Wrap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
