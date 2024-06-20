#[test]
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

    let test = Esyn::builder()
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
