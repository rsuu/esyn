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

    let expect = Test {
        _string: "hello".to_string(),
        _vec_u32: vec![1, 2, 4],
        _opt_i64: Some(-9223372036854775807),
    };
    let config = r#"
fn main() {
    let test = Test {
        _string: "hello",
        _vec_u32: [1, 2, 4],
    };

    test._opt_i64 = Some(-9223372036854775807);
}
"#;

    let test = EsynBuilder::new()
        .set_fn("main")
        .set_let("test")
        .get_once::<Test>(config)
        .unwrap();

    assert_eq!(test.get_ref(), &expect);

    // Serialization
    let code = quote! {
        fn main() {
            let test = #test;
        }
    }
    .into_pretty()
    .unwrap();

    println!("{code}");
}
```

For more examples take a look on [tests](https://github.com/rsuu/esyn/tree/main/crates/esyn/tests).

## Features

## Suffix Syntax

```rust
fn main() {
    // ...

    config.val = 123;
}
"#;
```

### Custom Syntax

```rust
// ...

#[derive(Debug, EsynDe, EsynSer, Default)]
#[custom_syntax]
struct Test {
    val: u8,
}

impl CustomSyntax for Test {
    // ...
}

// in config:
//
// fn main() {
//     let test = m!(123);
// }

// ...
```

see full example on [struct_custom_syntax](https://github.com/rsuu/esyn/tree/main/crates/esyn/tests/struct_custom_syntax.rs)

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
