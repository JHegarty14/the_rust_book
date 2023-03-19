use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // allowable if main returns a Result with a dynamic error type
    let greeting_file = File::open("hello.txt")?;

    // if main returns () unit type, we must explicitly throw exception via panic!
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Could not create file: {e}"),
    //         },
    //         other_error => {
    //             panic!("Could not open file: {:?}", other_error);
    //         },
    //     },
    // };

    println!("file: {:?}", greeting_file);

    // without match
    let file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() != ErrorKind::NotFound {
            panic!("Could not open file: {:?}", error);
        }

        File::create("hello2.txt").unwrap_or_else(|error| {
            panic!("Could not create file: {:?}", error);
        })
    });

    println!("file2: {:?}", file2);

    // will panic if open doesn't return Ok variant
    // let file3 = File::open("hello3.txt").unwrap();

    // .expect can be used like .unwrap, but allows us to provide a contextualizing message for the error
    let file3 = File::open("hello3.txt").expect("hello3.txt should be included in this directory");

    println!("file3: {:?}", file3);

    Ok(())
}

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_res = File::open("hello.txt");

    let mut username_file = match username_file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
fn read_username_from_file() -> Result<String, io::Error> {
    // ? operator can only be used in fns whose return type matches the value the ? is used on
    // File::open returns Result<File, io::Error>, so because the E type matches the function E type,
    // we can use ?
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn use_last_char() -> char {
    let text = "test string";

    let result = last_char_of_first_line(text);

    match result {
        Some(s) => s,
        None => panic!("empty string"),
    }
}
