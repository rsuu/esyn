use crate::{syn::*, *};

use std::time::Duration;

// TODO: dur!( 24/s  )
//       dur!( 24/ms )
//       dur!( 24/d  )
impl DeRs<Expr> for Duration {
    fn de(ast: &Expr) -> Res<Self> {
        let tmp = String::de(ast)?;
        let sd = SynDur::from_str(&tmp);

        todo!()
    }
}

impl MutPath for Duration {
    fn mut_path(&mut self, _iter: &mut std::slice::Iter<&Ident>, ast: &syn::Expr) -> Res<()> {
        *self = <Self as DeRs<Expr>>::de(ast)?;

        Ok(())
    }
}

impl EsynDefault for Duration {
    fn esyn_default() -> Self {
        Default::default()
    }
}

impl EsynSer for Duration {
    fn ser(&self) -> TokenStream {
        todo!()
    }
}

struct SynDur {
    num: u128,
    suffix: String,
}

impl SynDur {
    fn from_str(v: &str) -> Self {
        let mut num = Vec::with_capacity(10);
        let mut suffix = Vec::with_capacity(2);

        for c in v.chars() {
            match c {
                '0'..='9' => num.push(c),
                'a'..='z' => suffix.push(c),

                _ => unreachable!("{c}"),
            }
        }

        Self {
            num: String::from_iter(num).parse().unwrap(),
            suffix: String::from_iter(suffix),
        }
    }

    fn to_string(&self) -> String {
        let Self { num, suffix } = self;

        format!("{num}/{suffix}")
    }
}
