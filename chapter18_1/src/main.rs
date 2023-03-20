// Where can Patterns be used?

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let introduces shadowed variables, much like match arms
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is a green day");
    } else if let Ok(age) = age {
        // vv references shadowed age variable, so Ok(age) && age > 30 would be invalid
        // because shadowed age variable doesn't exist until we are in the new scope
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // while let follows similar convention and rules to if let
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec![1, 2, 3];

    // patterns can be used to destructure tuples
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3);
    // ^^^ won't compile: expected a tuple with 3 elements, found one with 2 elements
    // example of a pattern violation

    let coords = (-34.234678, 117.209345);
    print_coordinates(&coords);
}

// function params as patterns
fn print_coordinates(&(x,  y): &(f64, f64)) {
    println!("Current location: {}, {}", x, y);
}
