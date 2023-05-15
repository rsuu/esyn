# [WIP] esyn

[<img alt="github" src="https://img.shields.io/badge/github-rsuu/esyn-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rsuu/esyn)
[<img alt="crates.io" src="https://img.shields.io/crates/v/esyn.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/esyn)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-esyn-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/esyn)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/rsuu/esyn/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/rsuu/esyn/actions?query=branch%3Amain)

> Rusty Config File Parser.

## Example

```rust
use esyn::{Esyn, EsynDe};

fn main() {
    let config = r#"
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
        _opt_enum: None,
    };

    a.window = Window {
        borderless: true,
        topmost: true,
    };
    a.window.borderless = false;
    a.window.color = Color { bg: 12 };
}

"#;
    let esyn = esyn::Esyn::new(&config).unwrap();

    #[derive(Debug, PartialEq, EsynDe)]
    enum Map {
        Any(String),
        Named { a: u8, _color: Color },
        Down,
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
    }

    let list = esyn.get::<Config>("test_example").unwrap();
    let a = &list[0];

    assert_eq!(a.name, "hi".to_string());
    assert_eq!(a.map, Map::Down);
    assert_eq!(a.font, Some("abc".to_string()));

    assert_eq!(a.window.borderless, false);
    assert_eq!(a.window.topmost, true);

    dbg!(a);
}

```

## Supported Types

```rust
u8 u16 u32 u64 u128 usize
i8 i16 i32 i64 i128 isize
f32 f64
bool
char String

Option<T>
Vec<T>

Struct
Enum

```
