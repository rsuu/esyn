use crate::{syn::*, *};

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// Enum::Unnamed(...)
impl DeRs<Expr> for IpAddr {
    fn de(ast: &Expr) -> Res<Self> {
        let var = match ast {
            // Unnamed
            Expr::Call(ExprCall { func, .. }) => {
                let Expr::Path(ExprPath {
                    path: Path { segments, .. },
                    ..
                }) = func.as_ref()
                else {
                    unreachable!()
                };

                assert_eq!(segments.len(), 2);

                let PathSegment { ident, .. } = &segments[1];

                ident.to_string()
            }
            _ => unreachable!("{:#?}", ast),
        };

        // from tuple
        Ok(match var.as_str() {
            "V4" => {
                let Expr::Call(ExprCall { args: _, .. }) = ast else {
                    unreachable!()
                };

                let (a, b, c, d) = <(u8, u8, u8, u8) as DeRs<Expr>>::de(ast)?;

                Self::V4(Ipv4Addr::new(a, b, c, d))
            }

            "V6" => {
                let Expr::Call(ExprCall { args: _, .. }) = ast else {
                    unreachable!()
                };

                let (a, b, c, d, e, f, g, h) =
                    <(u16, u16, u16, u16, u16, u16, u16, u16) as DeRs<Expr>>::de(ast)?;

                Self::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h))
            }

            _ => unreachable!(),
        })
    }
}

// from array
impl DeRs<Expr> for Ipv4Addr {
    fn de(ast: &Expr) -> Res<Self> {
        let [a, b, c, d] = <[u8; 4] as DeRs<Expr>>::de(ast)?;

        Ok(Self::new(a, b, c, d))
    }
}

// from array
impl DeRs<Expr> for Ipv6Addr {
    fn de(ast: &Expr) -> Res<Self> {
        let [a, b, c, d, e, f, g, h] = <[u16; 8] as DeRs<Expr>>::de(ast)?;

        Ok(Self::new(a, b, c, d, e, f, g, h))
    }
}

impl MutPath for IpAddr {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        unimplemented!()
    }
}

impl MutPath for Ipv4Addr {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        unimplemented!()
    }
}

impl MutPath for Ipv6Addr {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        unimplemented!()
    }
}

impl EsynDefault for IpAddr {
    fn esyn_default() -> Self {
        Self::V4(Ipv4Addr::esyn_default())
    }
}

impl EsynDefault for Ipv4Addr {
    fn esyn_default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl EsynDefault for Ipv6Addr {
    fn esyn_default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0)
    }
}

impl EsynSer for IpAddr {
    fn ser(&self) -> TokenStream {
        match self {
            Self::V4(v) => {
                let iter = v.octets();

                quote! {
                    IpAddr::V4(
                        #(
                        #iter ,
                        )*
                    )
                }
            }
            Self::V6(v) => {
                let iter = v.segments();

                quote! {
                    IpAddr::V4(
                        #(
                        #iter ,
                        )*
                    )
                }
            }
        }
    }
}

impl EsynSer for Ipv4Addr {
    fn ser(&self) -> TokenStream {
        let v = self.octets();

        v.as_slice().ser()
    }
}

impl EsynSer for Ipv6Addr {
    fn ser(&self) -> TokenStream {
        let v = self.segments();

        v.as_slice().ser()
    }
}
