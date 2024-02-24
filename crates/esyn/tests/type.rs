use esyn::*;

#[derive(Debug, PartialEq, EsynDe)]
struct Mix<T, F = f32> {
    _bool: bool,

    _string: String,

    _vec: Vec<T>,
    _vec_f32: Vec<F>,

    _opt_none: Option<u8>,
    _opt_u8: Option<u8>,
    _opt_box_self: Option<Box<Self>>,
    _opt_box_u8: Option<Box<u8>>,

    _box_u8: Box<u8>,

    _isize: isize,
    _usize: usize,
    unit: Uint,
    int: Int,
    float: Float,
    neg_float: NegFloat,
}

#[derive(Debug, PartialEq, EsynDe)]
struct Uint {
    _u8_max: u8,
    _u8_min: u8,
    _u16_max: u16,
    _u16_min: u16,
    _u32_max: u32,
    _u32_min: u32,
    _u64_max: u64,
    _u64_min: u64,
    _u128_max: u128,
    _u128_min: u128,
}

#[derive(Debug, PartialEq, EsynDe)]
struct Int {
    _i8_max: i8,
    _i8_min: i8,
    _i16_max: i16,
    _i16_min: i16,
    _i32_max: i32,
    _i32_min: i32,
    _i64_max: i64,
    _i64_min: i64,
    _i128_max: i128,
    _i128_min: i128,
}

#[derive(Debug, EsynDe, PartialEq)]
struct Float {
    _f32: f32,
    _f64: f64,
}

#[derive(Debug, EsynDe, PartialEq)]
struct NegFloat {
    _f32: f32,
    _f64: f64,
}

#[test]
fn test_num() {
    let config = r#"
fn main() {
    let uint = Uint {
        _u8_max: 255u8,
        _u8_min: 0u8,
        _u16_max: 65_535u16,
        _u16_min: 0u16,
        _u32_max: 4_294_967_295u32,
        _u32_min: 0u32,
        _u64_max: 18_446_744_073_709_551_615u64,
        _u64_min: 0u64,
        _u128_max: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
        _u128_min: 0u128,
    };

    let int = Int {
        _i8_max: 127i8,
        _i8_min: -128i8,
        _i16_max: 32767i16,
        _i16_min: -32768i16,
        _i32_max: 2_147_483_647i32,
        _i32_min: -2_147_483_648i32,
        _i64_max: 9_223_372_036_854_775_807i64,
        _i64_min: -9_223_372_036_854_775_808i64,
        _i128_max: 170141183460469231731687303715884105727i128,
        _i128_min: -170141183460469231731687303715884105728i128,
    };

    let float = Float {
        _f32: 3.4028235e38f32,
        _f64: 1.7976931348623157e308f64,
    };
    let neg_float = NegFloat {
        _f32: -3.4028235e38f32,
        _f64: -1.7976931348623157e308f64,
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let uint = &EsynBuilder::new()
        .set_let("uint")
        .get::<Uint>(&esyn)
        .unwrap();
    let int = &EsynBuilder::new().set_let("int").get::<Int>(&esyn).unwrap();

    let _float = &esyn.get_value::<Float>("main", "float").unwrap();
    let _neg_float = &esyn.get_value::<NegFloat>("main", "neg_float").unwrap();

    assert_eq!(
        uint,
        &Uint {
            _u8_max: 255u8,
            _u8_min: 0u8,
            _u16_max: 65_535u16,
            _u16_min: 0u16,
            _u32_max: 4_294_967_295u32,
            _u32_min: 0u32,
            _u64_max: 18_446_744_073_709_551_615u64,
            _u64_min: 0u64,
            _u128_max: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
            _u128_min: 0u128,
        }
    );

    assert_eq!(
        int,
        &Int {
            _i8_max: 127i8,
            _i8_min: -128i8,
            _i16_max: 32767i16,
            _i16_min: -32768i16,
            _i32_max: 2_147_483_647i32,
            _i32_min: -2_147_483_648i32,
            _i64_max: 9_223_372_036_854_775_807i64,
            _i64_min: -9_223_372_036_854_775_808i64,
            _i128_max: 170141183460469231731687303715884105727i128,
            _i128_min: -170141183460469231731687303715884105728i128,
        }
    );

    //dbg!(float, neg_float);
    //panic!();
}

#[test]
fn test_num_rand() {
    let config = r#"
fn main() {
    let uint = Uint {
        _u8_max: 255u8,
        _u8_min: 0u8,
        _u16_max: 65_535u16,
        _u16_min: 0u16,
        _u32_max: 4_294_967_295u32,
        _u32_min: 0u32,
        _u64_max: 18_446_744_073_709_551_615u64,
        _u64_min: 0u64,
        _u128_max: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
        _u128_min: 0u128,
    };

    let int = Int {
        _i8_max: 27i8,
        _i8_min: -28i8,
        _i16_max: 2767i16,
        _i16_min: -2768i16,
        _i32_max: 147_483_647i32,
        _i32_min: -147_483_648i32,
        _i64_max: 223_372_036_854_775_807i64,
        _i64_min: -223_372_036_854_775_808i64,
        _i128_max: 70141183460469231731687303715884105727i128,
        _i128_min: -70141183460469231731687303715884105728i128,
    };

    let float = Float {
        _f32: 0.1234f32,
        _f64: 4.321f64,
    };

    let neg_float = NegFloat {
        _f32: -0.1234f32,
        _f64: -4.321f64,
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let uint = &esyn.get_value::<Uint>("main", "uint").unwrap();
    let int = &esyn.get_value::<Int>("main", "int").unwrap();
    let _float = &esyn.get_value::<Float>("main", "float").unwrap();
    let _neg_float = &esyn.get_value::<NegFloat>("main", "neg_float").unwrap();

    assert_eq!(
        uint,
        &Uint {
            _u8_max: 255u8,
            _u8_min: 0u8,
            _u16_max: 65_535u16,
            _u16_min: 0u16,
            _u32_max: 4_294_967_295u32,
            _u32_min: 0u32,
            _u64_max: 18_446_744_073_709_551_615u64,
            _u64_min: 0u64,
            _u128_max: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
            _u128_min: 0u128,
        }
    );

    assert_eq!(
        int,
        &Int {
            _i8_max: 27i8,
            _i8_min: -28i8,
            _i16_max: 2767i16,
            _i16_min: -2768i16,
            _i32_max: 147_483_647i32,
            _i32_min: -147_483_648i32,
            _i64_max: 223_372_036_854_775_807i64,
            _i64_min: -223_372_036_854_775_808i64,
            _i128_max: 70141183460469231731687303715884105727i128,
            _i128_min: -70141183460469231731687303715884105728i128,
        }
    );

    //    assert_eq!(
    //        float,
    //        &Float {
    //            _f32: 0.1234f32,
    //            _f64: 4.321f64,
    //        }
    //    );
    //    assert_eq!(
    //        neg_float,
    //        &NegFloat {
    //            _f32: -0.1234f32,
    //            _f64: -4.321f64,
    //        }
    //    );
}

#[test]
fn test_float() {
    let config = r#"
fn main() {
    let float = Float {
        _f32: 3.4028235e38f32,
        _f64: 1.7976931348623157e308f64,
    };
    let neg_float = NegFloat {
        _f32: -3.4028235e38f32,
        _f64: -1.7976931348623157e308f64,
    };
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let _float = &esyn.get_value::<Float>("main", "float").unwrap();
    let _neg_float = &esyn.get_value::<NegFloat>("main", "neg_float").unwrap();
}

#[test]
fn test_mix() {
    // TODO: more
    let config = r#"
fn main() {
    let mix = Mix {};
    mix._vec = ["aa", "bb", "cc"];
    mix._vec_f32 = [0.1, 0.2, 0.3];

    mix._opt_box_u8 = Some(255);
    //mix._opt_u8 = Some(1); // default
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let mix = &esyn.get_value::<Mix<String>>("main", "mix").unwrap();

    assert_eq!(mix._vec, ["aa", "bb", "cc"]);
    assert_eq!(mix._vec_f32, [0.1, 0.2, 0.3]);
    assert_eq!(mix._opt_box_u8, Some(Box::new(255)));
    assert_eq!(mix._opt_u8, None);
}

#[test]
fn test_full() {
    #[derive(Debug, PartialEq, EsynDe, Default)]
    enum Map {
        Any(String),
        Named {
            a: u8,
            _color: Color,
        },
        Down,
        #[default]
        Up,
    }

    #[derive(Debug, PartialEq, EsynDe)]
    struct Window {
        borderless: bool,
        topmost: bool,
        color: Color,
    }

    #[derive(Debug, PartialEq, EsynDe)]
    struct Color {
        bg: u8,
        fg: u8,
    }

    #[derive(Debug, PartialEq, EsynDe)]
    struct Test {
        id: u64,
        name: String,
        invert_mouse: bool,
        font: Option<String>,
        _opt_u8: Option<u8>,
        _opt_color: Option<Color>,
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
    let a = Test {
        id: 123456789123u64,
        name: "hi",
        map: Map::Down,
        map2: Map::Any("llll"),
        map3: Map::Named {
            a: 1,
            _color: Color {
                bg: 12,
                fg: 34,
            },
        },
        invert_mouse: true,
        font: Some("abc"),
        _tuple1: ("hi"),
        _tuple2: (123456789, -12345),
        _tuple3: (8, 123456789),
        _tuple4: (8, (true, "_tuple4")),
        _tuple5: (8, (true, "_tuple5", (1, 2),),),
        _opt_color: None,
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

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    let a = &esyn.get_value::<Test>("main", "a").unwrap();

    assert_eq!(
        a,
        &Test {
            id: 123456789123,
            name: "hi".to_string(),
            invert_mouse: true,
            font: Some("abc".to_string(),),
            _opt_u8: Some(56),
            _opt_color: None,
            _tuple1: ("hi".to_string()),
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
                _color: Color { bg: 12, fg: 34 },
            },
        }
    );
}

#[test]
fn text_null() {
    let config = r#"
fn main() {
    let a = ();
}
"#;

    let a = EsynBuilder::new()
        .set_let("a")
        .get_once::<()>(config)
        .unwrap();

    assert_eq!(a, ());
}

// TODO:
#[test]
fn test_lit_byte() {
    #[derive(Debug, PartialEq, EsynDe)]
    struct Test {
        _lit_byte: ByteStr,
        _char: char,
    }

    let config = r#"
fn main() {
    let a = Test {
        _lit_byte: b"abc",
        _char: 'o',
    };
}
"#;

    let a = &EsynBuilder::new()
        .set_let("a")
        .get_once::<Test>(config)
        .unwrap();

    assert_eq!(a._char, 'o');
    assert_eq!(a._lit_byte.value(), &[97, 98, 99]);
}
