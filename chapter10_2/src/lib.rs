use std::fmt::Display;
use std::cmp::PartialOrd;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else if self.y < self.x {
            println!("The largest member is y = {}", self.y);
        } else {
            println!("x and y are the same size");
        }
    }
}

pub fn func() {
    let pair = Pair::new(1, 2);

    println!("pair: {:?}", pair.cmp_display());
}