use esyn::*;
use std::fmt::Debug;

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

#[derive(Debug, PartialEq, EsynDe)]
enum EnumUnit {
    Unit1,
    Unit2,
}

#[derive(Debug, PartialEq, EsynDe)]
enum EnumUnnamed {
    Unnamed(u8, i32, String),
}

#[derive(Debug, PartialEq, EsynDe)]
enum EnumNamed {
    Named { _u8: u8, _i32: i32, _string: String },
}

#[derive(Debug, PartialEq, EsynDe)]
enum Enum {
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
fn test_enum() {
    #[derive(Debug, EsynDe)]
    struct Test {
        _enum_unit: EnumUnit,
        _enum_unnamed: EnumUnnamed,
        _enum_named: EnumNamed,
        _enum1: Enum,
        _enum2: Enum,
        _enum3: Enum,
        _enum4: Enum,
        _enum5: Enum,
        _enum6: Enum,
        _opt_enum_unit: Option<EnumUnit>,
    }

    let config = r#"
fn main() {
    let a = Test {
        _enum_unit: EnumUnit::Unit2,
        _enum_unnamed: EnumUnnamed::Unnamed(99, -99, "unnamed"),
        _enum_named: EnumNamed::Named {
            _i32: -123456789,
            _string: "named",
            _u8: 123,
        },
        _enum1: Enum::Unit1,
        _enum2: Enum::Unnamed2(
            "Unnamed2",
            '🍵',
            StructNamed {
                _u8: 12u8,
                _string: "StructNamed",
                _vec_t: [1, 2, 3],
            },
        ),
        _enum3: Enum::Named2 {
            _u8: 3,
            _u16: 456,
            _struct_named: StructNamed {
                _u8: 12,
                _string: "StructNamed",
                _vec_t: ['a', 'b', 'c'],
            },
        },

        // TODO: use ..Default::default()
        //_enum4, // default

        _opt_enum_unit: Some(EnumUnit::Unit1),
    };

    a._enum5 = Enum::Unit3;
}
"#;

    let res = EsynBuilder::new()
        .set_let("a")
        .get_once::<Test>(config)
        .unwrap();
    let a = res.get_ref();
    //dbg!(&a);

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
    assert_eq!(a._enum1, Enum::Unit1);
    assert_eq!(
        a._enum2,
        Enum::Unnamed2(
            "Unnamed2".to_string(),
            '🍵',
            StructNamed {
                _u8: 12,
                _string: "StructNamed".to_string(),
                _vec_t: [1, 2, 3,].to_vec(),
            },
        )
    );
    assert_eq!(
        a._enum3,
        Enum::Named2 {
            _u8: 3,
            _u16: 456,
            _struct_named: StructNamed {
                _u8: 12,
                _string: "StructNamed".to_string(),
                _vec_t: ['a', 'b', 'c',].to_vec(),
            },
        }
    );
    assert_eq!(a._enum4, Enum::Unit1);
    assert_eq!(a._enum5, Enum::Unit3);
    assert_eq!(a._opt_enum_unit, Some(EnumUnit::Unit1));
}

#[test]
fn enum_empty() {
    // ? #[derive(Debug, PartialEq, EsynDe)]
    enum EnumEmpty {}
}
