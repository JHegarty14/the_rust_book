/// Slices let you reference a contiguous sequence of elements in a collection rather than the entire collection
/// Slices are a kind of reference and DO NOT have ownership

fn main() {
    let mut str = String::from("hello world");
    let word = first_word(&str);

    println!("word: {word}");

    str.clear();
    // str is now empty, but word is still 5

    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];

    println!("{hello}");
    println!("{world}");
    println!("{whole}");

    let w = first_word_as_slice(&s);

    println!("as slice: {w}");
    
    // error! s is borrowed as immutable in first_word_as_slice
    // s.clear();

    let w = first_word_as_slice(hello);

    println!("31: {w}");
    
    let w = first_word_as_slice(world);

    println!("35: {w}");

    let w = first_word_as_slice(whole);

    println!("39: {w}");

    let my_literal = "hello world";

    let w = first_word_as_slice(&my_literal[..6]);

    println!("45: {w}");

    let w = first_word_as_slice(&my_literal[..]);

    println!("49: {w}");

    let w = first_word_as_slice(my_literal);

    println!("53: {w}");

    // array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_as_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
