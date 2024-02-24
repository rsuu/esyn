use esyn::*;

#[derive(Debug, PartialEq, EsynDe)]
struct BoxStruct {
    _box: Option<Box<Self>>,
}

#[derive(Debug, PartialEq, EsynDe)]
struct StructEmpty {}

#[derive(Debug, PartialEq, EsynDe)]
struct StructEmpty2();

#[derive(Debug, PartialEq, EsynDe)]
struct StructUnit;

#[derive(Debug, PartialEq, EsynDe)]
struct StructUnnamed(u8, String);

#[derive(Debug, PartialEq, EsynDe)]
struct StructUnnamed2(u8, (i8, (u16, i16)));

#[derive(Debug, PartialEq, EsynDe)]
struct StructUnnamed3(u8, u8);

#[derive(Debug, PartialEq, EsynDe)]
struct StructNamed<T> {
    _u8: u8,
    _string: String,
    _vec_t: Vec<T>,
}

//#[test]
//fn test_struct_unnamed() {
//    #[derive(Debug, PartialEq,  EsynDe)]
//    struct Test {
//        _un2: StructUnnamed2,
//        _opt_self: Option<Box<Self>>,
//    }
//
//    let config = r#"
//fn main() {
//    let a = Test {
//        _un2: StructUnnamed2(1, (2, (3, 4))),
//        _opt_self: Some(
//            Test {
//                _un2: StructUnnamed2(5, (6, (7, 8))),
//                _opt_self: (),
//            },
//        ),
//    };
//}
//"#;
//
//

//
//    let a = &esyn.get_value::<Test>("main", "a").unwrap();
//    assert_eq!(
//        a,
//        &Test {
//            _un2: StructUnnamed2(1, (2, (3, 4))),
//            _opt_self: Some(Box::new(Test {
//                _un2: StructUnnamed2(5, (6, (7, 8))),
//                _opt_self: (),
//            }))
//        }
//    );
//}

#[test]
fn test_struct_unit() {
    #[derive(Debug, PartialEq, EsynDe)]
    struct Test {
        _struct_unit: StructUnit,
        _default: StructUnit,
    }

    let config = r#"
fn main() {
    let a = Test {
        _struct_unit: StructUnit,
        //_default
    };
}
"#;

    assert_eq!(
        &EsynBuilder::new()
            .set_let("a")
            .get_once::<Test>(config)
            .unwrap(),
        &Test {
            _struct_unit: StructUnit,
            _default: StructUnit
        }
    );
}

#[test]
fn test_struct() {
    #[derive(Debug, PartialEq, EsynDe)]
    struct Test {
        _struct_empty: StructEmpty,
        _struct_empty2: StructEmpty2,
        _struct_unnamed: StructUnnamed,
        _struct_tuple: StructUnnamed2,
        _struct_tuple3: StructUnnamed3,
        _struct_unit: StructUnit,
        //_box: BoxStruct,
    }

    let config = r#"
fn main() {
    let a = Test {
        _struct_empty: StructEmpty {},
        _struct_unnamed: StructUnnamed(9, "abcd"),
        _struct_tuple: StructUnnamed2(1, (2, (3, 4))),
        _struct_tuple3: StructUnnamed3(1, 2),
    };
}
"#;

    assert_eq!(
        &EsynBuilder::new()
            .set_let("a")
            .get_once::<Test>(config)
            .unwrap(),
        &Test {
            _struct_empty: StructEmpty {},
            _struct_empty2: StructEmpty2(),
            _struct_unnamed: StructUnnamed(9, "abcd".to_string()),
            _struct_tuple: StructUnnamed2(1, (2, (3, 4))),
            _struct_tuple3: StructUnnamed3(1, 2),
            _struct_unit: StructUnit,
        }
    );
}
