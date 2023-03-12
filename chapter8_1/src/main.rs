#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mac_v = vec![1, 2, 3, 4, 5];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    matcher(third);

    let eighth: Option<&i32> = v.get(8);
    matcher(eighth);

    // first does_not_exist will panic at runtime
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let first = &v[0];

    // error: cannot mutate v because we are borrowing on line 24
    // vectors store elements together in memory, adding an element may force the vector to be moved
    // and the old memory to be deallocated
    // v.push(6);

    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row: {:?}", row);
}

fn matcher(to_match: Option<&i32>) {
    match to_match {
        Some(m) => println!("Match {m}"),
        None => println!("There is no match."),
    };
}
