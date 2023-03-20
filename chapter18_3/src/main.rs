struct Point {
    x: i32,
    y: i32,
}

struct TriPoint {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rbg(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Msg {
    Hello { id: i32 },
}

fn main() {
    // matching literals: useful when you want your code to take action if it gets
    // a particular value
    let a = 1;
    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("I can only count to three"),
    }

    // matching variables
    let d = Some(5);
    let e = 10;

    match d {
        Some(50) => println!("Got 50"),
        Some(e) => println!("Matched, e = {e}"),
        _ => println!("Default case, d = {:?}", d),
    }

    println!("at the end: d = {:?}, e = {e}", d);

    // multiple patterns
    let b = 1;

    match b {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    let c = 5;

    match c {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // with char values
    let ch = 'c';

    match ch {
        'a'..='j' => println!("early ascii letter"),
        'k'..='z' => println!("late ascii letter"),
        _ => println!("something else"),
    }

    // destructuring with patterns
    let p = Point { x: 1, y: 2 };

    let Point { x: a, y: b } = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    // with shorthand
    let Point { x, y } = p;
    assert_eq!(1, x);
    assert_eq!(2, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    let msg = Message::ChangeColor(Color::Rbg(1, 2, 3));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y } => println!("Move in the x direction {x} and y direction {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(Color::Rbg(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, value {v}");
        }
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}");
    println!("inches: {inches}");
    println!("x {x}");
    println!("y {y}");

    foo(3, 4);

    // ignoring parts of a value with nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let s = Some(String::from("hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // ignore remaining values
    let origin = TriPoint { x: 0, y: 0, z: 0 };
    match origin {
        TriPoint { x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // extra conditionals with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let n = 4;
    let m = false;

    match n {
        4 | 5 | 6 if m => println!("yes"),
        _ => println!("no"),
    }

    // bindings - @ operator lets us create a variable that holds a value at the same time
    // we're testing that value for a pattern match
    let message = Msg::Hello { id: 5 };

    match message {
        Msg::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {}", id_variable),
        Msg::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Msg::Hello { id } => println!("found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y param: {}", y);
}
