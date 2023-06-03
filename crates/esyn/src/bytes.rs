use crate::Res;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{Read, Write};
use syn::*;

// step 3. type -> struct
pub trait Bytes: Sized {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self>;
}

// step 2. bytes -> type
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

// step 1. syn/file -> bytes
pub trait ParseExpr {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8,
    U128 = 16,
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

impl<T: std::io::Read> ParseBytes for T {}

impl ParseExpr for &str {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        buf.write_u8(1)?;

        let var = self.as_bytes();
        let len = var.len() as u32;
        buf.write_u32::<LE>(len)?;
        buf.write_all(var)?;

        Ok(())
    }
}

impl ParseExpr for Expr {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        //dbg!(self);

        match self {
            Self::Lit(v) => v.write(buf)?,
            Self::Unary(v) => v.write(buf)?,

            _ => {
                dbg!(self);
                unreachable!()
            }
        }

        Ok(())
    }
}

impl ParseExpr for ExprLit {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        let ExprLit { lit, .. } = self;

        buf.write_u8(1)?;

        match lit {
            Lit::Int(v) => {
                //dbg!(v);
                // ?i64
                let v: u128 = v.base10_parse()?;

                write_int(buf, &v)?;
            }

            Lit::Float(v) => {
                let v: f64 = v.base10_parse()?;

                buf.write_f64::<LE>(v)?;
            }

            Lit::Char(v) => {
                let v = v.value();

                buf.write_u32::<LE>(v as u32)?;
            }

            Lit::Str(v) => {
                let v = v.value();
                let len = v.len();

                buf.write_u32::<LE>(len as u32)?;
                buf.write_all(v.as_bytes())?;
            }

            Lit::Bool(v) => {
                let v = v.value();

                // bool is special
                buf.pop().unwrap();
                buf.write_u8(v as u8)?;
            }

            _ => unreachable!(),
        }

        Ok(())
    }
}

impl ParseExpr for ExprUnary {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()> {
        use syn::UnOp::*;

        let Self { op, expr, .. } = self;

        match op {
            // NOTE:
            // e.g. *"u8"
            Deref(..) => buf.write_u8(0)?,

            Neg(..) => {
                let Expr::Lit(ExprLit {
                    lit, ..
                }) = expr.as_ref() else {
                    //dbg!(self);
                    todo!()
                };

                buf.write_u8(1)?;

                match lit {
                    Lit::Int(v) => {
                        let v: i128 = v.base10_parse()?;
                        write_int(buf, &((-1 * v) as u128))?;
                    }

                    Lit::Float(v) => {
                        let v: f64 = v.base10_parse()?;
                        buf.write_f64::<LE>(-1.0 * v)?;
                    }

                    _ => unreachable!(),
                }
            }

            _ => return Err(crate::MyErr::Unsupported),
        }

        Ok(())
    }
}

pub fn write_int(buf: &mut Vec<u8>, v: &u128) -> Res<()> {
    match *v {
        0..=0xff => {
            buf.write_u8(Flag::U8 as u8)?;
            buf.write_u8(*v as u8)?;
        }

        0..=0xffff => {
            buf.write_u8(Flag::U16 as u8)?;
            buf.write_u16::<LE>(*v as u16)?;
        }

        0..=0xffff_ffff => {
            buf.write_u8(Flag::U32 as u8)?;
            buf.write_u32::<LE>(*v as u32)?;
        }

        0..=0xffff_ffff_ffff_ffff => {
            buf.write_u8(Flag::U64 as u8)?;
            buf.write_u64::<LE>(*v as u64)?;
        }

        0..=u128::MAX => {
            buf.write_u8(Flag::U128 as u8)?;
            buf.write_u128::<LE>(*v as u128)?;
        }
    }

    Ok(())
}

pub fn read_int<W: ParseBytes>(buf: &mut W) -> Res<u128> {
    if buf.read_u8()? != 1 {
        return Ok(0);
    }

    let f = Flag::from(buf.read_u8()?);
    let res = match f {
        Flag::U8 => buf.read_u8()? as u128,
        Flag::U16 => buf.read_u16::<LE>()? as u128,
        Flag::U32 => buf.read_u32::<LE>()? as u128,
        Flag::U64 => buf.read_u64::<LE>()? as u128,
        Flag::U128 => buf.read_u128::<LE>()? as u128,
    };

    Ok(res)
}
