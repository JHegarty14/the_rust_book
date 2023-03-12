use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;
// ^ brings HashMap into scope in an idiomatic way

// avoid idiomatic use path when bringing in two items w/ same name
// use std::fmt;
// use std::io;

// prefer using as to rename imports
use std::fmt::Result;
use std::io::Result as IoResult;

pub mod garden;

fn main() {
    let veg = Asparagus {};
    println!("I'm growing {:?}", veg);

    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> Result {
//     write!("")
// }

fn function2() -> IoResult<i32> {
    let num = 5;
    Ok(num)
}