pub trait EsynDefault {
    fn esyn_default() -> Self;
}

macro_rules! impl_EsynDefault_for {
    ( $($t:ty)* ) => {$(
impl EsynDefault for $t {
    fn esyn_default() -> Self {
        Self::default()
    }
}
    )*}
}

macro_rules! impl_EsynDefault_for_tuple {
    ( $($t:ident),+ ) => {
impl< $($t: EsynDefault),+ > EsynDefault for ( $($t,)+ ) {
    fn esyn_default() -> Self {
        ($(
            <$t as EsynDefault>::esyn_default(),
        )+)
    }
}
    }
}

impl_EsynDefault_for! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
    bool
    char &str String
}

impl_EsynDefault_for_tuple!(T0);
impl_EsynDefault_for_tuple!(T0, T1);
impl_EsynDefault_for_tuple!(T0, T1, T2);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_EsynDefault_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);

impl EsynDefault for () {
    fn esyn_default() -> Self {}
}

impl<T: EsynDefault + Copy, const N: usize> EsynDefault for [T; N] {
    fn esyn_default() -> Self {
        [T::esyn_default(); N]
    }
}

impl<T: EsynDefault> EsynDefault for Vec<T> {
    fn esyn_default() -> Self {
        Default::default()
    }
}

impl<T: EsynDefault> EsynDefault for Option<T> {
    fn esyn_default() -> Self {
        None
    }
}

impl<T: EsynDefault> EsynDefault for Box<T> {
    fn esyn_default() -> Self {
        Box::new(T::esyn_default())
    }
}
