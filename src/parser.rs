use crate::{Ast, Bytes, ParseExpr, Res, TypeInfo};
use byteorder::{WriteBytesExt, LE};
use std::{
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
// struct = Self::from_bytes()

#[derive(Debug)]
pub struct Esyn {
    func_list: Vec<Func>,
}

#[derive(Debug)]
pub struct Func {
    ident: String,
    block: Block,
}

impl Esyn {
    pub fn new(input: &str) -> Res<Self> {
        let ast: syn::File = syn::parse_str(&input)?;
        //dbg!(&ast);

        let mut res = Self { func_list: vec![] };
        for item in ast.items.iter() {
            //dbg!(&item);

            match item {
                Item::Fn(v) => res.func_list.push(Func::from_item(v)),

                _ => {
                    unreachable!()
                }
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
        let Some(
            Block { ref stmts, .. }
            ) = self.try_get_fn(fn_name)
        else {
            return Err(crate::MyErr::Todo);
        };

        let ast1 = {
            let ast1 = <T as Ast>::ast();
            let ast1 = syn::parse_str::<syn::Expr>(&ast1)?;

            let Expr::Struct(res) = ast1
            else { unreachable!() };

            res
        };
        let ty = <T as TypeInfo>::name();

        let mut map: HashMap<String, ExprStruct> = HashMap::new();
        for stmt in stmts.iter() {
            //dbg!(&stmt);

            match stmt {
                Stmt::Local(Local {
                    attrs: _,
                    let_token: token::Let { .. },
                    pat: Pat::Ident(PatIdent { ident, .. }),
                    init:
                        Some(LocalInit {
                            expr,
                            diverge: None,
                            ..
                        }),
                    ..
                }) => {
                    let mut ast1 = ast1.clone();
                    let val_name = ident.to_string();
                    //dbg!(&val_name);

                    let Expr::Struct(ast2) = expr.as_ref()
                            else { unreachable!() };

                    merge_struct(ty, &mut ast1, ast2);

                    //dbg!(&ast1);
                    map.insert(val_name, ast1);
                }

                Stmt::Expr(Expr::Assign(ExprAssign { left, right, .. }), ..) => {
                    //dbg!(&left);
                    //dbg!(&right);

                    let l = &mut left.clone();
                    let mut path = vec![];

                    while let Expr::Field(ExprField {
                        base,
                        dot_token: _,
                        member: Member::Named(i),
                        ..
                    }) = l.as_ref()
                    {
                        path.push(i.to_string());
                        *l = base.clone();
                    }

                    let Expr::Path(ExprPath {
                            path: syn::Path {
                                segments,
                                ..
                            },
                            ..
                        }) = l.as_ref()

                        else { unreachable!() };
                    let val_name = segments[0].ident.to_string();
                    //dbg!(&val_name);

                    if let Some(v) = map.get_mut(val_name.as_str()) {
                        path.reverse();
                        let path = format!(".{}", path.join("."));

                        //dbg!(&path, &right);
                        update_struct(v, &[(path, right.as_ref().clone())]);
                    }
                }

                _ => {
                    todo!()
                }
            }
        }

        let mut res = HashMap::with_capacity(map.capacity());
        for (name, ast3) in map.iter() {
            let buf = &mut vec![];
            write_expr(buf, &Expr::Struct(ast3.clone()))?;
            //dbg!(&ast3);
            //dbg!(&buf);
            let buf = Cursor::new(&buf);

            res.insert(name.clone(), T::from_bytes(buf)?);
        }

        Ok(res)
    }

    fn try_get_fn(&self, fn_name: &str) -> Option<&Block> {
        for Func {
            ref ident,
            ref block,
        } in self.func_list.iter()
        {
            if fn_name == ident {
                return Some(&block);
            }
        }

        None
    }
}

impl Func {
    fn new(ident: String, block: Block) -> Self {
        Self { ident, block }
    }

    fn from_item(item: &ItemFn) -> Self {
        let ItemFn {
            block,
            sig: Signature { ident, .. },
            ..
        } = item;

        Self::new(ident.to_string(), block.as_ref().clone())
    }
}

fn merge_struct(ty: &str, left: &mut ExprStruct, right: &ExprStruct) {
    let ExprStruct { path, .. } = left;
    let (ty2, _is_enum) = get_type(path);

    if ty != ty2.as_str() {
        //dbg!(ty, &ty2);
        return;
    }

    let _path = ty.clone();
    let list = trans(right);
    //dbg!(&list);

    update_struct(left, &list);
}

fn trans(expr: &ExprStruct) -> Vec<(String, Expr)> {
    //dbg!(&expr);
    let mut res = vec![];
    let mut tmp = "".to_string();

    inner_trans(expr, &mut res, &mut tmp);

    //dbg!(&res);
    res
}

fn inner_trans(v: &ExprStruct, res: &mut Vec<(String, Expr)>, tmp: &mut String) {
    for FieldValue { member, expr, .. } in v.fields.iter() {
        let Member::Named(i) = member
        else { continue; };

        match &expr {
            Expr::Struct(next) => {
                let ExprStruct { path, .. } = next;

                let (_ty, is_enum) = get_type(path);
                if is_enum {
                    let n = format!("{}.{}", tmp.as_str(), i.to_string());
                    res.push((n.clone(), expr.clone()));
                    //dbg!(&ty, expr, n);
                } else {
                    let mut n = tmp.clone();
                    n.push('.');
                    n.push_str(&i.to_string());

                    inner_trans(next, res, &mut n);
                }
            }

            _ => {
                res.push((format!("{}.{}", tmp.as_str(), i.to_string()), expr.clone()));
            }
        }
    }
}

fn update_struct(v: &mut ExprStruct, list: &[(String, Expr)]) {
    inner_update_struct(v, list, &mut "".to_string());
}

fn inner_update_struct(v: &mut ExprStruct, list: &[(String, Expr)], tmp: &mut String) {
    for FieldValue { member, expr, .. } in v.fields.iter_mut() {
        let Member::Named(i) = member
        else { continue; };

        let mut p = format!("{}.{}", tmp, &i.to_string());
        match expr {
            Expr::Struct(next) => {
                let ExprStruct { path, .. } = next;
                let (_ty, is_enum) = get_type(path);

                if is_enum {
                    //dbg!(&p, &ty, is_enum);

                    for (path, expr2) in list.iter() {
                        if path.as_str() != p.as_str() {
                            //dbg!(&path, &p);
                            continue;
                        }

                        *expr = expr2.clone();
                    }
                } else {
                    for (path, expr2) in list.iter() {
                        if path.as_str() != p.as_str() {
                            //dbg!(&path, &p);
                            continue;
                        }

                        if let Expr::Struct(right) = expr2 {
                            let ExprStruct { path, .. } = right;

                            let (_ty, is_enum) = get_type(path);

                            if is_enum {
                                //dbg!(&p, &ty, is_enum);
                            }
                            let assign_list = trans(right);
                            //dbg!(&assign_list);
                            update_struct(next, &assign_list);
                        }
                    }

                    inner_update_struct(next, list, &mut p);
                }
            }

            _ => {
                for (path, expr2) in list.iter() {
                    if path.as_str() != p.as_str() {
                        //dbg!(&path, &p);
                        continue;
                    }

                    *expr = expr2.clone();
                    //dbg!(&path, &p, &expr2, &expr);
                }
            }
        }
    }
}

fn write_expr(buf: &mut Vec<u8>, expr: &Expr) -> Res<()> {
    //dbg!(&expr);

    match expr {
        // Empty:
        //     Struct {}
        //     Enum {}
        Expr::Struct(ExprStruct {
            path: _, fields, ..
        }) if fields.len() == 0 => {
            buf.write_u8(0)?;
        }

        // Named:
        //     StructNamed { ... }
        //     Enum::Named { ... }
        Expr::Struct(ExprStruct { path, fields, .. }) => {
            let (ty, is_enum) = get_type(path);
            if is_enum {
                buf.write_u8(1)?;
                ty.as_str().write(buf)?;
            }

            for FieldValue { expr, .. } in fields.iter() {
                write_expr(buf, expr)?;
            }
        }

        // Unnamed:
        //     Struct( ... )
        //     Enum  ( ... )
        Expr::Call(ExprCall { func, args, .. }) => {
            let Expr::Path(
                ExprPath { path, .. }
                ) = func.as_ref()
            else { unimplemented!() };

            let (ty, is_enum) = get_type(path);

            if is_enum {
                // Enum( ... )
                buf.write_u8(1)?;
                ty.as_str().write(buf)?;
            } else if ty.as_str() == "Some" {
                // Enum( ... )
                buf.write_u8(1)?;
                ty.as_str().write(buf)?;
            } else {
                // Struct( ... )
                //dbg!(&ty);
            }

            for expr in args.iter() {
                write_expr(buf, expr)?;
            }
        }

        // Unit:
        //     Enum::Unit
        Expr::Path(ExprPath {
            attrs: _,
            qself: _,
            path,
        }) => {
            let (ty, is_enum) = get_type(path);
            //dbg!(&ty);

            if is_enum {
                buf.write_u8(1)?;
                ty.as_str().write(buf)?;
            } else {
                if ty.as_str() == "None" {
                    buf.write_u8(0)?;
                }
                //dbg!(&ty);
            }
        }

        // Tuple:
        //     (T0, T1, .., T)
        Expr::Tuple(ExprTuple {
            attrs: _,
            paren_token: _,
            elems,
        }) => {
            for expr in elems.iter() {
                write_expr(buf, expr)?;
            }
        }

        // Single-Element Tuples:
        //     (T)
        Expr::Paren(ExprParen { expr, .. }) => {
            write_expr(buf, expr)?;
        }

        // Vec:
        //     [T, T, T]
        Expr::Array(ExprArray { elems, .. }) => {
            let len = elems.len();
            buf.write_u32::<LE>(len as u32)?;

            for expr in elems.iter() {
                write_expr(buf, expr)?;
            }
        }

        _ => expr.write(buf)?,
    }

    Ok(())
}

pub fn get_type(v: &syn::Path) -> (String, bool) {
    let syn::Path { segments, .. } = v;
    let is_enum = segments.len() == 2;
    let ty = {
        if is_enum {
            format!(
                "{}::{}",
                &segments[0].ident.to_string().as_str(),
                &segments[1].ident.to_string().as_str(),
            )
        } else {
            segments[0].ident.to_string()
        }
    };

    (ty, is_enum)
}
