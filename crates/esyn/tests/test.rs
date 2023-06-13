use esyn::{Esyn, EsynDe};

#[test]
fn test_full() {
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
            _color: Color {
                bg: 12,
                fg: 34,
            },
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
                _color: Color { bg: 12, fg: 34 },
            },
        }
    );
}

// TODO:
#[test]
fn test_lit_byte() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _lit_byte: Vec<u8>,
        _char: char,
    }

    let config = r#"
fn main() {
    let a = Config {
        _lit_byte: b"abc",
        _char: 'o',
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();

    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _char: 'o',
            _lit_byte: vec![97, 98, 99]
        }
    );
}
