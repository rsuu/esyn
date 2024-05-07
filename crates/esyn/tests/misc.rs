use esyn::*;

#[test]
fn docs_on_struct() {
    /// a
    /// b
    /// c
    #[derive(Debug, PartialEq, EsynDe, EsynSer)]
    struct A {
        a: u8,
    }

    /* a
     * b
     * c
     */
    #[derive(Debug, PartialEq, EsynDe, EsynSer)]
    struct B {
        a: u8,
    }
}
