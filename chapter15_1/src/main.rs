#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // Boxes allow for data to be stored on the heap instead of the stack, leaving a pointer to the heap data in the stack
    // typical uses:
    // - handling data whose size cannot be known at compile time when exact size is required
    // - transfering ownership of large amounts of data without copying
    // - when you want to own a value and only care that it's of a type that implements a specific trait rather than being a certain type

    let b = Box::new(5);
    println!("b = {b}");

    // recursive types with Boxes
    // since the recursion could theorectically be infinite, Rust can't know the required amount of space at compile time
    // a cons list is an example of this type of recursive data structure, a list in which each item containes the value of the current item, and the next item
    // let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    //     ^^^^ won't compile, because `list` could hold itself infinite times, so Rust cannot figure out how much space it needs to store a List

    let msg = Message::Quit;

    // with non-recursive types, Rust chooses the largest variant to guarantee that enough space will be available regardless of what type is chosen at runtime
    // wrapping a recursive type with Box<T> is "indirection"; instead of storing a value directly, the data structure will store a pointer to the value
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    // list now looks like this
    //
    // +----------------------+
    // |        Cons          |
    // |----------------------|
    // |     |       Box      |
    // |     |    +-------+   |
    // | i32 |    | usize |   |
    // |     |    +-------+   |
    // +----------------------+
    //
    // so the type is no longer recursive from the compiler's perspective

    println!("list: {:?}", list);

    // Boxes only provide indirection and heap allocation, so trading other special capabilities for 0 performance overhead
}
