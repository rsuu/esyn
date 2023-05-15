use esyn::{Esyn, EsynDe};

#[test]
fn main() {
    let esyn = esyn::Esyn::from_file("./full.rs").unwrap();

    test_example(&esyn);
    test_struct(&esyn);
    test_enum(&esyn);
    test_type(&esyn);
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructEmpty {}

//#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnit;

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed(u8, String);

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed2(u8, (i8, (u16, i16)));

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

fn test_example(esyn: &Esyn) {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    enum Map {
        #[default]
        Up,
        Any(String),
        Named {
            a: u8,
            _color: Color,
        },
        Down,
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
        name: String,
        invert_mouse: bool,
        font: Option<String>,
        window: Window,
        _opt_enum: Option<Color>,

        map: Map,
        map2: Map,
        map3: Map,
        _tuple: (u8, String, u32),
        _tuple2: (u8, (i8, (u16, i16))),
    }

    let list = esyn.get::<Config>("test_example").unwrap();
    let a = list.get("a").unwrap();
    dbg!(&a);

    assert_eq!(a.name, "hi".to_string());
    assert_eq!(a.map, Map::Down);
    assert_eq!(a.font, Some("abc".to_string()));

    assert_eq!(a.window.borderless, false);
    assert_eq!(a.window.topmost, true);
    assert_eq!(a._tuple2, (1, (-2, (3, (-4)))));
}

fn test_struct(esyn: &Esyn) {
    #[derive(Debug, Default, EsynDe)]
    struct Config {
        _struct_unnamed: StructUnnamed,
        _struct_empty: StructEmpty,

        _struct_tuple: StructUnnamed2,
        //_box: BoxStruct,
    }

    let list = esyn.get::<Config>("test_struct").unwrap();
    let a = list.get("a").unwrap();
    dbg!(&a);

    assert_eq!(a._struct_unnamed, StructUnnamed(9, "abcd".to_string()));
    assert_eq!(a._struct_empty, StructEmpty {});

    // FIXME:
    //assert_eq!(a._struct_tuple, StructUnnamed2(1, (-2, (3, (-4)))));
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

    let list = esyn.get::<Config>("test_enum").unwrap();
    let a = list.get("a").unwrap();
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
    #[derive(Debug, Default, EsynDe)]
    struct Config {
        _bool: bool,
        _f32: f32,
        _f64: f64,
        _i8: i8,
        _isize: isize,
        _opt_u8: Option<u8>,
        _opt_None: Option<u8>,
    }

    let list = esyn.get::<Config>("test_type").unwrap();
    let a = list.get("a").unwrap();
    dbg!(&a);

    assert_eq!(a._bool, false);
    assert_eq!(a._f32, 3.2);
    assert_eq!(a._f64, -0.1234_5678_9123);
    assert_eq!(a._i8, 123);
    assert_eq!(a._isize, -1234);
    assert_eq!(a._opt_u8, Some(10));
    assert_eq!(a._opt_None, None);

    let a = list.get("b").unwrap();
    dbg!(&a);

    assert_eq!(a._bool, false);
    assert_eq!(a._f32, 0.0);
}
