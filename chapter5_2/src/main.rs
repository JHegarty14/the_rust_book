#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("area: {}", area(width1, height1));

    let tuple_rect = (40, 60);

    println!("tuple area: {}", tuple_area(tuple_rect));

    let struct_rect = Rectangle {
        width: 50,
        height: 70,
    };

    // pass immutable borrow of Rectangle
    println!("struct area: {}", struct_area(&struct_rect));

    println!("print struct: {:?}", struct_rect);
    
    println!("pretty-print struct: {:#?}", struct_rect);

    let scale = 2;
    let debug_rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    // dbg! mactro prints to stderr, vs println! which prints to stdout
    dbg!(&debug_rect);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
