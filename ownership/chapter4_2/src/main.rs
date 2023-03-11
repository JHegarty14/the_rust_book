fn main() {
    let s1 = String::from("hello");

    // passing a reference to s1 prevents fn from taking ownership
    let len = calculate_length(&s1);

    println!("len: {len}");

    // errs, cannot modify immutable ref
    // let changed = change(&s1);

    let mut mut_str = String::from("mut hello");

    println!("before change: {mut_str}");


    change(&mut mut_str);

    println!("after change: {mut_str}");

    // let r1 = &mut mut_str;
    // let r2 = &mut mut_str;
    // errors: > 1 mutable borrows disallowed
    // multiple mutable references introduce potential data races
    // - multiple pointers accessing data simultaneously can lead to dirty reads/writes
    // - no synchronization mechanism = DANGER
    

    // create new scope to prevent data races
    {
        let r1 = &mut mut_str;
        println!("r1 is dropped here {r1}");
    }

    // r1 is dropped, so it is safe to create another mut ref
    let r2 = &mut mut_str;
    println!("r2 {r2}");

    {
        let mut s = String::from("forty");

        let ir1 = &s;
        let ir2 = &s;
        // DISALLOWED
        // mut ref when immutable refs exists still risks dirty reads when dereffing from ir1/ir2
        // let mr1 = &mut s;

        println!("{ir1}, {ir2}");
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(_s: &String) {
    // error: cannot modify borrowed ref
    // s.push_str(", world")
// }

fn change(s: &mut String) {
    s.push_str(", world!");
}

// Cannot return pointer to OOS data
// "s" is dropped when fn returns, so &s is a nullptr
// fn dangle() -> &String {
//     let s = String::from("dangler");
//     &s
// }

// solution is to return String directly
fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
}