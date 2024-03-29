use crate::{visit::*, *};

use std::cell::OnceCell;
use syn::*;

#[derive(Debug)]
pub struct Esyn<'ast> {
    ast: File,
    // ?sync::OnceLock
    pub map_fn: OnceCell<VisitItemFn<'ast>>,
}

#[derive(Debug)]
pub struct EsynBuilder {
    fn_name: String,
    let_name: Option<String>,
    flag_res: bool,
}

#[derive(Debug)]
pub struct FnBlock<'ast> {
    pub inner: &'ast ItemFn,
    //pub ident: &'ast Ident,
    //pub stmts: &'ast Vec<Stmt>,

    // e.g.
    //   let ident = expr;
    pub map_local: VisitLocal<'ast>,
    // e.g.
    //   ident = expr;
    pub map_assign: VisitExprAssign<'ast>,
    // e.g.
    //   ::alias(xxx, yyy);
    //   xxx = 123;
    pub map_alias: CallAlias<'ast>,

    pub ret: RetType,
    //pub ext_expr:Vec<Expr>
}

#[derive(Debug, Default, PartialEq)]
pub enum RetType {
    #[default]
    Unit,

    Any,
}

impl<'ast> Esyn<'ast> {
    pub fn new(code: &str) -> Self {
        Self {
            ast: syn::parse_str(code).unwrap(),
            map_fn: OnceCell::new(),
        }
    }

    pub fn init(&'ast self) -> Res<()> {
        self.update_map_fn()
    }

    pub fn get_value<T>(&self, fn_name: &str, let_name: &str) -> Res<Wrap<T>>
    where
        T: DeRs<Expr> + MutPath,
    {
        let mut res: T = self.inner_get_value(fn_name, let_name)?;

        let fn_name = quote::format_ident!("{fn_name}");
        let fb = self
            .map_fn
            .get()
            .unwrap()
            .inner
            .get(&fn_name)
            .ok_or(err!(NotFound: "{fn_name}"))?;

        fb.exec(&mut res, let_name)?;

        Ok(Wrap::new(res))
    }

    pub fn get_res<T>(&self, fn_name: &str) -> Res<Wrap<T>>
    where
        T: DeRs<Expr>,
    {
        let fn_name = quote::format_ident!("{fn_name}");
        let FnBlock { inner, ret, .. } = self
            .map_fn
            .get()
            .ok_or(MyErr::Todo)?
            .inner
            .get(&fn_name)
            .ok_or(err! {NotFound: "{fn_name}"})?;

        if ret != &RetType::Any {
            return err!(Expected "Any", "TODO");
        }

        let Stmt::Expr(expr, None) = &inner.block.stmts[0] else {
            unreachable!("{inner:#?}")
        };

        Ok(Wrap::new(<T as DeRs<Expr>>::de(expr)?))
    }

    fn inner_get_value<T>(&self, fn_name: &str, let_name: &str) -> Res<T>
    where
        T: DeRs<Expr> + MutPath,
    {
        let fn_name = quote::format_ident!("{fn_name}");
        let let_name = quote::format_ident!("{let_name}");

        let expr = {
            let map = &self.map_fn.get().ok_or(MyErr::Todo)?.inner;
            let map = &map.get(&fn_name).ok_or(MyErr::Todo)?.map_local.inner;

            map.get(&let_name).ok_or(MyErr::Todo)?
        };

        <T as DeRs<Expr>>::de(expr)
    }

    fn update_map_fn(&'ast self) -> Res<()> {
        let mut tmp: VisitItemFn = Default::default();
        tmp.visit_file(&self.ast);

        // update fields
        for (.., fb) in tmp.inner.iter_mut() {
            for ast in fb.inner.block.stmts.iter() {
                fb.visit_stmt(ast);
            }
        }

        //let crate_fn = crate_fn.unwrap_or(|_| Ok(()));
        //for (.., f) in tmp.inner.iter_mut() {
        //   f.visit()?;
        //}

        // init
        self.map_fn.set(tmp).unwrap();

        Ok(())
    }
}

impl<'ast> Esyn<'ast> {
    fn _get_fn(&'ast self, fn_name: &str) -> Res<&'ast FnBlock> {
        let fn_name = quote::format_ident!("{fn_name}");
        let fb = self
            .map_fn
            .get()
            .unwrap()
            .inner
            .get(&fn_name)
            .ok_or(err!(NotFound: "{fn_name}"))?;

        Ok(fb)
    }
}

impl<'ast> FnBlock<'ast> {
    pub fn new(inner: &'ast ItemFn, ret: RetType) -> Self {
        Self {
            inner,
            ret,
            map_local: Default::default(),
            map_assign: Default::default(),
            map_alias: Default::default(),
        }
    }

    fn _visit(&mut self) -> Res<()> {
        for stmt in self.inner.block.stmts.iter() {
            self.visit_stmt(stmt);
        }

        Ok(())
    }

    fn exec<T: MutPath>(&self, res: &mut T, let_name: &str) -> Res<()> {
        let map_assign = &self.map_assign;
        let map_alias = &self.map_alias;

        for InnerExprAssign {
            left_head,
            left_body,
            right,
        } in map_assign.inner.iter()
        {
            // assign:
            //   a.b.c.d = 456;
            //   │ └─┬─┘
            //   │   └── body
            //   └── head AND let_name
            //
            if *left_head == let_name {
                let mut path = left_body.clone();
                path.reverse();

                // update
                res.mut_path(&mut path.iter(), right)?;
            }
            // alias:
            //   ::alias(_alias, a.b.c.d);
            //   _alias.field = 123;
            //
            //                src_head
            //   InnerCall       │src_body
            //     ┌─┴─┐         │ ┌─┴─┐
            //   ::alias(_alias, a.b.c.d);
            //           └─┬──┘  └──┬──┘
            //             │        │
            //             │        └── src
            //             └── let_name/alias
            //
            //   _alias.field = 123;
            //   └─┬──┘ └─┬─┘    └─ val
            //     │      └── field
            //     └── let_name/alias
            //
            else if let Some(InnerCallAlias { src_head, src_body }) =
                map_alias.inner.get(left_head)
            {
                if *src_head == let_name {
                    let mut path = left_body.clone();

                    path.extend_from_slice(src_body.as_slice());
                    path.reverse();

                    res.mut_path(&mut path.iter(), right)?;
                }
            }
        }

        Ok(())
    }

    fn inner_visit_expr_call<'out: 'ast>(
        &mut self,
        ExprCall { func, args, .. }: &'out ExprCall,
    ) -> Res<()> {
        let Expr::Path(ExprPath {
            path: syn::Path {
                leading_colon,
                segments,
            },
            ..
        }) = func.as_ref()
        else {
            return Ok(());
        };

        let flag = leading_colon.is_some();
        let path = join_path_seg(segments, "::");

        match (flag, path.as_str()) {
            // e.g.
            //   ::alias(www, v.a.b.c);
            //   www = 123;
            //
            (true, "alias") => self.up_fn_alias(args),

            // e.g.
            //   crate::func( ... );
            //   (false, ..)
            //let name = join_path_seg(segments, "::");
            //(fn_block, name.as_str(), args).fm(crate_fn)?;
            _ => Ok(()),
        }
    }

    fn up_fn_alias<'out: 'ast>(&mut self, args: &'out Punctuated<Expr, Token![,]>) -> Res<()> {
        let (
            Expr::Path(ExprPath {
                path: Path { segments, .. },
                ..
            }),
            path,
        ) = (&args[0], &args[1])
        else {
            return err!(Panic "expected `path`");
        };

        let dst = &segments[0].ident;
        let (src_head, src_body) = get_field_path(path)?;

        self.map_alias
            .inner
            .insert(dst, InnerCallAlias { src_head, src_body });

        Ok(())
    }
}

impl EsynBuilder {
    pub fn new() -> Self {
        Self {
            fn_name: "main".to_string(),
            let_name: None,
            flag_res: false,
        }
    }

    pub fn set_fn<T: Into<String>>(mut self, i: T) -> Self {
        self.fn_name = i.into();

        self
    }

    pub fn set_let<T: Into<String>>(mut self, i: T) -> Self {
        self.let_name = Some(i.into());

        self
    }

    pub fn flag_res(mut self) -> Self {
        self.flag_res = true;

        self
    }

    //pub fn with_fn_expr<'scope, O>(f: FnExpr<'scope, O>) {}

    pub fn get_once<T>(&self, code: &str) -> Res<Wrap<T>>
    where
        T: DeRs<Expr> + MutPath,
    {
        let tmp = Esyn::new(code);
        tmp.update_map_fn()?;

        self.get(&tmp)
    }

    pub fn get<T>(&self, esyn: &Esyn) -> Res<Wrap<T>>
    where
        T: DeRs<Expr> + MutPath,
    {
        match &self {
            // e.g.
            //   let a = 1;
            //       ^
            Self {
                ref fn_name,
                let_name: Some(let_name),
                flag_res: false,
                ..
            } => esyn.get_value(fn_name, let_name),

            // e.g.
            //   fn main() -> Any {}
            //                ^^^
            Self {
                ref fn_name,
                let_name: None,
                flag_res: true,
                ..
            } => esyn.get_res(fn_name),

            _ => unreachable!("{self:#?}"),
        }
    }
}

impl RetType {
    pub fn from_ast(ast: &ReturnType) -> Self {
        let ReturnType::Type(.., ty) = ast else {
            return Default::default();
        };

        match ty.as_ref() {
            // e.g.
            //   fn f() -> Any{ ... }
            Type::Path(TypePath { path, .. })
                if (path.segments.len() == 1 && path.segments[0].ident == "Any") =>
            {
                Self::Any
            }

            _ => Default::default(),
        }
    }
}

impl<'ast> Visit<'ast> for FnBlock<'ast> {
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        match i {
            Stmt::Expr(Expr::Block(_ast), Some(..)) => {
                // TODO:
                // ?Fn OR visit
            }

            Stmt::Local(ast) => self.map_local.visit_local(ast),

            Stmt::Expr(Expr::Assign(ast), Some(..)) => self.map_assign.visit_expr_assign(ast),
            Stmt::Expr(Expr::Call(ast), ..) => self.visit_expr_call(ast),

            _ => {}
        }
    }

    fn visit_expr_call(&mut self, i: &'ast ExprCall) {
        self.inner_visit_expr_call(i).unwrap();
    }
}

impl Default for EsynBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn _join_field_path(i: &[&Ident]) -> String {
    let mut res = Vec::with_capacity(i.len());
    for f in i.iter().rev() {
        res.push(f.to_string());
    }

    res.join(".")
}

pub fn join_path_seg(i: &punctuated::Punctuated<PathSegment, Token!(::)>, sep: &str) -> String {
    let mut res = Vec::with_capacity(i.len());
    for PathSegment { ident, .. } in i.iter() {
        res.push(ident.to_string());
    }

    res.join(sep)
}

pub fn get_field_path(i: &Expr) -> Res<(&Ident, Vec<&Ident>)> {
    // maybe 6
    let mut tmp = Vec::with_capacity(6);
    let mut expr = i;
    loop {
        match expr {
            Expr::Field(ExprField {
                member: Member::Named(i),
                base,
                ..
            }) => {
                tmp.push(i);
                expr = base;
            }

            Expr::Path(v) => return Ok((v.path.get_ident().unwrap(), tmp)),

            _ => unreachable!("{expr:#?}"),
        }
    }
}
