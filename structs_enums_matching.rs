fn main() {
    passing_strings();
    block_scoped_example();
    test_tuple();
}

fn dump(s: &str) {
    println!("{}", s);
}

fn passing_strings() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("s1 {}", s1);
}

fn block_scoped_example() {
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a, b, and c are all visable.
        println!("c {}", c);
    }

    // a, b are visable.
    for _i in 0..a {
        let b = &b[1..];
        // og b isn't visible. its "shadowed"?
        println!("sliced b {:?}", b);
    }
    // sliced b isn't visible. i is not visible.

    println!("a {}", a);
    println!("b {}", b);
}

fn add_mul(x:f64, y:f64) -> (f64,f64) {
    (x + y, x*y)
}

fn test_tuple() {
    let t = add_mul(2.0, 10.0);

    println!("t {:?}", t);

    println!("add {}, mul {}", t.0, t.1);

    let (add, mul) = t;
    println!("add {} mul {}", add, mul);

    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}