use esyn::*;

use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, EsynDe)]
struct Test {
    hash_map: HashMap<u8, u16>,
    btree_map: BTreeMap<u8, bool>,
}

#[test]
fn main() {
    let config = r#"
fn main() {
    let a = Test {
        hash_map: [(1,2), (3,4)],
        btree_map: [(1,true), (2,false)],
    };
}
"#;

    assert_eq!(
        EsynBuilder::new()
            .set_let("a")
            .get_once::<Test>(config)
            .unwrap()
            .get_ref(),
        &Test {
            hash_map: HashMap::from_iter([(1, 2), (3, 4)].into_iter()),
            btree_map: BTreeMap::from_iter([(1, true), (2, false)].into_iter()),
        }
    );
}
