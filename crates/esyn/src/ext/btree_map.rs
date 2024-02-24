use crate::*;

use std::collections::BTreeMap;
use syn::*;

impl<K, V> DeRs<Expr> for BTreeMap<K, V>
where
    K: DeRs<Expr> + Ord,
    V: DeRs<Expr> + Ord,
{
    fn de(ast: &Expr) -> Res<Self> {
        Ok(Self::from_iter(<Vec<(K, V)> as DeRs<Expr>>::de(ast)?))
    }
}

impl<K, V> MutPath for BTreeMap<K, V>
where
    K: MutPath + Ord,
    V: MutPath + Ord,
{
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, _ast: &syn::Expr) -> Res<()> {
        unimplemented!()
    }
}

impl<K, V> EsynDefault for BTreeMap<K, V>
where
    K: EsynDefault + Ord,
    V: EsynDefault + Ord,
{
    fn esyn_default() -> Self {
        Default::default()
    }
}

impl<K, V> EsynSer for BTreeMap<K, V>
where
    K: EsynSer + Ord,
    V: EsynSer + Ord,
{
    fn ser(&self) -> TokenStream {
        let v = Vec::with_capacity(self.len());
        let (mut iter_k, mut iter_v) = (v.clone(), v.clone());
        for (k, v) in self.iter() {
            iter_k.push(k.ser());
            iter_v.push(v.ser());
        }

        // e.g.
        //   [ (1,2), (3,4) ]
        quote! {
            [
                #(
                ( #iter_k, #iter_v ),
                )*
            ]
        }
    }
}
