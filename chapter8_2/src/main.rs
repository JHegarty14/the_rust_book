fn main() {
    let mut s = String::new();
    let data = "initial contents";

    println!("initial s: {s}");

    s = data.to_string();

    println!("line 9: {s}");

    // works on string literals directly, too
    s = "literal".to_string();

    println!("line 14: {s}");

    let s2 = "bar";
    s.push_str(s2);

    // push_str doesn't take ownership so we can still print s2
    println!("s2 is {s2}");
    println!("after push: {s}");

    s = String::from("lo");
    s.push('l');

    println!("s is {s}");

    let concat1 = String::from("Hello, ");
    let concat2 = String::from("world!");
    let concat3 = concat1 + &concat2; // s1 is moved and can no longer be accessed
    // ^ Rust uses deref coercion to coerce &String to &str on concat

    println!("concat3 is {concat3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s_all = format!("{s1}-{s2}-{s3}");

    println!("{s_all}");

    let hello = "Здравствуйте";
    // takes the first 4 bytes
    let h = &hello[0..4];

    // would panic at runtime because each character takes 2 bytes
    // let h = &hello[0..1];

    println!("{h}");

    for c in h.chars() {
        println!("{c}");
    }

    for b in h.bytes() {
        println!("{b}");
    }
}
