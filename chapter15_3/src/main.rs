use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my string"),
    };
    let d = CustomSmartPointer {
        data: String::from("other string"),
    };

    println!("CustomSmartPointers created.");

    // output
    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!

    // Rust automatically calls drop when the CustomSmartPointer instances go out of scope
    // variables are dropped in reverse order, hence d being dropped before c

    // we can also drop a value early
    let e = CustomSmartPointer {
        data: String::from("drop early"),
    };

    println!("e created!");
    // e.drop()
    // ^^ won't compile: we can't call drop implicitly because rust would still try to drop e at the end of main when it goes out of scope
    // which would cause a double-free error
    drop(e);
    // ^^ calling std::mem::drop explicitly prevents double-free errors
    println!("e was dropped before end of main");
}
