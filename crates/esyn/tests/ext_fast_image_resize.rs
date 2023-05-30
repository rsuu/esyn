#[cfg(feature = "ext_fast_image_resize")]
#[test]
fn test_fast_image_resize() {
    use esyn::{Esyn, EsynDe};
    use fast_image_resize::FilterType;

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _type: FilterType,
        _default: FilterType,
    }

    let config = r#"
fn main() {
    let a = Config {
        _type: FilterType::Box,
    };
}
"#;

    let esyn = Esyn::new(&config).unwrap();
    let list = esyn.get::<Config>("main").unwrap();
    let a = list.get("a").unwrap();

    assert_eq!(a._type, FilterType::Box);
    assert_eq!(a._default, FilterType::Lanczos3);
}
