use esyn::*;

use std::path::PathBuf;

#[derive(Debug, PartialEq, EsynDe)]
struct Test {
    _path_buf: PathBuf,
}

#[test]
fn main() {
    let config = r#"
fn main() {
    let a = Test {
        _path_buf: "./a/b/c",
    };
}
"#;

    assert_eq!(
        &EsynBuilder::new()
            .set_let("a")
            .get_once::<Test>(config)
            .unwrap(),
        &Test {
            _path_buf: PathBuf::from("./a/b/c"),
        }
    );
}
