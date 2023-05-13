fn main() {
    let site = 0;

    // depth, offset
    //
    // Config,
    // data: vec![
    //   b.c.v1  = 1,
    //   b.c.v2  = 0.0,
    //   b.dd    = 0,
    // ]
    Config {
        // a: -1,     // TODO: Int
        s: "lo", // String
        a: 90,
        sb: B {
            // Struct
            // (1,0)
            sc: C {
                // (2,0)
                v1: 1, // (2,1)
                v2: 2, // BUG:
            },
            v2: 12,
        },
        v1: 88,
        c: None, // (0,3)
        d: 3.0,  // Float
        e: Some(123), // Option<T>

                 // TODO:
                 //f: ["a", "b", "c"], // Array
                 //g: (0.0, "hello", 'c'),
                 //h: Map::Left, // Enum
                 //i: User::Name("xxx"),
    };

    Config.s = "lol"; // TODO:
    Config.sb.sc.v1 = 3;
    //Config.sb.sc.v2 = 4;
    Config.sb.v2 = 14;
    Config.sb.v1 = 90;
    //Config.c = Some("2");
    //Config.d = 12.0;
}

//fn user1() {
//    let site = "123.com";
//    let alias = ["a.com", "b.com", "c.com"];
//
//    let c1 = Config {
//        // title1: "example2.com/abc",
//        a: "brushup-life",
//        __: "the-marvelous-mrs-maisel",
//        b: Mark {
//            c: C { d: D { e: E {} } },
//        },
//    };
//    c1.b.c.d.e.f.g.h = H {};
//
//    let k = c1.b.c.d.e.f;
//    k = F {
//        up: Map::Up,
//        down: Map::Down,
//    };
//    k.g = G {};
//
//    let v = (
//        "{aaa}" as Json,
//        "bbb" as String,
//        "8" as u8,
//        "1.1" as f32,
//        "ccc",
//        b"123",
//        '\u{123}',
//        r####"
//let a = "1";
//let c = r##" "2" "##;
//
//
/////////////////
//            )]
//        "####,
//    );
//}
//
//fn user2() {}
