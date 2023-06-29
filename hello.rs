fn main() {
    println!("Hello, World!");
    variable();
    test_assert();
    loops();
    adding();
    println!("{}", sqr(2.0));
    println!("{} , {}", abs(-3.001), abs(3.5));
    println!("{}", factorial(3));
}

fn variable() {
    let answer = 42;
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
        let even_odd = if i % 2 == 0 {"even"} else {"odd"}; // is this an array?
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
        n * factorial(n-1)
    }
}