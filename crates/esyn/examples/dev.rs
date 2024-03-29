use esyn::*;

fn main() {
    //syn_parse_str();
    //typ();

    //t1();
    //d1();
}

fn d1() {
    use syn::*;
    struct K {}

    impl<'ast> Visit<'ast> for K {
        fn visit_item_fn(&mut self, _i: &'ast ItemFn) {}
    }
}

fn t1() {
    #[derive(PartialEq, Debug, EsynDe, EsynSer, Default)]
    struct Tu8 {
        _u8: u8,
    }

    #[derive(PartialEq, Debug, EsynDe, EsynSer, Default)]
    struct Tu16 {
        _u16: u16,
    }

    let config = r#"
fn f1() {
    let a = Tu8 {
        _u8: 123,
    };
}

fn f2() {
    let a = Tu16 {
        _u16: 123,
    };
}
"#;

    let f1 = &EsynBuilder::new()
        .set_fn("f1")
        .set_let("a")
        .get_once::<Tu8>(config)
        .unwrap()
        .get();
    let f2 = &EsynBuilder::new()
        .set_fn("f2")
        .set_let("a")
        .get_once::<Tu16>(config)
        .unwrap()
        .get();

    let e = Esyn::new(config);
    e.init().unwrap();

    let f11 = &EsynBuilder::new()
        .set_fn("f1")
        .set_let("a")
        .get_once::<Tu8>(config)
        .unwrap()
        .get();
    let f21 = &EsynBuilder::new()
        .set_fn("f2")
        .set_let("a")
        .get::<Tu16>(&e)
        .unwrap()
        .get();

    assert_eq!(f1, f11);
    assert_eq!(f2, f21);
}

fn syn_parse_str() {
    let a: Wrap<Test> = syn::parse_str(
        "
Test {
    _u8: 1,
    _u16: 123,
}
",
    )
    .unwrap();

    assert_eq!(a.get_ref()._u8, 1);
}

#[derive(Debug, Default, EsynDe, EsynSer, PartialEq)]
struct Test {
    _u8: u8,
    _u16: u16,
}

#[derive(PartialEq, Debug, EsynDe, EsynSer, Default)]
struct S {
    _string: String,
    _u128: u128,
    _i128: i128,
    _s2: S2,
}

#[derive(PartialEq, Debug, EsynDe, EsynSer, Default)]
struct S2 {
    _u8: u8,
    _bool_default: bool,
    _vec: Vec<u8>,
}

#[derive(Debug, EsynDe, EsynSer, Default)]
enum EnumNamed {
    #[default]
    Unit1,
    A {
        name: String,
    },
}

#[derive(Debug, Default, EsynDe, EsynSer)]
struct StructUnit;

#[derive(Debug, Default, EsynDe, EsynSer)]
struct StructUnnamed2(u8, (i8, (u16, i16)));

//impl EsynPrint for StructUnnamed2 {
//    fn print(&self) -> TokenStream {
//        let iter = [self.0.to_tokens(), self.1.to_tokens()];
//
//        quote! {
//            StructUnnamed2(
//                #(#iter)*
//                          )
//        }
//    }
//}

#[derive(Debug, EsynDe, EsynSer, Default)]
enum Enum {
    #[default]
    Unit1,
    Unit2,
    Unit3,
    Unnamed(String),
    Unnamed2(String, char),
    Unnamed3(String, (u8, u16), S2),
    Named {
        _u8: u8,
    },
    Named2 {
        _u8: u8,
        _u16: u16,
    },
}

// #[alias(a = a.b.c)]
// c = 4;
fn typ() {
    let config = r#"
fn main() {
    let _bool = false;
    let _u8 = 8;
    let _vec = [1, 2, 3];
    let s = S {
        _s2: S2 {
            _u8: 7,
        },
        _u128: 340282366920938463463374607431768211455,
        _i128: -170141183460469231731687303715884105728,
        _string: "hello",
    };
    let struct_unnamed2 = StructUnnamed2(1, (2, (3, 4)));

    let enum_named = EnumNamed::A { name: "ABC" };
    let e = Enum::Named { _u8: 1 };
    let eu = Enum::Unnamed3(
        "unnamed",
        (8,16),
        S2 {
            _u8: 7,
        },
    );

    let struct_unit = StructUnit;

    ::alias(v1, s._s2._bool_default);
    ::alias(v2, s._s2);


    struct_unit = StructUnit;
    _bool = true;

    s._s2._u8 = 9;
    s._s2._bool_default = true;
    s._u128 = 14;
    s._s2._u8 = 9;

    v1 = false;
    v2._vec = [1, 2, 3, 4];
}
"#;

    let a = &EsynBuilder::new()
        .set_let("a")
        .get_once::<S>(config)
        .unwrap();
    let _ts = quote! {
        fn main() {
            let a = #a;
        }
    };

    println!("{_ts:#?}");
}

fn impls() {
    //impl DeRs<Expr> for Enum {
    //    fn de(ast: &Expr) -> Res<Self> {
    //        let var = match ast {
    //            // Unit
    //            Expr::Path(ExprPath {
    //                path: Path { segments, .. },
    //                ..
    //            })
    //            // Named
    //            | Expr::Struct(ExprStruct {
    //                path: Path { segments, .. },
    //                ..
    //            }) => {
    //                assert_eq!(segments.len(), 2);
    //
    //                let PathSegment { ident, .. } = &segments[1];
    //
    //                ident.to_string()
    //            }
    //
    //            // Unnamed
    //            Expr::Call(ExprCall { func, .. }) => {
    //                let Expr::Path(ExprPath { path:Path {  segments ,..},.. }) = func.as_ref()
    //                else {unreachable!()};
    //
    //                assert_eq!(segments.len(), 2);
    //
    //                let PathSegment { ident, .. } = &segments[1];
    //
    //                ident.to_string()
    //            }
    //            _ => unreachable!("{:#?}", ast),
    //        };
    //
    //        //dbg!(&var);
    //        //dbg!(ast);
    //        Ok(match var.as_str() {
    //            "Unit1" => Self::Unit1,
    //            "Unnamed" => {
    //                let Expr::Call(ExprCall { args, .. }) = ast
    //            else {unreachable!()};
    //
    //                let mut iter = args.iter();
    //                Self::Unnamed({ <String as DeRs<Expr>>::de({ &iter.next().unwrap() }).unwrap() })
    //            }
    //
    //            "Unnamed2" => {
    //                let Expr::Call(ExprCall { args,func, .. }) = ast
    //            else {unreachable!()};
    //
    //                let mut iter = args.iter();
    //                dbg!(&args);
    //                Self::Unnamed2(
    //                    { <String as DeRs<Expr>>::de({ &iter.next().unwrap() }).unwrap() },
    //                    { <char as DeRs<Expr>>::de({ &iter.next().unwrap() }).unwrap() },
    //                )
    //            }
    //            "Named" => {
    //                //dbg!(ast);
    //                let Expr::Struct(ExprStruct {
    //                    fields, ..
    //                }) = ast
    //                else { unreachable!() };
    //
    //                Self::Named {
    //                    _u8: get_struct_named_field(fields, "_u8").unwrap_or_default(),
    //                }
    //            }
    //
    //            _ => unreachable!(),
    //        })
    //    }
    //}

    //impl DeRs<Expr> for S2 {
    //    fn de(ast: &Expr) -> Res<Self> {
    //        let Expr::Struct(ExprStruct {
    //            fields, ..
    //        }) = ast
    //        else { unreachable!() };
    //
    //        Ok(Self {
    //            _u8: get_struct_named_field(fields, "_u8").unwrap_or_default(),
    //            _bool_default: get_struct_named_field(fields, "_bool_default").unwrap_or_default(),
    //            _vec: get_struct_named_field(fields, "_vec").unwrap_or_default(),
    //        })
    //    }
    //}
    //
    //impl DeRs<Expr> for S {
    //    fn de(ast: &Expr) -> Res<Self> {
    //        let Expr::Struct(ExprStruct {
    //            fields, ..
    //        }) = ast
    //        else { unreachable!() };
    //
    //        Ok(Self {
    //            _i128: get_struct_named_field(fields, "_i128").unwrap_or_default(),
    //            _u128: get_struct_named_field(fields, "_u128").unwrap_or_default(),
    //            _string: get_struct_named_field(fields, "_string").unwrap_or_default(),
    //            _s2: get_struct_named_field(fields, "_s2").unwrap_or_default(),
    //        })
    //    }
    //}

    //impl MutPath for S2 {
    //    fn mut_path(&mut self, iter: &mut std::slice::Iter<Ident>, ast: &syn::Expr) -> Res<()> {
    //        if let Some(i) = iter.next() {
    //            match i.to_string().as_str() {
    //                "_u8" => self._u8.mut_path(iter, ast)?,
    //                "_bool_default" => self._bool_default.mut_path(iter, ast)?,
    //                "_vec" => self._vec.mut_path(iter, ast)?,
    //                _ => unreachable!(),
    //            }
    //        } else {
    //            *self = <Self as DeRs<Expr>>::de(ast)?;
    //        }
    //
    //        Ok(())
    //    }
    //}
    //
    //impl MutPath for S {
    //    fn mut_path(&mut self, iter: &mut std::slice::Iter<Ident>, ast: &syn::Expr) -> Res<()> {
    //        if let Some(i) = iter.next() {
    //            match i.to_string().as_str() {
    //                "_string" => self._string.mut_path(iter, ast)?,
    //
    //                "_u128" => self._u128.mut_path(iter, ast)?,
    //                "_i128" => self._i128.mut_path(iter, ast)?,
    //                "_s2" => self._s2.mut_path(iter, ast)?,
    //                _ => unreachable!(),
    //            }
    //        } else {
    //            *self = <Self as DeRs<Expr>>::de(ast)?;
    //        }
    //
    //        Ok(())
    //    }
    //}

    //impl CustomSyntax for S2 {
    //    fn esyn_default() -> Self {
    //        Self {
    //            _u8: <u8 as CustomSyntax>::esyn_default(),
    //            _bool_default: <_bool_default as CustomSyntax>::esyn_default(),
    //            _vec: <Vec<u8> as CustomSyntax>::esyn_default(),
    //        }
    //    }
    //}

    //impl MutPath for StructUnnamed2 {
    //    fn mut_path(&mut self, iter: &mut std::slice::Iter<Ident>, ast: &syn::Expr) -> Res<()> {
    //        *self = <Self as DeRs<Expr>>::de(ast)?;
    //        Ok(())
    //    }
    //}

    //impl DeRs<Expr> for StructUnnamed2 {
    //    fn de(ast: &Expr) -> Res<Self> {
    //        // dbg!(ast);
    //
    //        let Expr::Call(ExprCall {
    //            func,
    //            args,
    //            ..
    //        }) = ast
    //        else {unreachable!()};
    //
    //        let Expr::Path(ExprPath {
    //            path: Path { segments, .. },
    //            ..
    //        }) = func.as_ref()
    //        else {unreachable!()};
    //
    //        assert_eq!(segments[0].ident, "StructUnnamed2");
    //
    //        Ok(Self(
    //            <u8 as DeRs<Expr>>::de(&args[0])?,
    //            <(i8, (u16, i16)) as DeRs<Expr>>::de(&args[1])?,
    //        ))
    //    }
    //}

    //impl MutPath for StructUnit {
    //    fn mut_path(&mut self, iter: &mut std::slice::Iter<Ident>, ast: &syn::Expr) -> Res<()> {
    //        *self = <Self as DeRs<Expr>>::de(ast)?;
    //        Ok(())
    //    }
    //}

    //impl DeRs<Expr> for StructUnit {
    //    fn de(ast: &Expr) -> Res<Self> {
    //        let Expr::Path(ExprPath {
    //            path: Path { segments, .. },
    //            ..
    //        }) = ast
    //        else { unreachable!() };
    //
    //        if segments[0].ident == "StructUnit" {
    //            Ok(Self)
    //        } else {
    //            todo!()
    //        }
    //    }
    //}

    //impl EsynPrint for StructUnnamed2 {
    //    fn print(&self) -> quote::__private::TokenStream {
    //        let a = <u8 as EsynPrint>::print(&self.0);
    //        let b = <(i8, (u16, i16)) as EsynPrint>::print(&self.1);
    //        quote! {
    //            StructUnnamed2(
    //                #a,
    //                #b,
    //            )
    //
    //        }
    //    }
    //}

    ////    _u8: u8,
    ////    _bool_default: bool,
    ////    _vec: Vec<u8>,
    //impl ToTokens for S2 {
    //    fn to_tokens(&self, tokens: &mut TokenStream) {
    //        for (i, segment) in self.segments.iter().enumerate() {
    //            if i > 0 || self.global {
    //                // Double colon `::`
    //                //tokens.append(Punct::new(':', Spacing::Joint));
    //                //tokens.append(Punct::new(':', Spacing::Alone));
    //            }
    //            segment.to_tokens(tokens);
    //        }
    //    }
    //}
}
