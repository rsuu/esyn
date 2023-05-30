fn test_example() {
    let a = Config {
        name: "hi",
        map: Map::Down,
        map2: Map::Any("llll"),
        map3: Map::Named {
            a: 1,
            _color: Color { fg: 32, bg: 12 },
        },
        invert_mouse: false,
        font: Some("abc"),
        window: Window {
            color: Color { fg: 32 },
        },
        _tuple: (1, ("hi"), 2),
        _tuple2: (1, (-2, (3, (-4)))),
        _opt_enum: None,
    };

    a.window = Window {
        borderless: true,
        topmost: true,
    };
    a.window.borderless = false;
    a.window.color = Color { bg: 12 };
}

fn test_enum() {
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

fn test_type() {
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
}

fn test_struct() {
    let a = Config {
        _struct_empty: StructEmpty {},
        _struct_unnamed: StructUnnamed(9, "abcd"),
        _struct_tuple: StructUnnamed2(1, (2, (3, 4))),
        _struct_tuple3: StructUnnamed3(1, 2),
        _box: None,
    };
}

fn full() {
    let a = A {
        //_u8: 123,
        _opt_u8: None,
        _u32: 123456,
        _vec_u8: [2, 3, 4, 5],
        _sb: B {
            _u8: 1,
            _tuple_3: (3, 4, "lol"),
            _opt_u8: Some(2),
            _opt_box_u8: Some(90),
            _sc: C {
                _vec_u8: [2, 3, 4, 5],
                _enum: Enum::Unnamed("abcd"),
            },
            _vec_u8: [0, 1, 2, 3],
            _enum_unit: Enum::Unit2,
            _enum_unnamed: Enum::Unnamed2("s1", "s2", D { _i8: 5 }),
            _enum_named: Enum::Named2 {
                _u8: 8,
                _u16: 16,
                _sd: D { _i8: -123 },
            },
        },
    };

    a._box_u8 = Some(999);
    a._u8 = 3;
    a._sb._vec_u8 = [3, 2, 1];
    a._sb._u8 = 7;
    a._sb._enum_unit = Enum::Unit3;
}
