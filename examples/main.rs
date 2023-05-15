use esyn::EsynDe;

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
    window: Window,
    map: Map,
    map2: Map,
    map3: Map,
}

fn main() {
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
    let list = esyn.get::<Config>("test_example").unwrap();
    let a = list.get("a").unwrap();

    assert_eq!(a.name, "hi".to_string());
    assert_eq!(a.map, Map::Down);
    assert_eq!(a.font, Some("abc".to_string()));
    assert_eq!(a.window.borderless, true);
    assert_eq!(a.window.topmost, false);
    assert_eq!(a.window.color.bg, 13);
}
