use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} team has scored {score} points.");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // ^ on insert, field_name and field_value are moved and are invalid past this point

    // throws borrow error
    // println!("{field_name}: {field_value}");

    let field2_name = String::from("Favorite movie");
    let field2_value = String::from("La Haine");

    map.insert(field2_name.clone(), field2_value.clone());
    // clone before insert to avoid original value being moved

    println!("{field2_name}: {field2_value}");
    println!("{:?}", map);

    // overwritting values
    map.insert(field2_name.clone(), String::from("Banshees of Inisherin"));

    println!("{:?}", map);

    // add only if key does not exist
    map.entry(field2_name).or_insert(String::from("Shrek"));
    map.entry(String::from("Cat's name:")).or_insert(String::from("Toulouse"));

    println!("{:?}", map);

    // update value based on old value
    let text = "hello world wonderful world";

    let mut update_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = update_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", update_map);
}
