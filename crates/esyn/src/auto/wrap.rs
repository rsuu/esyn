use crate::*;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Deref, DerefMut},
};
use syn::*;

// REFS: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
#[repr(transparent)]
pub struct Wrap<T>(pub T);

impl<T> Wrap<T>
where
    T: EsynSer,
{
    pub fn get(self) -> T {
        self.0
    }

    pub fn get_ref(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn to_expr(self) -> syn::Expr {
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

impl<T> AsRef<T> for Wrap<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Wrap<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Wrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Wrap<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Display> Display for Wrap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for Wrap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: PartialEq> PartialEq<T> for Wrap<T> {
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl<T: Hash> Hash for Wrap<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
