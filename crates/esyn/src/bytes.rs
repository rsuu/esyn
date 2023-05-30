use crate::Res;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{Read, Write};
use syn::*;

// step 3. esyn-bytes to type
pub trait Bytes: Sized {
    fn from_bytes<W: ParseBytes>(buf: &mut W) -> Res<Self>;
}

// step 2. bytes to esyn-bytes
pub trait ParseBytes: Read + ReadBytesExt + ReadBytesExt {
    fn read_bool(&mut self) -> Res<bool> {
        Ok(self.read_u8()? == 1)
    }

    fn read_string(&mut self) -> Res<String> {
        let len = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;

        Ok(String::from_utf8(buf)?)
    }

    fn read_char(&mut self) -> Res<char> {
        Ok(char::from_u32(self.read_u32::<LE>()?).unwrap())
    }
}

// step 1. syn/file to bytes
pub trait ParseExpr {
    fn write(&self, buf: &mut Vec<u8>) -> Res<()>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    I8 = 1,
    I16 = 2,
    I32 = 4,
    I64 = 8,
    I128 = 16,
}

impl From<u8> for Flag {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::I8,
            2 => Self::I16,
            4 => Self::I32,
            8 => Self::I64,
            16 => Self::I128,
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
                let v: i128 = v.base10_parse()?;

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
                        write_int(buf, &(-1 * v))?;
                    }

                    Lit::Float(v) => {
                        let v: f64 = v.base10_parse()?;
                        buf.write_f64::<LE>(-1.0 * v)?;
                    }

                    _ => unreachable!(),
                }
            }

            _ => {
                todo!()
            }
        }

        Ok(())
    }
}

pub fn write_int(buf: &mut Vec<u8>, v: &i128) -> Res<()> {
    match *v {
        -0x80..=0x7F => {
            buf.write_u8(Flag::I8 as u8)?;
            buf.write_i8(*v as i8)?;
        }

        -0x8000..=0x7FFF => {
            buf.write_u8(Flag::I16 as u8)?;
            buf.write_i16::<LE>(*v as i16)?;
        }

        -0x80000000..=0x7FFFFFFF => {
            buf.write_u8(Flag::I32 as u8)?;
            buf.write_i32::<LE>(*v as i32)?;
        }

        -0x8000000000000000..=0x7FFFFFFFFFFFFFFF => {
            buf.write_u8(Flag::I64 as u8)?;
            buf.write_i64::<LE>(*v as i64)?;
        }

        i128::MIN..=i128::MAX => {
            buf.write_u8(Flag::I128 as u8)?;
            buf.write_i128::<LE>(*v as i128)?;
        }
    }

    Ok(())
}

pub fn read_int<W: ParseBytes>(buf: &mut W) -> Res<i128> {
    if buf.read_u8()? != 1 {
        return Ok(0);
    }

    let f = Flag::from(buf.read_u8()?);
    let res = match f {
        Flag::I8 => buf.read_i8()? as i128,
        Flag::I16 => buf.read_i16::<LE>()? as i128,
        Flag::I32 => buf.read_i32::<LE>()? as i128,
        Flag::I64 => buf.read_i64::<LE>()? as i128,
        Flag::I128 => buf.read_i128::<LE>()? as i128,
    };

    Ok(res)
}
