#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // use &self so we don't take ownership. We only want to read width & height
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // all fns defined within an impl block are called associated functions
    // because they are associated with the type named after the impl
    // associated fns differ from methods bc they don't take a &self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // associated fns can be used as constructors to return a new instance of a struct
    // "new" is not a special name in Rust
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

// structs can have multiple impl blocks
impl Rectangle {
    fn cannot_hold(&self, other: &Rectangle) -> bool {
        self.width <= other.width && self.height <= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("impl area: {}", rect1.area());

    if rect1.width() {
        println!("Rectangle width is: {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(40);

    println!("square area: {}", square.area());

    let new_rect = Rectangle::new(100, 200);

    println!("new rect area: {}", new_rect.area());

    println!("square cannot hold new_rect: {}", square.cannot_hold(&new_rect));
}
