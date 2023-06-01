use esyn::{Esyn, EsynDe};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructEmpty {}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructEmpty2();

//#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnit;

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed(u8, String);

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed2(u8, (i8, (u16, (i16))));

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed3(u8, u8);

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructNamed<T> {
    _u8: u8,
    _string: String,
    _vec_t: Vec<T>,
}

// FIXME:
#[derive(Debug, PartialEq, Default, EsynDe)]
struct BoxStruct {
    // ?
    // consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`test`)
    //_box: Option<Box<BoxStruct>>,
}

//enum EnumEmpty {}

#[derive(Debug, PartialEq, Default, EsynDe)]
enum EnumUnit {
    #[default]
    Unit1,
    Unit2,
}

#[derive(Debug, PartialEq, EsynDe)]
enum EnumUnnamed {
    Unnamed(u8, i32, String),
}

impl Default for EnumUnnamed {
    fn default() -> Self {
        Self::Unnamed(0, 0, "".to_string())
    }
}

#[derive(Debug, PartialEq, EsynDe)]
enum EnumNamed {
    Named { _u8: u8, _i32: i32, _string: String },
}

impl Default for EnumNamed {
    fn default() -> Self {
        Self::Named {
            _u8: 0,
            _i32: 0,
            _string: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Default, EsynDe)]
enum Enum {
    #[default]
    Unit1,
    Unit2,
    Unit3,
    Unnamed(String),
    Unnamed2(String, char, StructNamed<u8>),
    Named {
        _u8: u8,
    },
    Named2 {
        _u8: u8,
        _u16: u16,
        _struct_named: StructNamed<char>,
    },
}

#[test]
fn test_example() {
    #[derive(Debug, PartialEq, EsynDe)]
    enum Map {
        Any(String),
        Named { a: u8, _color: Color },
        Down,
        Up,
    }

    impl Default for Map {
        fn default() -> Self {
            Map::Up
        }
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Window {
        borderless: bool,
        topmost: bool,
        color: Color,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Color {
        bg: u8,
        fg: u8,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        id: u64,
        name: String,
        invert_mouse: bool,
        font: Option<String>,
        _opt_u8: Option<u8>,
        _opt_enum: Option<Color>,
        _tuple1: String,
        _tuple2: (u32, i16),
        _tuple3: (u8, u32),
        _tuple4: (u8, (bool, String)),
        _tuple5: (u8, (bool, String, (u8, u8))),
        window: Window,
        map: Map,
        map2: Map,
        map3: Map,
    }

    let config = r#"
fn main() {
    let a = Config {
        id: 123456789123,
        name: "hi",
        map: Map::Down,
        map2: Map::Any("llll"),
        map3: Map::Named {
            a: 1,
            _color: Color { fg: 32, bg: 12 },
        },
        invert_mouse: true,
        font: Some("abc"),
        _tuple1: (("hi")),
        _tuple2: (123456789, -12345),
        _tuple3: (8, 123456789),
        _tuple4: (8, (true, "_tuple4")),
        _tuple5: (8, (true, "_tuple5", (1, 2),),),
        _opt_enum: None,
        window: Window {
            borderless: true,
            topmost: false,
        },
        _opt_u8: Some(56),
    };

    a.window.color = Color {
        bg:13,
        fg:12,
    };
}
"#;
    let esyn = esyn::Esyn::new(&config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();

    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            id: 123456789123,
            name: "hi".to_string(),
            invert_mouse: true,
            font: Some("abc".to_string(),),
            _opt_u8: Some(56,),
            _opt_enum: None,
            _tuple1: "hi".to_string(),
            _tuple2: (123456789, -12345,),
            _tuple3: (8, 123456789,),
            _tuple4: (8, (true, "_tuple4".to_string(),),),
            _tuple5: (8, (true, "_tuple5".to_string(), (1, 2),),),
            window: Window {
                borderless: true,
                topmost: false,
                color: Color { bg: 13, fg: 12 },
            },
            map: Map::Down,
            map2: Map::Any("llll".to_string(),),
            map3: Map::Named {
                a: 1,
                _color: Color { bg: 32, fg: 12 },
            },
        }
    );
}

#[test]
fn test_struct_unnamed() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct StructUnnamed2(u8, (i8, (u16, (i16))));

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _un2: StructUnnamed2,
    }

    let config = r#"
fn main() {
    let a = Config {
        _un2: StructUnnamed2(1, (2, (3, 4))),
    };
}
"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _un2: StructUnnamed2(1, (2, (3, 4)))
        }
    );
}

#[test]
fn test_struct() {
    #[derive(Debug, Default, PartialEq, EsynDe)]
    struct Config {
        _struct_empty: StructEmpty,
        _struct_empty2: StructEmpty2,
        _struct_unnamed: StructUnnamed,
        _struct_tuple: StructUnnamed2,
        _struct_tuple3: StructUnnamed3,
        //_box: BoxStruct,
    }

    let config = r#"
fn main() {
    let a = Config {
        _struct_empty: StructEmpty {},
        _struct_unnamed: StructUnnamed(9, "abcd"),
        _struct_tuple: StructUnnamed2(1, (2, (3, 4))),
        _struct_tuple3: StructUnnamed3(1, 2),
    };
}
"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _struct_empty: StructEmpty {},
            _struct_empty2: StructEmpty2(),
            _struct_unnamed: StructUnnamed(9, "abcd".to_string()),
            _struct_tuple: StructUnnamed2(1, (2, (3, 4))),
            _struct_tuple3: StructUnnamed3(1, 2),
        }
    );
}

#[test]
fn test_enum() {
    #[derive(Debug, Default, EsynDe)]
    struct Config {
        _enum_unit: EnumUnit,
        _enum_unnamed: EnumUnnamed,
        _enum_named: EnumNamed,
        _enum1: Enum,
        _enum2: Enum,
        _enum3: Enum,
        _enum4: Enum,
        _enum5: Enum,
    }

    let config = r#"
fn main() {
    let a = Config {
        _enum_unit: EnumUnit::Unit2,
        _enum_unnamed: EnumUnnamed::Unnamed(99, -99, "unnamed"),
        _enum_named: EnumNamed::Named {
            _u8: 123,
            _i32: -123456789,
            _string: "named",
        },
        _enum1: Enum::Unit2,
        _enum2: Enum::Unnamed2(
            "Unnamed2",
            '🍵',
            StructNamed {
                _u8: -12,
                _string: "StructNamed",
                _vec_t: [1, 2, 3],
            },
        ),

        _enum3: Enum::Named2 {
            _u8: -3,
            _u16: 456,
            _struct_named: StructNamed {
                _u8: -12,
                _string: "StructNamed",
                _vec_t: ['a', 'b', 'c'],
            },
        },
        //_enum4: Enum, // default
    };

    a._enum5 = Enum::Unit3;
}
"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();
    dbg!(&a);

    assert_eq!(a._enum_unit, EnumUnit::Unit2);
    assert_eq!(
        a._enum_unnamed,
        EnumUnnamed::Unnamed(99, -99, "unnamed".to_string()),
    );
    assert_eq!(
        a._enum_named,
        EnumNamed::Named {
            _u8: 123,
            _i32: -123456789,
            _string: "named".to_string(),
        }
    );
    assert_eq!(a._enum1, Enum::Unit2);
    assert_eq!(
        a._enum2,
        Enum::Unnamed2(
            "Unnamed2".to_string(),
            '🍵',
            StructNamed {
                _u8: 244,
                _string: "StructNamed".to_string(),
                _vec_t: [1, 2, 3,].to_vec(),
            },
        )
    );
    assert_eq!(
        a._enum3,
        Enum::Named2 {
            _u8: 253,
            _u16: 456,
            _struct_named: StructNamed {
                _u8: 244,
                _string: "StructNamed".to_string(),
                _vec_t: ['a', 'b', 'c',].to_vec(),
            },
        }
    );
    assert_eq!(a._enum4, Enum::Unit1);
    assert_eq!(a._enum5, Enum::Unit3);
}

#[test]
fn test_type() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _bool: bool,
        _f32: f32,
        _f64: f64,
        _i8: i8,
        _isize: isize,
        _opt_none: Option<u8>,
        _opt_u8: Option<u8>,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config2 {
        _u8: u8,
        _i8: i8,

        _u16: u16,
        _i16: i16,

        _u32: u32,
        _i32: i32,

        _u64: u64,
        _i64: i64,

        _u128: u128,
        _i128: i128,

        _bool: bool,
        _char: char,

        _vec_u8: Vec<u8>,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Other {
        _u8: u8,
    }

    let config = r#"
fn main() {
    let a = Config {
        _bool: true,
        _f32: 3.2,
        _f64: -0.1234_5678_9123,
        _i8: 123,
        _isize: -12345678912345,
        _opt_none: None,
        _opt_u8: Some(10),
    };

    let b = Config {};

    let other = Other {
        _u8: 1
    };

    let a2 = Config2 {
        _u8: 127,
        _i8: -128,

        _u16: 1,
        _i16: 1,

        _u32: 1,
        _i32: 1,

        _u64: 1,
        _i64: 1,

        _u128: 1,
        _i128: 1,

        _bool:true,
        _char:'A',

        _vec_u8: [0, 1, 2, 3],

    };
}
"#;
    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _bool: true,
            _f32: 3.2,
            _f64: -0.123456789123,
            _i8: 123,
            _isize: -12345678912345,
            _opt_none: None,
            _opt_u8: Some(10),
        }
    );

    assert_eq!(
        map.get("b").unwrap(),
        &Config {
            _bool: false,
            _f32: 0.0,
            _f64: 0.0,
            _i8: 0,
            _isize: 0,
            _opt_none: None,
            _opt_u8: None,
        }
    );

    let map = esyn.get::<Other>("main").unwrap();
    assert_eq!(map.get("other").unwrap(), &Other { _u8: 1 });

    let map = esyn.get::<Config2>("main").unwrap();
    assert_eq!(
        map.get("a2").unwrap(),
        &Config2 {
            _u8: 127,
            _i8: -128,

            _u16: 1,
            _i16: 1,

            _u32: 1,
            _i32: 1,

            _u64: 1,
            _i64: 1,

            _u128: 1,
            _i128: 1,

            _bool: true,
            _char: 'A',
            _vec_u8: [0, 1, 2, 3].to_vec(),
        }
    );
}

#[test]
fn test_tuple() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _tuple_default: (u8, (u16, (u32, u64))),
    }

    let config = r#"
fn main() {
    let a = Config {
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();

    dbg!(map.get("a").unwrap(),);
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _tuple_default: (0, (0, (0, 0))),
        }
    );
}

#[test]
fn test_hashmap() {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _hashmap_u8_string: HashMap<u8, String>,
        _default: HashMap<u8, String>,
    }

    let config = r#"
fn main() {
    let a = Config {
        _hashmap_u8_string: [
            (0, "a"),
            (1, "b"),
            (2, "c"),
        ] as HashMap,
        _default: [] as HashMap,
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();
    let a1 = &a._hashmap_u8_string;

    assert_eq!(a1.get(&0).unwrap(), "a");
    assert_eq!(a1.get(&1).unwrap(), "b");
    assert_eq!(a1.get(&2).unwrap(), "c");

    assert_eq!(a._default, HashMap::default());
}

#[test]
fn test_ip() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _ip4: Option<Ipv4Addr>,
        _ip6: Option<Ipv6Addr>,

        _ip: Option<IpAddr>,
        _ip_none: Option<IpAddr>,
    }

    let config = r#"
fn main() {
    let a = Config {
        _ip4: Some( (127, 0, 0, 1) as Ipv4Addr ),
        _ip6: Some( (1234, 4321, 0, 0, 0, 0, 0, 0 ) as Ipv4Addr ),

        _ip: Some( IpAddr::V4(1, 1, 1, 1) ),
        //_ip_none,

    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();

    assert_eq!(a._ip4, Some(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(a._ip6, Some(Ipv6Addr::new(1234, 4321, 0, 0, 0, 0, 0, 0)));
    assert_eq!(a._ip, Some(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
    assert_eq!(a._ip_none, None);
}

// TODO:
//#[test]
//fn test_closure() {
//    dbg!(syn::parse_str::<syn::Expr>(
//        r#"
//|alias|("a.b.c", "k")
//"#
//    ));
//}
