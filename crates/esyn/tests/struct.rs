use esyn::{Esyn, EsynDe};

#[derive(Debug, PartialEq, Default, EsynDe)]
struct BoxStruct {
    _box: Option<Box<Self>>,
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructEmpty {}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructEmpty2();

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnit;

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed(u8, String);

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed2(u8, (i8, (u16, i16)));

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructUnnamed3(u8, u8);

#[derive(Debug, PartialEq, Default, EsynDe)]
struct StructNamed<T> {
    _u8: u8,
    _string: String,
    _vec_t: Vec<T>,
}

#[test]
fn test_struct_unnamed() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _un2: StructUnnamed2,
        _opt_self: Option<Box<Self>>,
    }

    let config = r#"
fn main() {
    let a = Config {
        _un2: StructUnnamed2(1, (2, (3, 4))),
        _opt_self: Some(
            Config {
                _un2: StructUnnamed2(5, (6, (7, 8))),
                _opt_self: None,
            },
        ),
    };
}
"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _un2: StructUnnamed2(1, (2, (3, 4))),
            _opt_self: Some(Box::new(Config {
                _un2: StructUnnamed2(5, (6, (7, 8))),
                _opt_self: None,
            }))
        }
    );
}

#[test]
fn test_struct_unit() {
    #[derive(Debug, Default, PartialEq, EsynDe)]
    struct Config {
        _struct_unit: StructUnit,
        _default: StructUnit,
    }

    let config = r#"
fn main() {
    let a = Config {
        _struct_unit: StructUnit,
        //_default
    };
}
"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _struct_unit: StructUnit,
            _default: StructUnit
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
        _struct_unit: StructUnit,
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
            _struct_unit: StructUnit,
        }
    );
}
