use crate::{MyErr, Res, Zeroed};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, LE};
use std::io::{Cursor, Write};
use syn::*;

// step 4. type to struct
pub trait FromEsyn: EsynBytes {
    fn from_esyn(buf: impl ParseBytes, fields: &mut Vec<String>) -> Res<Self>;

    fn from_esyn_default(buf: impl ParseBytes, fields: &mut Vec<String>) -> Res<Self>
    where
        Self: Default;

    // use MaybeUninit and addr_of_mut!()
    unsafe fn from_esyn_uninit(buf: impl ParseBytes, fields: &mut Vec<String>) -> Res<Self>;
}

// step 3. esyn-bytes to type
pub trait EsynBytes: Sized {
    fn from_bytes(buf: impl ParseBytes) -> Res<Self>;
}

// step 2. bytes to esyn-bytes
pub trait ParseBytes: std::io::Read {
    // NOTE: only `1` is true
    fn read_bool(&mut self) -> Res<bool> {
        let flag = self.read_u8()?;

        Ok(if flag == 1 { true } else { false })
    }

    fn read_string(&mut self) -> Res<String> {
        let len = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf);

        Ok(String::from_utf8(buf)?)
    }
}

// step 1. syn/file to bytes
pub trait ParseExpr {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()>;
}

impl<T: std::io::Read> ParseBytes for T {}

impl ParseExpr for Expr {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        //dbg!(self);

        match self {
            // e.g. 123 OR 123_u8
            Self::Lit(v) => v.write(buf)?,

            // e.g. None
            // TODO: enum
            Self::Path(v) => v.write(buf)?,

            // e.g. Some(T)
            Self::Call(v) => v.write(buf)?,

            _ => {}
        }

        Ok(())
    }
}

impl ParseExpr for ExprPath {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        //dbg!(self);

        let ExprPath { path, .. } = self;

        let syn::Path { segments, .. } = path
        else {
            unreachable!()
        };

        let PathSegment { ident, .. } = &segments[0];

        if ident.to_string().as_str() == "None" {
            buf.write_i64::<LE>(0)?;
        }

        Ok(())
    }
}

impl ParseExpr for ExprCall {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        //dbg!(self);

        let Self { func, args, .. } = self;

        let Expr::Path(ExprPath {
                path: syn::Path { segments, .. },
                ..
            }) = &**func
        else { unimplemented!() };

        let PathSegment { ident, .. } = &segments[0];

        if ident.to_string().as_str() != "Some" {
            unimplemented!();
        }

        buf.write_u8(1)?;
        for f in args.iter() {
            f.write(buf)?;
        }

        Ok(())
    }
}

impl ParseExpr for ExprLit {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        let ExprLit { attrs, lit } = self;

        match lit {
            Lit::Int(v) => {
                let v: i64 = v.base10_parse()?;
                buf.write_all(&v.to_le_bytes())?;
            }

            Lit::Str(v) => {
                let v = v.value();
                let len = v.len();

                buf.write_u32::<LE>(len as u32)?;
                buf.write_all(v.as_bytes())?;
            }

            Lit::Float(v) => {
                let v: f64 = v.base10_parse()?;
                buf.write_all(&v.to_le_bytes())?;
            }

            _ => unreachable!(),
        }

        Ok(())
    }
}

// u8: {
//   data: u8
// }
impl EsynBytes for u8 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        Ok(buf.read_i64::<LE>()? as u8)
    }
}

// i8: {
//   data: i8
// }
impl EsynBytes for i8 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        Ok(buf.read_i64::<LE>()? as i8)
    }
}

// f32: {
//   data: f32
// }
impl EsynBytes for f32 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        // WARN: UB
        Ok(buf.read_f64::<LE>()? as f32)
    }
}

// f64: {
//   data: f64
// }
impl EsynBytes for f64 {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        Ok(buf.read_f64::<LE>()?)
    }
}

// String {
//   len:  u32,
//   data: Vec<u8>,
// }
impl EsynBytes for String {
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        buf.read_string()
    }
}

// Option<T>: {
//   is_some: bool, // u8
//   opt: if is_some { data: T }
// }
impl<T> EsynBytes for Option<T>
where
    T: EsynBytes,
{
    fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
        let is_some = buf.read_bool()?;
        if !is_some {
            return Ok(None);
        }

        let res = T::from_bytes(&mut buf)?;

        Ok(Some(res))
    }
}

// Vec<T>: {
//   len: u32,
//   data: [T; len]
// }
impl<T> EsynBytes for Vec<T>
where
    T: EsynBytes,
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
