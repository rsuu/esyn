use crate::*;
use byteorder::LE;

//==================
pub trait TypeInfo {
    fn name() -> &'static str;
}

//==================
pub trait Ast {
    fn ast() -> String;
}

macro_rules! impl_Ast_for {
     ( $($t:ty)* ) => {$(
         impl Ast for $t {
            fn ast() -> String {
                format!("*\"Std\"")
            }
         }
     )*};
}

macro_rules! impl_Ast_for_tuple {
    ( $($name:ident),+ ) => {
        impl< $($name:Ast),+ > Ast for ($($name,)+) {
            fn ast() -> String {
                format!("*\"Tuple\"")
            }
        }
    };
}

impl_Ast_for! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
    bool
    char String
}

impl_Ast_for_tuple!(T0);
impl_Ast_for_tuple!(T0, T1);
impl_Ast_for_tuple!(T0, T1, T2);
impl_Ast_for_tuple!(T0, T1, T2, T3);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_Ast_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);

impl<T: Ast> Ast for Option<T> {
    fn ast() -> String {
        format!("*\"Option\"")
    }
}

impl<T: Ast> Ast for Vec<T> {
    fn ast() -> String {
        format!("*\"Vec\"")
    }
}

impl<T: Ast> Ast for Box<T> {
    fn ast() -> String {
        format!("*\"Box\"")
    }
}

//==================
macro_rules! impl_Bytes_for {
    ( $($t:ty)* ) => {$(
        impl Bytes for $t {
            fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
                Ok(read_int(&mut buf)? as $t)
            }
        }
    )*};
}

macro_rules! impl_Bytes_for_tuple {
    ( $($t:ident),+ ) => {
        impl< $($t: Bytes),+ > Bytes
            for ( $($t,)+ )
        {
            fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
                Ok(
                ($(
                <$t as Bytes>::from_bytes(&mut buf)?,
                )+)
                )
            }
        }
    };
}

//==============
impl_Bytes_for! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
}

impl_Bytes_for_tuple!(T0);
impl_Bytes_for_tuple!(T0, T1);
impl_Bytes_for_tuple!(T0, T1, T2);
impl_Bytes_for_tuple!(T0, T1, T2, T3);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_Bytes_for_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);

impl Bytes for f32 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        if buf.read_u8()? == 1 {
            Ok(buf.read_f64::<LE>()? as f32)
        } else {
            Ok(Self::default())
        }
    }
}

impl Bytes for f64 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        if buf.read_u8()? == 1 {
            Ok(buf.read_f64::<LE>()?)
        } else {
            Ok(Self::default())
        }
    }
}

impl Bytes for bool {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        buf.read_bool()
    }
}

impl Bytes for char {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        if buf.read_u8()? == 1 {
            buf.read_char()
        } else {
            Ok(char::default())
        }
    }
}

impl Bytes for String {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        if buf.read_u8()? == 1 {
            buf.read_string()
        } else {
            Ok(Self::default())
        }
    }
}

impl<T> Bytes for Option<T>
where
    T: Bytes,
{
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        if !buf.read_bool()? {
            return Ok(None);
        }

        let name = buf.read_string()?;
        assert_eq!(&name, "Some");

        Ok(Some(T::from_bytes(buf)?))
    }
}

impl<T> Bytes for Vec<T>
where
    T: Bytes,
{
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        let len = buf.read_u32::<LE>()? as usize;

        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(T::from_bytes(&mut buf)?);
        }

        Ok(res)
    }
}

// TODO: ?remove
impl<T> Bytes for Box<T>
where
    T: Bytes,
{
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        Ok(Box::new(T::from_bytes(&mut buf)?))
    }
}
