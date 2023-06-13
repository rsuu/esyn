use crate::Res;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{Read, Write};
use syn::*;

// 1/3. syn/file -> bytes
pub trait ParseExpr {
    fn write<W: Write>(&self, w: &mut W) -> Res<()>;
}

// 2/3. bytes -> type
pub trait ParseBytes: Read + ReadBytesExt + ReadBytesExt {
    fn read_bool(&mut self) -> Res<bool> {
        Ok(self.read_u8()? != 0)
    }

    fn read_string(&mut self) -> Res<String> {
        let len = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;

        Ok(unsafe { String::from_utf8_unchecked(buf) })
    }

    fn read_char(&mut self) -> Res<char> {
        Ok(unsafe { char::from_u32_unchecked(self.read_u32::<LE>()?) })
    }
}

// 3/3. type -> struct
pub trait Bytes: Sized {
    fn from_bytes<W: ParseBytes>(w: &mut W) -> Res<Self>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8,
    U128 = 16,
}

impl<T: std::io::Read> ParseBytes for T {}

impl ParseExpr for &str {
    fn write<W: Write>(&self, w: &mut W) -> Res<()> {
        let var = self.as_bytes();
        let len = var.len() as u32;

        w.write_u8(1)?;
        w.write_u32::<LE>(len)?;
        w.write_all(var)?;

        Ok(())
    }
}

impl ParseExpr for Expr {
    fn write<W: Write>(&self, w: &mut W) -> Res<()> {
        //dbg!(self);

        match self {
            Self::Lit(v) => v.write(w)?,
            Self::Unary(v) => v.write(w)?,

            _ => {
                panic!("{:#?}", self);
            }
        }

        Ok(())
    }
}

impl ParseExpr for ExprLit {
    fn write<W: Write>(&self, w: &mut W) -> Res<()> {
        let ExprLit { lit, .. } = self;

        match lit {
            Lit::Bool(v) => {
                // w.write_u8(1)?; // unnecessary
                w.write_u8(v.value() as u8)?;
            }

            // int
            Lit::Int(v) => {
                //dbg!(v);
                // ?i64
                let v: u128 = v.base10_parse()?;

                w.write_u8(1)?;
                write_int(w, &v)?;
            }

            // f64
            Lit::Float(v) => {
                let v: f64 = v.base10_parse()?;

                w.write_u8(1)?;
                w.write_f64::<LE>(v)?;
            }

            // char
            Lit::Char(v) => {
                let v = v.value();

                w.write_u8(1)?;
                w.write_u32::<LE>(v as u32)?;
            }

            // String
            Lit::Str(v) => {
                let v = v.value();
                let len = v.len();

                w.write_u8(1)?;
                w.write_u32::<LE>(len as u32)?;
                w.write_all(v.as_bytes())?;
            }

            // Vec<u8>
            Lit::ByteStr(v) => {
                let buf = v.value();
                let len = buf.len();

                if len == 0 {
                    // None
                    w.write_u8(0)?;
                } else {
                    w.write_u8(1)?; // Some(Vec<T>)
                    w.write_u32::<LE>(len as u32)?;

                    // write T
                    for f in buf.iter() {
                        w.write_u8(1)?; // Some(T)
                        w.write_u8(Flag::U8 as u8)?;
                        w.write_u8(*f)?;
                    }
                }
            }

            _ => {
                dbg!(lit);
                unreachable!()
            }
        }

        Ok(())
    }
}

impl ParseExpr for ExprUnary {
    fn write<W: Write>(&self, w: &mut W) -> Res<()> {
        //dbg!(self);
        let Self { op, expr, .. } = self;

        match op {
            UnOp::Neg(..) => {}

            // e.g. *"u8"
            // This means Option<T> is None
            UnOp::Deref(..) => {
                w.write_u8(0)?;
                return Ok(());
            }

            _ => return Err(crate::MyErr::Unsupported),
        }

        // UnOp::Neg(..)
        let Expr::Lit(ExprLit {
            lit, ..
        }) = expr.as_ref()
        else { unreachable!() };

        match lit {
            Lit::Int(v) => {
                let v: i128 = v.base10_parse()?;
                let v = (-1 * v) as u128;

                w.write_u8(1)?; // Some(..)
                write_int(w, &v)?;
            }

            Lit::Float(v) => {
                let v: f64 = v.base10_parse()?;

                w.write_u8(1)?;
                w.write_f64::<LE>(-1.0 * v)?;
            }

            _ => return Err(crate::MyErr::Unsupported),
        }

        Ok(())
    }
}

pub fn write_int<W: Write>(w: &mut W, v: &u128) -> Res<()> {
    match *v {
        0..=0xff => {
            w.write_u8(Flag::U8 as u8)?;
            w.write_u8(*v as u8)?;
        }

        0..=0xffff => {
            w.write_u8(Flag::U16 as u8)?;
            w.write_u16::<LE>(*v as u16)?;
        }

        0..=0xffff_ffff => {
            w.write_u8(Flag::U32 as u8)?;
            w.write_u32::<LE>(*v as u32)?;
        }

        0..=0xffff_ffff_ffff_ffff => {
            w.write_u8(Flag::U64 as u8)?;
            w.write_u64::<LE>(*v as u64)?;
        }

        0..=u128::MAX => {
            w.write_u8(Flag::U128 as u8)?;
            w.write_u128::<LE>(*v as u128)?;
        }
    }

    Ok(())
}

pub fn read_int<W: Read>(w: &mut W) -> Res<u128> {
    if w.read_u8()? != 1 {
        return Ok(0);
    }

    let f = Flag::from(w.read_u8()?);
    let res = match f {
        Flag::U8 => w.read_u8()? as u128,
        Flag::U16 => w.read_u16::<LE>()? as u128,
        Flag::U32 => w.read_u32::<LE>()? as u128,
        Flag::U64 => w.read_u64::<LE>()? as u128,
        Flag::U128 => w.read_u128::<LE>()? as u128,
    };

    Ok(res)
}

impl From<u8> for Flag {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::U8,
            2 => Self::U16,
            4 => Self::U32,
            8 => Self::U64,
            16 => Self::U128,

            _ => unreachable!(),
        }
    }
}
