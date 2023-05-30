use esyn::{Esyn, EsynDe};

fn main() {
    let config = r#"
fn main() {
    let a = Config {
        name: "hi",
        map: Map::Down,
        window: Window {
            borderless: true,
            topmost: false,
        },
        opt: Some(56),
    };

    a.window.color = Color {
        bg:13,
        fg:12,
    };

    let other = Other {
        _a: 1
    };
}
"#;
    let esyn = Esyn::new(&config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    let a = map.get("a").unwrap();

    assert_eq!(
        a,
        &Config {
            name: "hi".to_string(),
            map: Map::Down,
            window: Window {
                borderless: true,
                topmost: false,
                color: Color { bg: 13, fg: 12 },
            },
            opt: Some(56),
        }
    );

    en(a);

    let map = esyn.get::<Other>("main").unwrap();
    let a = map.get("other").unwrap();
}

fn en(a: &Config) {
    use esyn::TypeInfo;
    //let mut f = std::fs::File::create("./en.rs").unwrap();
    let mut f = vec![];
    a.en(&mut f, "main", "a").unwrap();

    //dbg!(String::from_utf8(f).unwrap());
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct Other {
    _a: u8,
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct Config {
    name: String,
    opt: Option<u8>,
    window: Window,
    map: Map,
}

#[derive(PartialEq, EsynDe)]
enum Map {
    Up,
    Down,
    Any(String),
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Map::Up"),
            Self::Down => write!(f, "Map::Down",),
            Self::Any(v) => write!(f, "Map::Any({:?})", v),
        }
    }
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
