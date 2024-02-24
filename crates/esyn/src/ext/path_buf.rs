use crate::*;

use std::path::PathBuf;
use syn::*;

impl DeRs<Expr> for PathBuf {
    fn de(ast: &Expr) -> Res<Self> {
        Ok(Self::from(<String as DeRs<Expr>>::de(ast)?))
    }
}

impl MutPath for PathBuf {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        *self = <Self as DeRs<Expr>>::de(ast)?;
        Ok(())
    }
}

impl EsynDefault for PathBuf {
    fn esyn_default() -> Self {
        Default::default()
    }
}

impl EsynSer for PathBuf {
    fn ser(&self) -> TokenStream {
        let v = self.to_str().unwrap();
        quote! {
            #v
        }
    }
}
