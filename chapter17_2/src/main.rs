// Trait objects for polymorphic behavior

use chapter17_2::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // let screen2 = Screen {
    //     components: vec![
    //         Box::new(String::from("Hi")),
    //     ],
    // };
    // ^^ won't compile: String doesn't implement Draw trait

    // screen2.run();

    // Trait objects perform Dynamic Dispatch because the compiler can't know all possible types that may be used
    // with the code that's using the trait object. This means that at runtime, Rust uses pointers inside the trait
    // object to know which method to call. This incurs a runtime cost that doesn't occur with static dispath. If 
    // you know you will always be dealing with a homogenous collections, because these can be monomorphized at compile
    // time to use the concrete types
}
