use std::rc::Rc;

enum BoxList {
    BoxCons(i32, Box<BoxList>),
    Nil,
}

enum List {
    Cons(i32, Rc<List>),
    Nil
}

// use crate::BoxList::{BoxCons, Nil};

use crate::List::{Cons, Nil};

fn main() {
    // handling cases where a single value has multiple owners
    // ex. in a graph, a node may have mulitple edges pointing to it, which "own" the node
    // the node should not be cleaned up unless it has 0 edges pointing to it, therefore 0 owners

    // Rc<T>: reference counter that keeps track of the number of references to a value
    // we use Rc<T> when we want to allocate data on the heap for multiple parts of a program,
    // but don't know at compile time which owner will stop using it last
    // Rc<T> is only for SINGLE THREADED scenarios

    // let a = BoxCons(5, Box::new(BoxCons(10, Box::new(Nil))));
    // let b = BoxCons(3, Box::new(a));
    // let c = BoxCons(4, Box::new(a));
    // ^^^ won't compile: BoxList doesn't implement Copy trait, so when a is moved to b leaving it inaccessible to c

    // wrap a and nested cons list in reference counters
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // clone reference to a to avoid moving/copying a
    // Rc::clone doesn't make a deep copy, whereas most types' `clone` implementation do, and so is more performant
    // now a won't be cleaned up until it has 0 references
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _d = Cons(6, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));

    // Rc<T> provides immutable references to allow you to share data between multiple parts of your program for READING ONLY

}
