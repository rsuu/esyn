use esyn::{Esyn, EsynDe};

fn main() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        _opt_none: Option<Other>,
        _opt: Option<Other>,
        _o: Opt<Other>,
        _on: Opt<Other>,
        //_opt_box: Option<Box<Self>>
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Other {
        _u8: u8,
        _u32: u32,
    }

    #[derive(Debug, PartialEq, EsynDe)]
    enum Opt<T> {
        V(T),
        N,
    }

    impl<T> Default for Opt<T> {
        fn default() -> Self {
            Self::N
        }
    }

    let config = r#"
fn main() {
    let a = Config {
        _opt_none: None,
        _opt: Some(
            Other {
                _u32: 34,
                _u8: 12,
            },
        ),
        _o: Opt::N,
        _on: Opt::V(Other { _u8: 34, _u32: 12 }),
    };
}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            _opt_none: None,
            _opt: Some(Other { _u8: 12, _u32: 34 }),
            _o: Opt::N,
            _on: Opt::V(Other { _u32: 12, _u8: 34 }),
        }
    );
}
