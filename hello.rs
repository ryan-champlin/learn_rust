use std::f64::consts;


fn main() {
    println!("Hello, World!");
    variable();
    test_assert();
    loops();
    adding();
    println!("{}", sqr(2.0));
    println!("{} , {}", abs(-3.001), abs(3.5));
    println!("{}", factorial(3));

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&i);
    let res3 = by_ref(&41);
    println!("{} {} {}", res1, res2, res3);

    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);

    println!("{:?}", using_constants()); // doesn't work like that. Still pretty cool to have build in unit tests

    array_fun();

    let arr = [10,20,30,40];
    println!("sum {}", sum_arr(&arr));

    slices();
    slices2();

    vector();

    vector_slice();

    iterator();

    sum_arr_level_2();

    vec();
    vec_fun();

    string_types();

    string_manip();

    print_array();

    cmd_line_arg();
}

fn variable() {
    let answer = 42; // can also be written as  `let answer: i64 = 42` if you want to be very obvious what the type is.
    println!("Hello agent {}", answer); // ! is needed because its a macro. TODO difference between macro and built in function.
}

fn test_assert() {
    let answer = 42; // didn't have to declare that this was an int or some version of a number?
    assert_eq!(answer, 42);
    // first run of above was with 40 not 42 in order to give a runtime error. Note, was not a compiler error.
    // no output on true. 
}

fn loops() {
    for i in 0..5 { 
        println!("Hello {}", i);
    }

    for i in 5..10 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("false {}", i);
        }
    }

    for i in 10..15 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"}; 
        println!("{} {}", even_odd, i);
    }
}

fn adding() {
    let mut sum = 0; // this needs mut (mutable) in order for the variable to be able to be change. thats a bit strange.
    for i in 0..5 {
        sum += i; //+= can only add ints. its possible to cast it with something like `sum += i as f64`
    }
    println!("The sum is {}", sum);
}

fn sqr(x: f64) -> f64 {
    x * x   // doesn't need a return or ; because its the last line of the function
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1) // easy enough
    }
}

// pass by reference
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

// mutable reference
fn modifies(x: &mut f64) {
    *x = 1.0;
}

// passing by reference is important when we have a large object and don't wish to copy it. 
fn using_constants() {
    let x = 2.0 * consts::PI;

    let abs_difference = (x.cos() - 1.0).abs();

    assert!(abs_difference < 1e-10);
}

//
// Arrays and Slices
//

fn array_fun() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];

    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }

    println!("length {}", arr.len());
}

fn sum_arr(values: &[i32]) -> i32 {
    let mut res = 0;

    for i in 0..values.len() {
        res += values[i] // it feels like there should be a semicolon here, but there doesn't need to be one.
    }
    res
}

fn slices() {
    let ints = [1,2,3,4,5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[..4];
    let slice3 = &ints[1..];

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
    println!("slice3 {:?}", slice3);
}

fn slices2() {
    let ints = [1,2,3,4,5];
    let slice = &ints;
    let index = slice[0];
    let first = slice.get(0);
    let last = slice.get(5); // index intentionally out of bounds;

    println!("indexed {}", index);
    println!("first {:?}", first);
    println!("last {:?}", last);

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    let maybe_last = slice.get(5);

    let safe_last = if maybe_last.is_some() {
        *maybe_last.unwrap()
        // if I put a semicolong here ^ it fails, I'm not yet sure why. error says that
        // it changes the type? not sure what is actually going on. 
    } else {
        -1
    };
    println!("safe last check {}", safe_last);

    // the above can be done with a shortcut
    let quick_last = *slice.get(5).unwrap_or(&-1);
    println!("quick safe last check {}", quick_last);
}


//
// Vectors
// 

fn vector() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
 
    let first = v[0];
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}

fn vector_slice() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);

    
}

fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn iterator() {
    let arr = [10, 20, 30];

    for i in arr {
        println!("{}", i);
    }
}

fn sum_arr_level_2(){
    let sum: i32 = (0..5).sum();
    println!("Sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("Sum was {}", sum);
}

fn vec() {
    let mut v1 = vec![10,20,30,40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10,20,30,0,1]);
}

fn vec_fun() {
    let mut v1 = vec![1,10,5,1,2,11,2,40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}

fn dump_string(s: &str) {
    println!("Str '{}'", s);
}

fn string_types() {
    let text = "hello!"; // string slice
    let s = text.to_string(); //allocated string

    dump_string(text);
    dump_string(&s);
}

fn string_manip() {
    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!";
    s.pop();

    assert_eq!(s, "Hello World");

    let mut _text = "test";  //fails because this is &str not &String
    // text.push('!');
    // assert_eq!(text, "test!");
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();

    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn print_array() {
    let arr = array_to_str(&[10, 20, 30]);
    let result = format!("hello {}", arr);

    assert_eq!(result, "hello [10,20,30]");
}

fn cmd_line_arg() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}