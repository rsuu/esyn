# [WIP] esyn

[<img alt="github" src="https://img.shields.io/badge/github-rsuu/esyn-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rsuu/esyn)
[<img alt="crates.io" src="https://img.shields.io/crates/v/esyn.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/esyn)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-esyn-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/esyn)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/rsuu/esyn/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/rsuu/esyn/actions?query=branch%3Amain)

**De/Serialization Rust In Rust**

## Example

```rust
fn main() {
    use esyn::*;

    #[derive(Debug, PartialEq, EsynDe, EsynSer)]
    struct Test {
        _string: String,
        _vec_u32: Vec<u32>,
        _opt_i64: Option<i64>,
    }

    let test = Test {
        _string: "hello".to_string(),
        _vec_u32: vec![1, 2, 4],
        _opt_i64: Some(-9223372036854775807),
    };
    let config = r#"
fn main() {
    let a = Test {
        _string: "hello",
        _vec_u32: [1, 2, 4],
    };

    a._opt_i64 = Some(-9223372036854775807);
}
"#;

    let a = EsynBuilder::new()
        .set_fn("main")
        .set_let("a")
        .get_once::<Test>(config)
        .unwrap();

    assert_eq!(test, *a);

    // Serialization
    let code = quote! {
        fn main() {
            let a = #a;
        }
    }
    .into_pretty()
    .unwrap();

    println!("{code}");
}
```

For more examples take a look on [tests](https://github.com/rsuu/esyn/tree/main/crates/esyn/tests).

## Features

### Custom Syntax

see [examples](https://github.com/rsuu/esyn/tree/main/crates/esyn/examples/dev_derive_attr.rs)

In short:

```rust
// ...

#[derive(Debug, EsynDe, EsynSer, Default)]
struct Test {
    // parse:
    //   8  |  f()  |  m!(32)  |  m![32]  |  m!{32}
    #[parse(
        Expr::Call => parse_fn(),
        Expr::Macro => parse_macro(),
    )]
    _u32: u32,
}

// ...
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
Box<T>

Struct
Enum
Tuple

```

[see more](https://github.com/rsuu/esyn/tree/main/crates/esyn/src/ext.rs)
