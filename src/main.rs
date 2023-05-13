use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt, LE};
use esyn::{AutoDefault, Esyn, EsynBytes, FromEsyn, MyErr, ParseBytes, ParseExpr, Res, Zeroed};
use std::{
    default,
    fs::File,
    io::{Cursor, Read, Write},
    mem::{self, MaybeUninit},
    path::Path,
    ptr::addr_of_mut,
};
use syn::*;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub val: Option<Expr>,
}

#[derive(Debug)]
struct Config {
    _01: u8,
    _02: u16,
    _03: u32,
    _04: u64,
    _05: usize,
    _06: i8,
    _07: i16,
    _08: i32,
    _09: i64,
    _10: f32,
    _11: f64,
    _12: bool,
    _13: char,
    _14: String,
    //_15: Cow<'a, str>,
    _16: Vec<u8>,
    //_17:         Cow<'a, [T]> ,
    //_18:        HashMap<K, V> ,
    //_19:       BTreeMap<K, V> ,
    //_20:           HashSet<T> ,
    //_21:          BTreeSet<T> ,
    //_22:             Range<T> ,
    _23: Option<bool>,
    _24: (),
    //_25:                  (T) ,
    //_26:               (T, T) ,
    //_27:           (T, .., T) ,
    //_28:                enums ,
    //_29:             AtomicU8 ,
    //_30:             AtomicI8 ,
    //_31:            AtomicU16 ,
    //_32:            AtomicI16 ,
    //_33:            AtomicU32 ,
    //_34:            AtomicI32 ,
    //_35:            AtomicU64 ,
    //_36:            AtomicI64 ,
    //_37:           NonZeroU32 ,
    _38: std::net::Ipv4Addr,
    _39: std::net::Ipv6Addr,
    _40: std::net::IpAddr,
    _41: std::time::Duration,
    _42: std::time::SystemTime,
    //_43:           uuid::Uuid ,
}

fn main() -> Res<()> {
    let ts = from_file("./tests/test.rs")?;
    let mut items = ts.items.into_iter();

    //               (name    value )
    let mut buf: Vec<(String, String)> = vec![];
    for item in items.as_mut_slice() {
        //dbg!(&item);

        let Item::Fn(
        ItemFn {
            ref mut block,
            sig: Signature { ref mut ident, .. },
            ..
        }) = item
        else {unreachable!()};

        let Block { ref mut stmts, .. } = block.as_mut()
        else {unreachable!()};

        parse_stmts(stmts)?;
    }

    Ok(())
}

pub fn from_file(path: impl AsRef<Path>) -> Res<syn::File> {
    let mut f = File::open(path)?;
    let mut buf = "".to_string();
    f.read_to_string(&mut buf)?;
    let res: syn::File = syn::parse_str(&buf)?;
    //let k: TokenStream = syn::parse_str(&buf)?;
    //dbg!(k);

    Ok(res)
}

fn parse_stmts(stmts: &mut [Stmt]) -> Res<()> {
    let mut struct_list = vec![];
    let mut expr_list = vec![];
    for s in stmts.iter_mut() {
        //dbg!(&s);

        match s {
            Stmt::Expr(Expr::Struct(s), ..) => {
                struct_list.push(s.clone());
            }

            Stmt::Expr(Expr::Assign(expr), ..) => {
                let expr = parse_expr_assign_dot(&expr);
                //dbg!(&expr);
                expr_list.push(expr);
            }

            // // e.g. feed( ... );
            // Stmt::Expr(Expr::Call(ExprCall { func, args, .. }), ..) => {
            //     let Expr::Path(ExprPath {
            //             path: syn::Path {
            //                 segments, ..
            //             },
            //             ..
            //         }) = func.as_ref()
            //         else {unreachable!()};
            //
            //     let ident = segments[0].ident.to_string();
            //     dbg!(ident);
            //     //self.parse_fn(ident.as_str(), args.iter());
            // }
            _ => {}
        }
    }

    //dbg!(&struct_list);
    //dbg!(&expr_list);

    for (list, val) in expr_list.iter() {
        for s in struct_list.iter_mut() {
            update_struct(s, list.as_slice(), val);
            //dbg!(s);
        }
    }

    // generate struct from esyn bytes
    for s in struct_list.iter() {
        //dbg!(s);
        if get_struct_name(s).as_str() != "Config" {
            continue;
        }

        //esyn_bytes_without_macro(s)?;
        esyn_bytes_macro(s)?;
    }

    Ok(())
}

fn esyn_bytes_macro(s: &ExprStruct) -> Res<()> {
    #[derive(Debug)]
    pub struct Ka {
        a: std::sync::Arc<u8>,
        b: u32,
        c: [u8; 3],
        d: Option<Box<Ka>>,
    }
    let tmp = Ka {
        a: std::sync::Arc::new(2),
        b: 1234,
        c: [0, 0, 0],
        d: None,
    };

    let config = unsafe {
        let mut uninit = MaybeUninit::<Ka>::uninit();
        let ptr = uninit.as_mut_ptr();

        addr_of_mut!((*ptr).a).write(std::sync::Arc::new(1));
        addr_of_mut!((*ptr).b).write(123);
        addr_of_mut!((*ptr).c).write([1, 2, 3]);
        addr_of_mut!((*ptr).d).write(Some(Box::new(tmp)));

        uninit.assume_init()
    };
    dbg!(config);

    // TODO: feat: struct A<T> {}
    #[derive(Debug, Esyn, AutoDefault)]
    struct A {
        a: i8,
        s: String,
        sb: B,
        v1: u8,
    }
    #[derive(Debug, Esyn, AutoDefault)]
    struct B {
        sc: C,
        v2: u8,
    }
    #[derive(Debug, Esyn, AutoDefault)]
    struct C {
        v1: u8,
        v2: u8,
    }

    let a = {
        let mut fields = get_fields(s);
        dbg!(&fields);

        let buf = parse_struct(s)?;
        let mut buf = Cursor::new(buf);

        A::from_esyn_default(buf, &mut fields)?
    };
    dbg!(&a);

    let a = {
        let mut fields = get_fields(s);
        dbg!(&fields);

        let buf = parse_struct(s)?;
        let mut buf = Cursor::new(buf);

        unsafe { A::from_esyn_uninit(buf, &mut fields)? }
    };
    dbg!(&a);

    let a = {
        let mut fields = get_fields(s);
        dbg!(&fields);

        let buf = parse_struct(s)?;
        let mut buf = Cursor::new(buf);

        unsafe { A::from_esyn(buf, &mut fields)? }
    };
    dbg!(&a);

    Ok(())
}

fn esyn_bytes_without_macro(s: &ExprStruct) -> Res<()> {
    #[derive(Debug)]
    struct A {
        a: u8,
        s: String,
        sb: B,
        c: Option<String>,
    }
    #[derive(Debug)]
    struct B {
        sc: C,
    }
    #[derive(Debug)]
    struct C {
        v1: u8,
        v2: u8,
    }

    let buf = parse_struct(s)?;
    let mut buf = Cursor::new(buf);

    impl EsynBytes for C {
        fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
            Ok(Self {
                v1: u8::from_bytes(&mut buf)?,
                v2: u8::from_bytes(&mut buf)?,
            })
        }
    }

    impl EsynBytes for B {
        fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
            Ok(Self {
                sc: C::from_bytes(&mut buf)?,
            })
        }
    }

    impl EsynBytes for A {
        fn from_bytes(mut buf: impl ParseBytes) -> Res<Self> {
            Ok(Self {
                a: <u8 as EsynBytes>::from_bytes(&mut buf)?,
                s: String::from_bytes(&mut buf)?,
                sb: B::from_bytes(&mut buf)?,
                c: Option::from_bytes(&mut buf)?,
            })
        }
    }

    let a = A::from_bytes(buf)?;

    dbg!(a);

    Ok(())
}

fn get_struct_name(s: &ExprStruct) -> String {
    let ExprStruct {
        path:syn::Path { segments,.. },
        ..
    } = s else {unreachable!()};
    let PathSegment {
        ident,
        arguments:PathArguments::None
    } = &segments[0] else {unreachable!()};

    ident.to_string()
}

fn get_fields(s: &ExprStruct) -> Vec<String> {
    let mut buf = vec![];

    for f in s.fields.iter() {
        //dbg!(&f);

        let FieldValue {
                member: Member::Named(i),
                colon_token: Some(token::Colon { .. }),
                expr,
                ..
            } = f else { todo!() };

        buf.push(i.to_string());
    }

    buf
}

fn parse_struct(s: &ExprStruct) -> Res<Vec<u8>> {
    fn inner(s: &ExprStruct, buf: &mut Vec<u8>, id: &mut usize) -> Res<()> {
        for f in s.fields.iter() {
            //dbg!(&f);

            let FieldValue {
                member: Member::Named(i),
                colon_token: Some(token::Colon { .. }),
                expr,
                ..
            } = f else { continue; };

            match expr {
                Expr::Struct(s) => inner(s, buf, id)?,

                Expr::Call(v) => {
                    //dbg!(&v);
                    v.write(buf)?;
                    *id += 1;
                }

                Expr::Path(v) => {
                    v.write(buf)?;
                    *id += 1;
                }

                Expr::Lit(v) => {
                    dbg!(&i.to_string(), &id);
                    v.write(buf)?;
                    *id += 1;
                }

                _ => {
                    continue;
                }
            }
        }

        Ok(())
    }

    let mut id = 0;
    let mut buf = vec![];
    inner(s, &mut buf, &mut id);

    Ok(buf)
}

// BUG:
fn update_struct(s: &mut ExprStruct, list: &[String], val: &Expr) {
    fn check(list: &[String], depth: &mut usize, buf: &mut Vec<String>, i: &Ident) -> bool {
        // let now_fields = vec![]
        if *depth < list.len() && list[*depth].as_str() == i.to_string().as_str() {
            //dbg!(&buf, &list);
            *depth += 1;
            buf.push(i.to_string());

            true
        } else {
            false
        }
    }

    fn inner(
        s: &mut ExprStruct,
        list: &[String],
        depth: &mut usize,
        buf: &mut Vec<String>,
        val: &Expr,
    ) {
        for (i, f) in s.fields.iter_mut().enumerate() {
            //dbg!(&f);

            let FieldValue {
                member: Member::Named(i),
                colon_token: Some(token::Colon { .. }),
                expr,
                ..
            } = f else { continue; };

            match expr {
                Expr::Struct(s) => {
                    if check(list, depth, buf, i) {

                        inner(s, list, depth, buf, val);
                    }
                }

                Expr::Call(_) //.
                    | Expr::Lit(_)
                    | Expr::Path(_) => {
                    check(list, depth, buf, i);
                }

                _ => {
                    continue;
                }
            }

            // TODO: if !is_exitis { ... }

            if *depth == list.len()
                && buf.as_slice() == &list[1..]
                && i.to_string().as_str() == list.last().unwrap().as_str()
            {
                //dbg!(&expr, &buf, &list);
                *expr = val.clone();

                // NOTE: Make ensure that we never duplicate values with identical names again.
                buf.clear();
            }
        }
    }

    // skip first here if true.
    if list[0].as_str() != get_struct_name(s) {
        return;
    }

    //dbg!(list);
    let mut depth = 1;
    let mut buf = vec![];
    inner(s, list, &mut depth, &mut buf, val);
}

fn parse_expr_assign_dot(expr: &ExprAssign) -> (Vec<String>, Expr) {
    let ExprAssign {
   left,
   eq_token,
   right,
   ..
    } = expr else {unreachable!()};
    //dbg!(&left);

    let tmp = &mut left.clone();
    let mut buf = vec![];
    loop {
        match tmp.as_ref() {
            Expr::Field(ExprField {
                attrs,
                base,
                dot_token,
                member: Member::Named(i),
            }) => {
                buf.push(i.to_string());
                *tmp = base.clone();
            }

            Expr::Path(ExprPath {
                path: syn::Path { segments, .. },
                ..
            }) => {
                let PathSegment { ident, arguments:PathArguments::None} = &segments[0] else {unreachable!()};
                buf.push(ident.to_string());
                break;
            }

            _ => {
                break;
            }
        }
    }

    buf.reverse();

    (buf, *right.clone())
}

//    impl<'a> Iterator for LinearContentIterator<'a> {
//    type Item = &'a mut String;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let foo = self.stack.pop()?;
//
//        self.stack.extend(foo.childs.iter_mut().rev());
//
//        Some(&mut foo.description)
//    }
//}
//

mod test {
    use super::*;

    #[test]
    fn test_parse() {
        #[derive(Default)]
        struct A {
            a: u8,
            b: B,
        }
        struct B {
            b: String,
        }

        impl Default for A {
            fn default() -> Self {
                Self {
                    a: 0,
                    b: B { b: "".to_string() },
                }
            }
        }

        let mut a = A::null();
        let c = A::default();
        let expr: ExprLit = parse_quote!("string");

        a.b.b = expr.parse_string().unwrap();

        assert_eq!(a.b.b.as_str(), "string");
    }
}
