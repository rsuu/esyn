use esyn::*;

use std::net::*;

#[derive(Debug, PartialEq, EsynDe)]
struct Test {
    v1: IpAddr,
    v2: IpAddr,

    v4: Ipv4Addr,
    v6: Ipv6Addr,
}

#[test]
fn main() {
    let config = r#"
fn main() {
    let a = Test {
        v1: IpAddr::V4(127, 0, 0, 1),
        v2: IpAddr::V6(127, 0, 0, 1, 1, 2, 3, 4),
        v4: [127, 0, 0, 1],
        v6: [127, 0, 0, 1, 0, 0, 0, 0],
    };
}
"#;

    assert_eq!(
        Esyn::builder()
            .set_let("a")
            .get_once::<Test>(config)
            .unwrap()
            .get_ref(),
        &Test {
            v1: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            v2: IpAddr::V6(Ipv6Addr::new(127, 0, 0, 1, 1, 2, 3, 4)),
            v4: Ipv4Addr::new(127, 0, 0, 1),
            v6: Ipv6Addr::new(127, 0, 0, 1, 0, 0, 0, 0)
        }
    );
}
