fn main() {
    passing_strings();
    block_scoped_example();
    test_tuple();
    test_struct();

    let p = Person::new("Ryan", "Champlin");
    println!("person {} {}", p.first_name, p.last_name);
    println!("person {}", p.full_name());

    let _p2 = Person::copy(&p);
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

    for p in ["zero", "one", "two"].iter().enumerate() {
        println!(" {} {}", p.0, p.1);
    }

    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];

    for item in names.iter().zip(nums.iter()) {
        println!("{} {}", item.0, item.1);
    }
}

struct Person {
    first_name: String,
    last_name: String
}

fn test_struct() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };

    println!("person {} {}", p.first_name, p.last_name);
}

impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }
}

