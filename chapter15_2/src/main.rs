use std::ops::{Deref, DerefMut};

struct MyBoxWrong<T>(T);

impl<T> MyBoxWrong<T> {
    fn new(x: T) -> MyBoxWrong<T> {
        MyBoxWrong(x)
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // assert_eq!(5, y);
    // ^^ won't compile: comparing an int to a reference is not valid

    let x1 = 5;
    let y1 = Box::new(x1);
    //  ^^ y1 is a copied value of x rather than a reference pointing to x

    assert_eq!(5, x1);
    assert_eq!(5, *y1);
    // ^^ we can still use the dereference operator to follow the pointer as we did with the raw pointer

    let x2 = 5;
    let _y2 = MyBoxWrong::new(x2);

    assert_eq!(5, x2);
    // assert_eq!(5, *y2);
    // ^^ won't compile: MyBoxWrong cannot be deferenced. We need to implement the Deref trait

    let x3 = 5;
    let y3 = MyBox::new(x3);

    assert_eq!(5, x3);
    assert_eq!(5, *y3);
    // ^^ *y actual transpiles to *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // ^^ because MyBox implements Deref, Rust can turn &MyBox(String) into &String by calling deref.
    // If it didn't do this for us, we'd have to write this as

    let n = MyBox::new(String::from("Rust"));
    hello(&(*n)[..]);

    // both compile and run, but option 1 is obviously much cleaner and readable
    // deref coercion incurs no runtime penalty

    let mut x4 = MyBox::new(5);
    *x4 = 10;
    assert_eq!(10, *x4);
}

fn hello(name: &str) {
    println!("Hello {name}");
}