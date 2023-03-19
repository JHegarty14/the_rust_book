#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Interior Mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
    // this pattern relies on `unsafe` code inside a data structure to bend Rust's rules arround borrowing and mutation
    // `unsafe` code tells the compiler that the programmer is responsible for checking the code, not the compiler
    // The Rust compiler is inherently conservative, and will reject programs that it cannot validate runtime memory safety for,
    // even if the programmer is able to validate the program is safe at runtime
    
    // let x = 5;
    // let y = &mut x;
    // ^^^ won't compile: cannot borrow immutable value as mutable
    // there are cases where it can be helpful to make x immutable to certain areas of your program, but allow other areas to mutate it

    // Use Case for Interior Mutability: Mock Objects
    // see src/lib.rs
    // jack note: this would be a good pattern for DI

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Output:
    // a before = Cons(RefCell { value: 5 }, Nil)
    // b before = Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Nil))
    // c before = Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Nil))
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
    // changes to value after mutable borrow propogate through references
}
