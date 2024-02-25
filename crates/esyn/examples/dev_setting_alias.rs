use esyn::*;

fn main() {
    let config = r#"
fn main() {
// TODO:
//    ::AliasMap::new!(
//        a.a     => a,
//        a.b.c.d => b,
//    );
}
"#;

    let esyn = Esyn::new(config);
    esyn.init().unwrap();

    //    let v = &EsynBuilder::new()
    //        .set_fn("main")
    //        .set_let("a")
    //        .get::<Test>(&esyn)
    //        .unwrap();
    //    dbg!(v);
}
