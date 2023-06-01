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
fn main() {
    let a = Config {
        name: "hi",
        window: Window {
            borderless: true,
        },
    };

    a.window.color = Color {
        bg:13,
        fg:12,
    };
}
"#;
    let esyn = Esyn::new(&config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();

    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            name: "hi".to_string(),
            window: Window {
                borderless: true,
                color: Color { bg: 13, fg: 12 },
            },
        }
    );
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct Config {
    name: String,
    window: Window,
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct Window {
    borderless: bool,
    color: Color,
}

#[derive(Debug, PartialEq, Default, EsynDe)]
struct Color {
    bg: u8,
    fg: u8,
}

```

## Supported Types

```rust
u8 u16 u32 u64 u128 usize
i8 i16 i32 i64 i128 isize
f32 f64
bool
char String

Vec<T>
Option<T>
HashMap<K, V>
BTreeMap<K, V>

Option<IpAddr>
Option<Ipv4Addr>
Option<Ipv6Addr>

Struct
Enum
Tuple

fast_image_resize::FilterType

?Box<T>

```
