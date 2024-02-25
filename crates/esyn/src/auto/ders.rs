use crate::*;
use syn::*;

//pub type FnDeRsMacro=FnMut();

/// Type::from_ast()
pub trait DeRs<E>: Sized + EsynDefault {
    fn de(ast: &E) -> Res<Self>;
}

macro_rules! impl_DeRs_for_num {
    ( $($t:ty)* ) => {$(
impl DeRs<Expr> for $t {
    fn de(ast: &Expr) -> Res<Self> {
        match ast {
            Expr::Lit(v) => DeRs::de(v),
            Expr::Unary(v) => DeRs::de(v),

            _ => unimplemented!(),
        }
    }
}
    )*}
}

macro_rules! impl_DeRs_ExprLit_for_num {
    ( $($t:ty)* ) => {$(
impl DeRs<ExprLit> for $t {
    fn de(ast: &ExprLit) -> Res<Self> {
        let ExprLit { lit, .. } = ast;

        Ok(match lit {
            Lit::Int(v)  => v.base10_parse::<$t>()?,
            Lit::Float(v) => v.base10_parse::<$t>()?,
            _ => unimplemented!()
        })
    }
}
    )*}
}

macro_rules! impl_DeRs_ExprUnary_for_num {
    ( $($t:ty)* ) => {$(
impl DeRs<ExprUnary> for $t {
    fn de(ast: &ExprUnary) -> Res<Self> {
        let ExprUnary {
            op: UnOp::Neg(..),
            expr, ..
        } = ast
        else { unreachable!() };

        let Expr::Lit(ExprLit {
            lit, ..
        }) = expr.as_ref()
        else { unreachable!() };

        Ok(match lit {
            Lit::Int(v) => format!("-{}", v.base10_digits()).parse()?,
            Lit::Float(v) => format!("-{}", v.base10_digits()).parse()?,

            _ => unreachable!("{lit:#?}"),
        })
    }
}
    )*}
}

macro_rules! impl_DeRs_for_tuple {
    ( $($t:ident),+ ) => {
impl< $($t: DeRs<Expr>),+ > DeRs<Expr> for ( $($t,)+ ) {
    fn de(ast: &Expr) -> Res<Self> {
        let mut iter = match ast {
            Expr::Tuple(ExprTuple { elems, .. }) => elems.iter(),
            Expr::Call(ExprCall { args, .. }) => args.iter(),

            _ => unreachable!("{ast:#?}"),
        };

        Ok(
            (
                $(
                    <$t as DeRs<Expr>>::de(
                        &iter.next().unwrap()
                    ).unwrap() ,
                )+
            )
        )
    }
}
    }
}

impl_DeRs_for_num! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
}
impl_DeRs_ExprLit_for_num! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
}
impl_DeRs_ExprUnary_for_num! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
}

// REFS: https://doc.rust-lang.org/stable/std/primitive.tuple.html
// Due to a temporary restriction in Rust’s type system, the following traits are only implemented on tuples of arity 12 or less.
impl_DeRs_for_tuple!(A);
impl_DeRs_for_tuple!(A, B);
impl_DeRs_for_tuple!(A, B, C);
impl_DeRs_for_tuple!(A, B, C, D);
impl_DeRs_for_tuple!(A, B, C, D, E);
impl_DeRs_for_tuple!(A, B, C, D, E, F);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G, H);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_DeRs_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl DeRs<Expr> for () {
    fn de(_ast: &Expr) -> Res<Self> {
        Ok(())
    }
}

impl<T: DeRs<Expr>> DeRs<Expr> for Box<T> {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(Box::new(DeRs::<Expr>::de(ast)?))
    }
}

impl<T: DeRs<Expr>> DeRs<Expr> for Option<T> {
    fn de(ast: &Expr) -> Res<Self> {
        // destructure box: Some(box( ... ))
        // https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=c660bffdc36a267f04c84d01a9cd3694
        // https://github.com/rust-lang/rust/issues/29641

        //dbg!(&ast);
        match ast {
            Expr::Call(ExprCall { func, args, .. }) => {
                let Expr::Path(ExprPath {
                    path: Path { segments, .. },
                    ..
                }) = func.as_ref()
                else {
                    unreachable!()
                };

                debug_assert_eq!(segments[0].ident, "Some");

                Ok(Some(DeRs::de(&args[0])?))
            }

            Expr::Path(ExprPath {
                path: Path { segments, .. },
                ..
            }) => {
                debug_assert_eq!(segments[0].ident, "None");

                Ok(None)
            }

            _ => unreachable!("{ast:#?}"),
        }
    }
}

impl<T: DeRs<Expr>> DeRs<Expr> for Vec<T> {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(match ast {
            Expr::Array(ExprArray { elems, .. }) => {
                let mut res = Vec::with_capacity(elems.len());
                for f in elems.iter() {
                    res.push(DeRs::de(f).unwrap());
                }

                res
            }

            _ => unreachable!("{ast:#?}"),
        })
    }
}

impl<T: DeRs<Expr> + Copy, const N: usize> DeRs<Expr> for [T; N] {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(match ast {
            Expr::Array(ExprArray { elems, .. }) => {
                let mut res = [T::esyn_default(); N];
                for i in 0..N {
                    res[i] = DeRs::de(&elems[i])?;
                }

                res
            }

            _ => unreachable!("{ast:#?}"),
        })
    }
}

impl DeRs<Expr> for bool {
    fn de(ast: &Expr) -> Res<Self> {
        let Expr::Lit(ExprLit {
            lit: Lit::Bool(v), ..
        }) = ast
        else {
            unreachable!()
        };

        Ok(v.value())
    }
}

impl DeRs<Expr> for char {
    fn de(ast: &Expr) -> Res<Self> {
        let Expr::Lit(ExprLit {
            lit: Lit::Char(v), ..
        }) = ast
        else {
            unreachable!()
        };

        Ok(v.value())
    }
}

impl DeRs<Expr> for String {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(match ast {
            // e.g.
            //   "abc"
            Expr::Lit(ExprLit {
                lit: Lit::Str(v), ..
            }) => v.value(),

            // e.g.
            //   ("abc")
            Expr::Paren(ExprParen { expr, .. }) => {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(v), ..
                }) = expr.as_ref()
                {
                    v.value()
                } else {
                    unimplemented!()
                }
            }

            _ => unreachable!("{ast:#?}"),
        })
    }
}
