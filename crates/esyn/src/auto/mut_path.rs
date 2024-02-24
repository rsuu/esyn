use crate::*;
use syn::*;

pub trait MutPath: DeRs<Expr> {
    fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()>;
}

macro_rules! impl_MutPath_for {
    ( $($t:ty)* ) => {$(
impl MutPath for $t {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}
    )*};
}

macro_rules! impl_MutPath_for_tuple {
    ( $($t:ident),+ ) => {
impl< $($t: MutPath),+ > MutPath for ( $($t,)+ ) {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        unimplemented!()
    }
}
    }
}

impl_MutPath_for! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
    bool
    char String
}

impl_MutPath_for_tuple!(A);
impl_MutPath_for_tuple!(A, B);
impl_MutPath_for_tuple!(A, B, C);
impl_MutPath_for_tuple!(A, B, C, D);
impl_MutPath_for_tuple!(A, B, C, D, E);
impl_MutPath_for_tuple!(A, B, C, D, E, F);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G, H);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_MutPath_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl MutPath for () {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        Ok(())
    }
}

impl<T: MutPath> MutPath for Box<T> {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}

impl<T: MutPath> MutPath for Option<T> {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}

impl<T: MutPath> MutPath for Vec<T> {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        //TODO: vec[0]
        match ast {
            _ => {
                dbg!(&ast);
            }
        }

        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}

impl<T: MutPath + Copy, const N: usize> MutPath for [T; N] {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        //TODO: array[0]
        match ast {
            _ => {
                dbg!(&ast);
            }
        }

        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}
