use esyn::{Esyn, EsynDe};

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
        _box_u8: Box<u8>,
        _opt_box_u8: Option<Box<u8>>,
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
        _opt_box_self: Option<Box<Self>>,
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
        _box_u8: 12,
        _opt_box_u8: Some(13),
    };

    let b = Config {};

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

    let other = Other {
        _u8: 1,
        _opt_box_self: Some(Other {
            //_opt_box_self: None, // panic
            _u8: 2,
            _opt_box_self: None,
        }),
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
            _box_u8: Box::new(12),
            _opt_box_u8: Some(Box::new(13))
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
            _box_u8: Box::new(0),
            _opt_box_u8: None
        }
    );

    let map = esyn.get::<Other>("main").unwrap();
    assert_eq!(
        map.get("other").unwrap(),
        &Other {
            _u8: 1,

            _opt_box_self: Some(Box::new(Other {
                _u8: 2,
                _opt_box_self: None
            })),
        }
    );

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

    //dbg!(map.get("a").unwrap(),);
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
fn test_btreemap() {
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _btreemap_u8_string: BTreeMap<u8, String>,
    }

    let config = r#"
fn main() {
    let a = Config {
        _btreemap_u8_string: [
            (0, "a"),
            (1, "b"),
            (2, "c"),
        ] as BTreeMap,
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();
    let a1 = &a._btreemap_u8_string;

    assert_eq!(a1.get(&0).unwrap(), "a");
    assert_eq!(a1.get(&1).unwrap(), "b");
    assert_eq!(a1.get(&2).unwrap(), "c");
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

#[test]
fn test_box() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _opt_box_self_none: Option<Box<Self>>,
        _u8: u8,
        _opt_box_self: Option<Box<Self>>,
    }

    let config = r#"
fn main() {
    let a = Config {
        //_u8,
        _opt_box_self: Some(
            Config {
                _u8: 2,
                _opt_box_self_none: None,
                _opt_box_self: None,
            },
        ),
        _opt_box_self_none: None,
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();
}

#[test]
fn test_option() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _opt_none: Option<Other>,
        _opt: Option<Other>,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Other {
        _u8: u8,
        _u32: u32,
    }

    let config = r#"
fn main() {
    let a = Config {
        _opt_none: None,
        _opt: Some(
            Other {
                _u8: 12,
                _u32: 34,
            },
        ),

    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _opt_none: None,
            _opt: Some(Other { _u32: 34, _u8: 12 }),
        }
    );
}
