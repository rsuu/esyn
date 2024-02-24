use crate::*;

/// Type::into_tokens()
pub trait EsynSer {
    fn ser(&self) -> TokenStream;
}

macro_rules! impl_EsynSer_for {
    ( $($t:ty)* ) => {$(
        impl EsynSer for $t {
            fn ser(&self) -> TokenStream {
                quote! {
                    #self
                }
            }
        }
    )*}
}

macro_rules! impl_EsynSer_for_tuple {
    ($($ty:ident, $ident:ident),*) => {
        impl<$($ty),*> EsynSer
        for ($($ty,)*)
            where $($ty: EsynSer),*
        {
            fn ser(&self) -> TokenStream {
                let ($($ident,)*) = self;

                $(
                    let $ident = $ident.ser();
                )*

                quote! {
                    (
                      $(
                          #$ident,
                      )*
                    )
                }
            }
        }
    }
}

impl<'a, T> EsynSer for &'a T
where
    T: EsynSer,
{
    fn ser(&self) -> TokenStream {
        let ts = <T as EsynSer>::ser(self);

        quote! {
            #ts
        }
    }
}

impl<'a, T> EsynSer for &'a [T]
where
    T: EsynSer,
{
    fn ser(&self) -> TokenStream {
        let iter = self.iter().map(|v| v.ser());

        quote! {
            [ #(#iter,)* ]
        }
    }
}

impl<T> EsynSer for Vec<T>
where
    T: EsynSer,
{
    fn ser(&self) -> TokenStream {
        let iter = self.iter().map(|v| v.ser());

        quote! {
            [ #(#iter,)* ]
        }
    }
}

impl<T> EsynSer for Option<T>
where
    T: EsynSer,
{
    fn ser(&self) -> TokenStream {
        if let Some(v) = self {
            let ts = v.ser();

            quote! { Some(#ts) }
        } else {
            quote! { None }
        }
    }
}

impl<T> EsynSer for Box<T>
where
    T: EsynSer,
{
    fn ser(&self) -> TokenStream {
        let ts = self.as_ref().ser();

        quote! { #ts }
    }
}

impl_EsynSer_for! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
    bool
    char &str String
}

impl_EsynSer_for_tuple!(A, a);
impl_EsynSer_for_tuple!(A, a, B, b);
impl_EsynSer_for_tuple!(A, a, B, b, C, c);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i, J, j);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i, J, j, K, k);
impl_EsynSer_for_tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i, J, j, K, k, L, l);
