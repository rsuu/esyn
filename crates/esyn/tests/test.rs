use std::fmt::Debug;

use esyn::{Esyn, EsynDe};

#[test]
fn main() {
    let esyn = esyn::Esyn::from_file("./full.rs").unwrap();

    test_struct_unnamed();
    test_struct(&esyn);
    test_example();
    test_struct(&esyn);
    test_enum(&esyn);
    test_type(&esyn);
}

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
fn test_example() {
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
    let map = esyn.get::<Config>("test_example").unwrap();

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

fn test_struct_unnamed() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct StructUnnamed2(u8, (i8, (u16, (i16))));

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _un2: StructUnnamed2,
    }

    let c = r#"
fn main() {
    let a = Config {
        _un2: StructUnnamed2(1, (2, (3, 4))),
    };
}
"#;

    let esyn = Esyn::new(c).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();
    dbg!(&a);

    assert_eq!(a._un2, StructUnnamed2(1, (2, (3, 4))));
}

fn test_struct(esyn: &Esyn) {
    #[derive(Debug, Default, EsynDe)]
    struct Config {
        _struct_empty: StructEmpty,
        _struct_empty2: StructEmpty2,
        _struct_unnamed: StructUnnamed,
        _struct_tuple: StructUnnamed2,
        _struct_tuple3: StructUnnamed3,
        //_box: BoxStruct,
    }

    let map = esyn.get::<Config>("test_struct").unwrap();
    let a = map.get("a").unwrap();
    dbg!(&a);

    //FIXME:
    assert_eq!(a._struct_empty, StructEmpty {});
    assert_eq!(a._struct_empty2, StructEmpty2());

    assert_eq!(a._struct_unnamed, StructUnnamed(9, "abcd".to_string()));

    assert_eq!(a._struct_tuple, StructUnnamed2(1, (2, (3, 4))));
    assert_eq!(a._struct_tuple3, StructUnnamed3(1, 2));
}

fn test_enum(esyn: &Esyn) {
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

    let map = esyn.get::<Config>("test_enum").unwrap();
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
                _vec_t: vec![1, 2, 3,],
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
                _vec_t: vec!['a', 'b', 'c',],
            },
        }
    );
    assert_eq!(a._enum4, Enum::Unit1);
    assert_eq!(a._enum5, Enum::Unit3);
}

fn test_type(esyn: &Esyn) {
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

    let map = esyn.get::<Config>("test_type").unwrap();
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
}
