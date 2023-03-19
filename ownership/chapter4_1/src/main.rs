fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("s: {s}");

    let x = 5;
    let y = x;

    println!("stack alloc: {x}, {y}");

    let s1 = String::from("literal");

    println!("heap alloc: {s1}");

    let s2 = s1;

    println!("heap alloc + s1 invalid: {s2}");

    takes_ownership(s2);

    // s2 is invalidated
    // println!("{}", s2);

    makes_copy(y);

    // int is not invalidated after it goes OOS in makes_copy
    println!("after makes_copy: {}", y);

    let s3 = gives_ownership();

    println!("s3 {s3}");

    // When a variable that includes data on the heap goes out of scope,
    // the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    let s4 = takes_and_gives_back(s3);

    println!("s4 {s4}");

    let (s5, len) = calculate_length(s4);

    println!("The length of {s5} is {len}.");
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let new_string = String::from("yours");
    new_string
}

fn takes_and_gives_back(s: String) -> String {
    // s comes into scope, invalidated in calling scope
    s // s is returned and moves out to calling fn
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}