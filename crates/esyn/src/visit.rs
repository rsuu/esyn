use crate::{parser::*, *};

use std::collections::HashMap;
use syn::*;

// REFS: https://stackoverflow.com/questions/35651279/error-closure-may-outlive-the-current-function-but-it-will-not-outlive-it
//       https://internals.rust-lang.org/t/allow-coercing-a-fn-value-to-a-static-mut-dyn-fnmut-value/17467/2
//
// let mut other = None;
// f(|ast| {
//    ^^^
//    |
//    lifetime: 'ast
//
//   other = ast.do();
//   ^^^^^
//   |
//   lifetime: 'other
// });

pub type FnVisit<'other, 'ast, Input, Output> = Box<dyn FnMut(Input) -> Option<Output> + 'other>;

pub type FnVisitStmtMacro<'other, 'ast, Output> =
    FnVisit<'other, 'ast, WrapStmtMacro<'ast>, Output>;

#[derive(Debug, Default)]
pub struct VisitItemFn<'ast> {
    pub inner: HashMap<&'ast Ident, FnBlock<'ast>>,
}

pub struct VisitStmtMacro<'ast> {
    pub vec: Vec<MacroRT<'ast>>,
    pub index: usize,
}

pub struct WrapExprBlock<'ast> {
    i: &'ast ExprBlock,

    pub depth: usize,
}

#[derive(Debug)]
pub struct WrapStmtMacro<'ast> {
    i: &'ast StmtMacro,

    path: String,
    index: usize,
}

impl<'ast> WrapStmtMacro<'ast> {
    pub fn parse2<T>(&self) -> Res<T>
    where
        T: syn::parse::Parse,
    {
        Ok(syn::parse2(self.tokens().clone())?)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn tokens(&self) -> &TokenStream {
        &self.i.mac.tokens
    }
}

pub enum MacroRT<'ast> {
    Core(WrapStmtMacro<'ast>),
    Crate(WrapStmtMacro<'ast>),
}

impl<'ast> VisitStmtMacro<'ast> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            index: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct VisitExprAssign<'ast> {
    pub inner: Vec<InnerExprAssign<'ast>>,
}

// e.g.
//   a.b.c = 123;
#[derive(Debug)]
pub struct InnerExprAssign<'ast> {
    pub left_head: &'ast Ident,
    pub left_body: Vec<&'ast Ident>,
    pub right: &'ast Expr,
}

#[derive(Debug, Default)]
pub struct VisitLocal<'ast> {
    pub inner: HashMap<&'ast Ident, &'ast Expr>,
}

#[derive(Debug, Default)]
pub struct CallAlias<'ast> {
    pub inner: HashMap<&'ast Ident, InnerCallAlias<'ast>>,
}

#[derive(Debug)]
pub struct InnerCallAlias<'ast> {
    pub src_head: &'ast Ident,
    pub src_body: Vec<&'ast Ident>,
}

// TODO: visit macro
//
// Esyn::init().vi(|ast| {
//    ...
// });
//

impl<'ast> Visit<'ast> for VisitStmtMacro<'ast> {
    fn visit_stmt_macro(&mut self, i: &'ast StmtMacro) {
        //dbg!(i);

        let StmtMacro {
            attrs,
            mac:
                Macro {
                    path:
                        Path {
                            leading_colon,
                            segments,
                        },
                    bang_token,
                    delimiter,
                    tokens,
                },
            semi_token,
        } = i;

        let flag = leading_colon.is_some();
        let path = join_path_seg(segments, "::");

        let wrap = WrapStmtMacro {
            path,
            index: self.vec.len(),
            i,
        };

        let res = {
            if flag {
                MacroRT::Core(wrap)
            } else {
                MacroRT::Crate(wrap)
            }
        };

        self.vec.push(res);
    }
}

impl<'ast> Visit<'ast> for VisitLocal<'ast> {
    fn visit_local(&mut self, i: &'ast Local) {
        if let Local {
            pat: Pat::Ident(PatIdent { ident, .. }),
            init: Some(LocalInit { expr, .. }),
            ..
        } = i
        {
            self.inner.insert(ident, expr.as_ref());
        }
    }
}

impl<'ast> Visit<'ast> for VisitExprAssign<'ast> {
    fn visit_expr_assign(&mut self, i: &'ast ExprAssign) {
        let left = i.left.as_ref();
        let right = i.right.as_ref();

        let (left_head, left_body) = get_field_path(left).unwrap();

        self.inner.push(InnerExprAssign {
            left_head,
            left_body,
            right,
        });
    }
}

impl<'ast> Visit<'ast> for VisitItemFn<'ast> {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let ident = &i.sig.ident;
        let output = &i.sig.output;

        let ret = RetType::from_ast(output);
        let Block { .. } = i.block.as_ref();

        self.inner.insert(ident, FnBlock::new(i, ret));
    }
}
