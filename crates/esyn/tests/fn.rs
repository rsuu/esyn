use esyn::*;

#[derive(Debug, PartialEq, EsynDe)]
struct Test {
    _u8: u8,
    _u16: u16,
}

#[derive(Debug, PartialEq, EsynDe)]
enum Enum {
    A { _u8: u8, _bool: bool },
}

#[derive(Debug, PartialEq, EsynDe)]
struct Test2 {
    l1: L1,
    _s1: S1,
}

#[derive(Debug, PartialEq, EsynDe)]
struct S1 {
    _u8: u8,
}

#[derive(Debug, PartialEq, EsynDe)]
struct L1 {
    val1: u8,
    l2: L2,
}

#[derive(Debug, PartialEq, EsynDe)]
struct L2 {
    val2: u8,
    l3: L3,
}

#[derive(Debug, PartialEq, EsynDe)]
struct L3 {
    val3: u8,
    l4: L4,
}

#[derive(Debug, PartialEq, EsynDe)]
struct L4 {
    val4: u8,
}

// TODO: rewrite
#[test]
fn test_fn_alias_path() {
    let config = r#"
fn main() {
    let a = Test2 {};

    ::alias(_alias1, a.l1.l2.l3.l4);
    ::alias(_alias2, a.l1.l2);
    ::alias(_alias3, a.l1.val1);
    crate::test( 1,2,3 );

    _alias1.val4 = 4;

    _alias2.val2 = 2;
    _alias2.l3.val3 = 3;

    _alias3 = 1;

    a._s1 = S1 {
        _u8: 4,
    };
}
"#;

    assert_eq!(
        &EsynBuilder::new()
            .set_fn("main")
            .set_let("a")
            .get_once::<Test2>(config)
            .unwrap(),
        &Test2 {
            _s1: S1 { _u8: 4 },
            l1: L1 {
                val1: 1,
                l2: L2 {
                    val2: 2,
                    l3: L3 {
                        val3: 3,
                        l4: L4 { val4: 4 },
                    },
                },
            },
        }
    );
}

#[test]
fn test_fn_alias_path_exec_with() {
    let config = r#"
fn main() {
    let a = Test2 {};

    ::alias(_alias1, a.l1.l2.l3.l4);
    ::alias(_alias2, a.l1.l2);
    ::alias(_alias3, a.l1.val1);

    _alias1.val4 = 4;

    _alias2.val2 = 2;
    _alias2.l3.val3 = 3;

    _alias3 = 1;

    crate::test(123);
    a._s1 = S1 {
        _u8: 4,
    };

}
"#;

    assert_eq!(
        &EsynBuilder::new()
            .set_fn("main")
            .set_let("a")
            .get_once::<Test2>(config)
            .unwrap(),
        &Test2 {
            _s1: S1 { _u8: 4 },
            l1: L1 {
                val1: 1,
                l2: L2 {
                    val2: 2,
                    l3: L3 {
                        val3: 3,
                        l4: L4 { val4: 4 },
                    },
                },
            },
        }
    );
}

#[test]
fn test_fn_return() {
    let config = r#"
// return ()
fn main() {
    let a = Test {
        _u8: 8,
        _u16: 16,
    };
}

fn return_any_struct_named() -> Res {
    Res {
        _u16: 16,
        _u8: 8,
    }
}

fn return_any_enum_named() -> Res {
    Res::A {
        _u8: 8,
        _bool: false,
    }
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    assert_eq!(
        &EsynBuilder::new()
            .set_fn("return_any_struct_named")
            .flag_res()
            .get::<Test>(&esyn)
            .unwrap(),
        &Test { _u16: 16, _u8: 8 }
    );

    assert_eq!(
        &EsynBuilder::new()
            .set_fn("return_any_enum_named")
            .flag_res()
            .get::<Enum>(&esyn)
            .unwrap(),
        &Enum::A {
            _u8: 8,
            _bool: false,
        }
    );

    assert_eq!(
        &EsynBuilder::new()
            .set_fn("return_any_enum_named")
            .flag_res()
            .get_once::<Enum>(config)
            .unwrap(),
        &Enum::A {
            _u8: 8,
            _bool: false,
        }
    );
}
