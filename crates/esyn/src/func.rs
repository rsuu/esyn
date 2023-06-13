use crate::{MyErr, ParseExpr, Res};
use byteorder::{WriteBytesExt, LE};
use std::{collections::HashMap, io::Write};
use syn::*;

#[derive(Debug, Clone)]
pub struct Func {
    pub ident: String,
    pub block: Block,
    pub fmap: FuncMap,
}

#[derive(Debug, Clone)]
pub struct FuncMap {
    pub map_alias: HashMap<String, (String, String)>,
    pub map: HashMap<String, ExprStruct>,
}

#[derive(Debug, PartialEq)]
pub enum ValType {
    Enum,
    Struct,
    Fn,
}

impl Func {
    pub fn new(ident: String, block: Block) -> Self {
        Self {
            ident,
            block,
            fmap: FuncMap::new(),
        }
    }

    pub fn from_item(item: &ItemFn) -> Self {
        let ItemFn {
            sig: Signature { ident, .. },
            block,
            ..
        } = item;

        Self::new(ident.to_string(), block.as_ref().clone())
    }
}

impl FuncMap {
    pub fn new() -> Self {
        Self {
            map_alias: HashMap::new(),
            map: HashMap::new(),
        }
    }

    pub fn match_closure(&self, v: &ExprClosure) -> Res<()> {
        //dbg!(&v);
        let ExprClosure { inputs, .. } = v;
        let Pat::Path(
            ExprPath{ path, ..
        }) = &inputs[0]
        else { return Err(MyErr::Todo); };

        let (name, vty) = get_type(path);
        if vty != ValType::Fn {
            return Ok(());
        }

        // ?
        match name.as_str() {
            _ => {}
        }

        Ok(())
    }

    pub fn match_fn(&mut self, v: &ExprCall) -> Res<()> {
        //dbg!(&v);
        let ExprCall { func, args, .. } = v;

        let Expr::Path(
            ExprPath {
                path: syn::Path {
                    leading_colon: Some(..),
                    segments,
                }, ..
            }
        ) = func.as_ref()
        else { unreachable!() };

        match segments[0].ident.to_string().as_str() {
            "alias_path" => self.fn_alias_path(args),

            _ => return Err(MyErr::Unsupported),
        }

        Ok(())
    }

    pub fn fn_alias_path(&mut self, args: &syn::punctuated::Punctuated<Expr, token::Comma>) {
        let (
            Expr::Path(ExprPath {
                path: syn::Path { segments, .. },
                ..
            }),
            field,
        ) = (&args[0], &args[1])
        else { unreachable!() };

        let alias = segments[0].ident.to_string();

        let (val, ref mut path) = get_expr_path(&field);
        if val.is_empty() || path.is_empty() {
            return;
        }

        path.reverse();
        //dbg!(&path);

        let path = path.join(".");
        self.map_alias.insert(alias, (val, path));
    }

    pub fn parse_let(&mut self, v: &Local, ty: &str, ast1: &ExprStruct) {
        let Local {
            pat: Pat::Ident(PatIdent { ident, .. }),
            init: Some(LocalInit {
                expr, diverge: None, ..
            }),
            ..
        } = v
        else { return; };

        let Expr::Struct(ast2) = expr.as_ref()
        else { unreachable!() };

        let ExprStruct { path, .. } = ast2;
        let (ty2, ..) = get_type(&path);

        //dbg!(ty, &ty2);
        if ty != ty2.as_str() {
            return;
        }

        let mut ast1 = ast1.clone();
        merge_struct(&mut ast1, ast2);

        let val_name = ident.to_string();
        self.map.insert(val_name, ast1);
    }

    pub fn parse_stmt_expr_assign(&mut self, v: &ExprAssign) {
        let ExprAssign { left, right, .. } = v;
        //dbg!(&left);
        //dbg!(&right);

        let l = &mut left.clone();
        let mut tmp = vec![];

        while let Expr::Field(ExprField {
            member: Member::Named(i),
            base,
            ..
        }) = l.as_ref()
        {
            tmp.push(i.to_string());
            *l = base.clone();
        }

        let Expr::Path(
            ExprPath {
                path: syn::Path { segments, .. },
                ..
            }
        ) = l.as_ref()
        else { unreachable!() };
        let val_name = segments[0].ident.to_string();
        let alias = self.map_alias.get(val_name.as_str());

        //dbg!(&path, &right);
        //dbg!(&val_name);
        //dbg!(&map_alias);

        // check map_alias
        if let Some((val, part1)) = alias {
            //dbg!(&val, &part1);

            if let Some(v) = self.map.get_mut(val.as_str()) {
                tmp.reverse();
                let path = format!(".{}.{}", part1, tmp.join("."));

                update_struct(v, &[(path, right.as_ref().clone())]);
            }

        // check map
        } else if let Some(v) = self.map.get_mut(val_name.as_str()) {
            tmp.reverse();
            let path = format!(".{}", tmp.join("."));

            update_struct(v, &[(path, right.as_ref().clone())]);
        }
    }
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

                let (.., vty) = get_type(path);
                if vty == ValType::Enum {
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

fn merge_struct(left: &mut ExprStruct, right: &ExprStruct) {
    let vec = trans(right);
    //dbg!(&vec);

    update_struct(left, &vec);
}

fn update_struct(v: &mut ExprStruct, vec: &[(String, Expr)]) {
    inner_update_struct(v, vec, &mut "".to_string());
}

fn inner_update_struct(v: &mut ExprStruct, vec: &[(String, Expr)], tmp: &mut String) {
    for FieldValue { member, expr, .. } in v.fields.iter_mut() {
        let Member::Named(i) = member
        else { continue; };

        let mut path_now = format!("{}.{}", tmp, &i.to_string());
        //dbg!(&path_now);

        match expr {
            Expr::Struct(next) => {
                let ExprStruct { path, .. } = next;
                let (.., vty) = get_type(path);

                // enum
                if vty == ValType::Enum {
                    //dbg!(&p, &ty, is_enum);

                    for (path, expr2) in vec.iter() {
                        if path.as_str() != path_now.as_str() {
                            //dbg!(&path, &p);
                            continue;
                        }

                        *expr = expr2.clone();
                    }

                    continue;
                }

                // struct
                for (path, expr2) in vec.iter() {
                    if path.as_str() != path_now.as_str() {
                        //dbg!(&path, &path_now);
                        continue;
                    }

                    let Expr::Struct(right) = expr2
                    else { continue; };

                    //dbg!(&ast3);

                    let ExprStruct { path, .. } = right;

                    let (.., vty) = get_type(path);

                    if vty == ValType::Enum {
                        //dbg!(&p, &ty, is_enum);
                        unimplemented!();
                    }
                    let vec_assign = trans(right);
                    //dbg!(&vec_assign);
                    update_struct(next, &vec_assign);
                }

                inner_update_struct(next, vec, &mut path_now);
            }

            _ => {
                for (path, expr2) in vec.iter() {
                    if path.as_str() != path_now.as_str() {
                        //dbg!(&path, &p);
                        continue;
                    }

                    //dbg!(&expr2);
                    *expr = expr2.clone();

                    //dbg!(&path, &p, &expr2, &expr);
                }
            }
        }
    }
}

pub fn write_expr<W: Write>(w: &mut W, expr: &Expr) -> Res<()> {
    //dbg!(&expr);

    match expr {
        // Empty:
        //     Struct {}
        //     Enum {}
        Expr::Struct(ExprStruct { fields, .. }) if fields.len() == 0 => {
            w.write_u8(0)?;
        }

        // Named:
        //     StructNamed { ... }
        //     Enum::Named { ... }
        Expr::Struct(ExprStruct { path, fields, .. }) => {
            let (ty, vty) = get_type(path);
            if vty == ValType::Enum {
                // write type name
                ty.as_str().write(w)?;
            } else {
                // struct
                w.write_u8(1)?;
            }

            // NOTE: sort fields
            let mut tmp = vec![];
            for FieldValue { expr, member, .. } in fields.iter() {
                let Member::Named(i) = member
                else { unreachable!() };
                //dbg!(&i);
                tmp.push((expr, i.to_string()));
            }
            tmp.sort_by(|a, b| a.1.cmp(&b.1));

            for (expr, ..) in tmp.iter() {
                write_expr(w, expr)?;
            }
        }

        // Unnamed:
        //     Struct( ... )
        //     Enum  ( ... )
        Expr::Call(ExprCall { func, args, .. }) => {
            let Expr::Path(
                ExprPath { path, .. }
            ) = func.as_ref()
            else { unreachable!() };

            let (ty, vty) = get_type(path);
            match (ty.as_str(), vty) {
                // Enum( ... )
                (.., ValType::Enum) | ("Some", ValType::Struct) => ty.as_str().write(w)?,

                // Enum::Var( ... )
                _ => w.write_u8(1)?,
            }

            for expr in args.iter() {
                //dbg!(&expr);
                write_expr(w, expr)?;
            }
        }

        // Unit:
        //     Enum::Unit
        //     StructUnit;
        Expr::Path(ExprPath { path, .. }) => {
            let (ty, vty) = get_type(path);
            //dbg!(&ty);

            match (ty.as_str(), vty) {
                // Enum::Unit
                (.., ValType::Enum) => {
                    ty.as_str().write(w)?;
                }

                // None
                ("None", ValType::Struct) => {
                    // empty string
                    w.write_u8(0)?;
                }

                // StructUnit
                _ => {
                    w.write_u8(1)?;
                }
            }
        }

        // Tuple:
        //     (T0, T1, .., T)
        Expr::Tuple(ExprTuple { elems, .. }) => {
            for expr in elems.iter() {
                write_expr(w, expr)?;
            }
        }

        // Single-Element Tuple:
        //     (T)
        Expr::Paren(ExprParen { expr, .. }) => {
            write_expr(w, expr)?;
        }

        // Vec:
        //     [T, T, T]
        Expr::Array(ExprArray { elems, .. }) => {
            let len = elems.len();

            if len == 0 {
                w.write_u8(0)?;
            } else {
                w.write_u8(1)?;
                w.write_u32::<LE>(len as u32)?;

                for expr in elems.iter() {
                    write_expr(w, expr)?;
                }
            }
        }

        // e.g. [ (K, V), .. ] as HashMap
        //      ( T, T, T, T ) as Ipv4Addr
        Expr::Cast(ExprCast { expr, ty, .. }) => {
            let vec1 = &[
                "HashMap", "BTreeMap", //
            ];
            let vec2 = &[
                "Ipv4Addr", "Ipv6Addr", //
            ];

            let Type::Path(
                TypePath { path, .. }
            ) = ty.as_ref()
            else {
                return Err(MyErr::Unsupported);
            };
            let (ty, ..) = get_type(path);

            if vec1.contains(&ty.as_str()) {
                ty.as_str().write(w)?;
            } else if vec2.contains(&ty.as_str()) {
            } else {
                return Err(MyErr::UnType(ty));
            }

            write_expr(w, expr)?;
        }

        _ => expr.write(w)?,
    }

    Ok(())
}

// e.g. `let a = a.b.c;`
// var : a
// path: a.b.c
fn get_expr_path(f: &Expr) -> (String, Vec<String>) {
    let val;
    let mut path = vec![];
    let mut f = f.clone();
    loop {
        match f {
            Expr::Field(ExprField {
                member: Member::Named(i),
                base,
                ..
            }) => {
                path.push(i.to_string());

                f = base.as_ref().clone();
            }

            Expr::Path(p) => {
                val = p.path.get_ident().unwrap().to_string();
                break;
            }

            _ => unreachable!(),
        }
    }

    (val, path)
}

fn get_type(v: &syn::Path) -> (String, ValType) {
    //dbg!(v);
    let syn::Path {
        segments,
        leading_colon,
        ..
    } = v;

    // if start_with("::")
    if leading_colon.is_some() {
        let v = segments
            .into_iter()
            .map(|f| f.ident.to_string())
            .collect::<Vec<_>>();
        let v = v.join("");

        return (v, ValType::Fn);
    }

    // Enum OR Struct
    let is_enum = segments.len() == 2;
    if is_enum {
        // e.g. Enum::A
        let v = format!(
            "{}::{}",
            &segments[0].ident.to_string().as_str(),
            &segments[1].ident.to_string().as_str(),
        );

        return (v, ValType::Enum);
    } else {
        let v = segments[0].ident.to_string();

        return (v, ValType::Struct);
    }
}
