use std::io;

fn main() {
    loop_labels();
    println!("Input a number");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess = match guess.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("error: {e}");
                continue;
            }
        };
        let z = plus_one(guess);
        println!("z equals {z}");

        control_flow(z);

        let bool_test = let_assignment(z);
        
        if bool_test {
            break counter;
        }
    };

    println!("Result: {result}");

    while_loops();
    for_loops();
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

fn control_flow(num: i32) {
    if num < 5 {
        println!("less than");
    } else if num > 5 {
        println!("greater than");
    } else {
        println!("equal");
    }
}

fn let_assignment(num: i32) -> bool {
    let condition: bool = if num > 3 { true } else { false };
    condition
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut n = 3;

    while n < 0 {
        n -= 1;
    }

    println!("Done");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50, 60];
    
    for elem in a {
        println!("Value of current element is: {elem}");
    }
}