#[test]
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
