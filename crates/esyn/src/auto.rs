use crate::*;
use byteorder::LE;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

//==================
pub trait TypeInfo {
    // return struct/enum ident.
    fn name() -> &'static str;
}

pub trait FmtEnum {
    // TODO: match
    //           Self::A => "Self::A"
    fn fmt() -> &'static str;
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

// WARN: Ignore elements in tuple is not allowed.
macro_rules! impl_Ast_for_tuple {
    ( $($name:ident),+ ) => {
        impl< $($name:Ast),+ > Ast
         for ($($name,)*) {
            fn ast() -> String {
                 let mut tmp = "".to_string();

                 $(
                 tmp.push_str(&<$name as Ast>::ast());
                 tmp.push(',');
                 )*

                 //dbg!(&tmp);
                 format!("({})", tmp)
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

impl<K: Ast, V: Ast> Ast for HashMap<K, V> {
    fn ast() -> String {
        format!("*\"HashMap\"")
    }
}

impl<K: Ast, V: Ast> Ast for BTreeMap<K, V> {
    fn ast() -> String {
        format!("*\"BTreeMap\"")
    }
}

impl Ast for Ipv4Addr {
    fn ast() -> String {
        format!("*\"Ipv4Addr\"")
    }
}

impl Ast for Ipv6Addr {
    fn ast() -> String {
        format!("*\"Ipv6Addr\"")
    }
}

impl Ast for IpAddr {
    fn ast() -> String {
        format!("*\"IpAddr\"")
    }
}

//==================
macro_rules! impl_Bytes_for {
    ( $($t:ty)* ) => {$(
        impl Bytes for $t {
            fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
                Ok(read_int(buf)? as $t)
            }
        }
    )*};
}

macro_rules! impl_Bytes_for_tuple {
    ( $($t:ident),+ ) => {
        impl< $($t: Bytes),+ > Bytes
            for ( $($t,)+ )
        {
            fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
                Ok(
                ($(
                <$t as Bytes>::from_bytes(buf)?,
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

impl Bytes for f32 {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if buf.read_bool()? {
            Ok(buf.read_f64::<LE>()? as f32)
        } else {
            Ok(Self::default())
        }
    }
}

impl Bytes for f64 {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if buf.read_bool()? {
            Ok(buf.read_f64::<LE>()?)
        } else {
            Ok(Self::default())
        }
    }
}

impl Bytes for bool {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        buf.read_bool()
    }
}

impl Bytes for char {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if buf.read_bool()? {
            buf.read_char()
        } else {
            Ok(char::default())
        }
    }
}

impl Bytes for String {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if buf.read_bool()? {
            buf.read_string()
        } else {
            Ok(Self::default())
        }
    }
}

impl<T: Bytes> Bytes for Option<T> {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if !buf.read_bool()? {
            return Ok(None);
        }

        let name = buf.read_string()?;
        Ok(match name.as_str() {
            "Some" => Some(T::from_bytes(buf)?),
            "" => None,
            _ => return Err(MyErr::Todo),
        })
    }
}

impl<K, V> Bytes for HashMap<K, V>
where
    K: Bytes + Hash + Eq,
    V: Bytes + Hash + Eq,
{
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        // like Vec
        let len = buf.read_u32::<LE>()? as usize;

        let mut res = Self::with_capacity(len);
        for _ in 0..len {
            res.insert(K::from_bytes(buf)?, V::from_bytes(buf)?);
        }

        Ok(res)
    }
}

impl<T: Bytes> Bytes for Vec<T> {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        let len = buf.read_u32::<LE>()? as usize;

        let mut res = Self::with_capacity(len);
        for _ in 0..len {
            res.push(T::from_bytes(buf)?);
        }

        Ok(res)
    }
}

// TODO: ?remove
impl<T: Bytes> Bytes for Box<T> {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        Ok(Box::new(T::from_bytes(buf)?))
    }
}

impl<K, V> Bytes for BTreeMap<K, V>
where
    K: Bytes + Ord,
    V: Bytes + Ord,
{
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        // like Vec
        let len = buf.read_u32::<LE>()? as usize;

        let mut res = Self::new();
        for _ in 0..len {
            res.insert(K::from_bytes(buf)?, V::from_bytes(buf)?);
        }

        Ok(res)
    }
}

impl Bytes for Ipv4Addr {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        let (a, b, c, d) = <(u8, u8, u8, u8) as Bytes>::from_bytes(buf)?;

        Ok(Self::new(a, b, c, d))
    }
}

impl Bytes for Ipv6Addr {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        let (a, b, c, d, e, f, g, h) =
            <(u16, u16, u16, u16, u16, u16, u16, u16) as Bytes>::from_bytes(buf)?;

        Ok(Self::new(a, b, c, d, e, f, g, h))
    }
}

impl Bytes for IpAddr {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self> {
        if !buf.read_bool()? {
            return Err(MyErr::Todo);
        }

        let name = buf.read_string()?;
        Ok(match name.as_str() {
            "IpAddr::V4" => Self::V4(Ipv4Addr::from_bytes(buf)?),
            "IpAddr::V6" => Self::V6(Ipv6Addr::from_bytes(buf)?),
            _ => return Err(MyErr::Todo),
        })
    }
}
