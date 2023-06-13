use crate::{func, Ast, Bytes, Func, MyErr, Res, TypeInfo};
use std::{
    any::Any,
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
    path::Path,
};
use syn::*;

// Design:
//
// ast1 from derive macro
// ast2 from file
//
// ast3 = merge(ast1, ast2)
// ast3.into_bytes()
//
// Self::from_bytes()

#[derive(Debug)]
pub struct Esyn {
    vec_func: Vec<Func>,
}

impl Esyn {
    pub fn new(input: &str) -> Res<Self> {
        let ast: syn::File = syn::parse_str(&input)?;
        //dbg!(&ast);

        let mut res = Self {
            vec_func: Vec::with_capacity(ast.items.len()),
        };
        for item in ast.items.iter() {
            //dbg!(&item);

            match item {
                Item::Fn(v) => res.vec_func.push(Func::from_item(v)),

                _ => return Err(MyErr::Unsupported),
            }
        }

        Ok(res)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Res<Self> {
        let mut f = File::open(path)?;
        let mut buf = "".to_string();
        f.read_to_string(&mut buf)?;

        Self::new(&buf)
    }

    pub fn get<T>(&self, fn_name: &str) -> Res<HashMap<String, T>>
    where
        T: Default + TypeInfo + Bytes + Ast,
    {
        let Some(Func {
            block,
            ref mut fmap,
            ..
        }) = self.get_fn(fn_name)
        else {
            return Err(MyErr::NotFound(fn_name.to_string()))
        };

        let ty = <T as TypeInfo>::name();
        let ast1 = {
            let ast1 = <T as Ast>::ast();
            let ast1 = syn::parse_str::<syn::Expr>(&ast1)?;

            let Expr::Struct(res) = ast1
            else { unreachable!() };

            res
        };
        //dbg!(&ast1);

        for stmt in block.stmts.iter() {
            //dbg!(&stmt);

            match stmt {
                // e.g. let val = struct {};
                Stmt::Local(v) => fmap.parse_let(v, &ty, &ast1),

                // e.g. val.inner = 123;
                Stmt::Expr(Expr::Assign(v), ..) => fmap.parse_stmt_expr_assign(v),

                // e.g. ::alias_path( ... );
                Stmt::Expr(Expr::Call(v), ..) => fmap.match_fn(&v)?,

                // ?
                Stmt::Expr(Expr::Closure(v), ..) => fmap.match_closure(&v)?,

                _ => return Err(MyErr::Unsupported),
            }
        }

        let mut res = HashMap::with_capacity(fmap.map.capacity());
        for (name, ast3) in fmap.map.iter() {
            //dbg!(&name);
            //dbg!(&ast3);

            let mut buf = vec![];
            let mut buf = Cursor::new(&mut buf);
            func::write_expr(&mut buf, &Expr::Struct(ast3.clone()))?;
            buf.set_position(0);
            //dbg!(&buf);
            res.insert(name.clone(), T::from_bytes(&mut buf)?);
        }

        Ok(res)
    }

    fn get_fn(&self, fn_name: &str) -> Option<Func> {
        for f in self.vec_func.iter() {
            if fn_name == f.ident {
                return Some(f.clone());
            }
        }

        None
    }
}
