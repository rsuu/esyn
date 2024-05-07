use crate::{syn::*, *};

// b"abc"
#[derive(Debug, PartialEq, Clone)]
pub struct ByteStr {
    pub inner: Vec<u8>,
}

impl ByteStr {
    pub fn value(&self) -> &[u8] {
        &self.inner
    }
}

impl DeRs<Expr> for ByteStr {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(match ast {
            // e.g.
            //   b"abc"
            Expr::Lit(ExprLit {
                lit: Lit::ByteStr(v),
                ..
            }) => ByteStr { inner: v.value() },
            _ => unreachable!("{ast:#?}"),
        })
    }
}

impl MutPath for ByteStr {
    fn mut_path(&mut self, iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        if let Some(i) = iter.next() {
            assert_eq!(i, &"inner");
            self.inner.mut_path(iter, ast)?;
        } else {
            *self = <Self as DeRs<Expr>>::de(ast)?;
        }

        Ok(())
    }
}

impl EsynDefault for ByteStr {
    fn esyn_default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
