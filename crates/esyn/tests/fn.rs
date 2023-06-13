use esyn::{Esyn, EsynDe};

#[test]
fn test_fn_alias_path() {
    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct Config {
        l1: L1,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct L1 {
        val1: u8,
        l2: L2,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct L2 {
        val2: u8,
        l3: L3,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct L3 {
        val3: u8,
        l4: L4,
    }

    #[derive(Debug, PartialEq, Default, EsynDe)]
    struct L4 {
        val4: u8,
    }

    let config = r#"
fn main() {
    let a = Config {};

    ::alias_path(_alias2, a.l1.l2);
    _alias2.val2 = 2;
    _alias2.l3.val3 = 3;

    ::alias_path(_alias, a.l1.l2.l3.l4);
    _alias.val4 = 4;

    ::alias_path(_alias, a.l1);
    _alias.val1 = 1;

}"#;

    let esyn = Esyn::new(config).unwrap();
    let map = esyn.get::<Config>("main").unwrap();

    //assert_eq!(map.get("a").unwrap());
    assert_eq!(
        map.get("a").unwrap(),
        &Config {
            l1: L1 {
                val1: 1,
                l2: L2 {
                    val2: 2,
                    l3: L3 {
                        val3: 3,
                        l4: L4 { val4: 4 },
                    },
                },
            },
        }
    );
}
