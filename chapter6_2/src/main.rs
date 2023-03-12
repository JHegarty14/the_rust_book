#[derive(Debug)]
enum States {
    Alabama,
    Alaska,
    Arkansas,
    California,
    Conneticut,
    Deleware
}

enum Coins {
    Penny,
    Nickle,
    Dime,
    Quarter(States),
}

fn main() {
    let coin = Coins::Nickle;
    let value = value_in_cents(&coin);

    println!("value: {}", value);

    let coin = Coins::Penny;
    let value = value_in_cents(&coin);

    println!("value: {}", value);

    let coin = Coins::Quarter(States::California);
    let value = value_in_cents(&coin);

    println!("value: {}", value);

    let coin = Coins::Dime;
    let value = value_in_cents(&coin);

    println!("value: {}", value);

    let coin = Coins::Quarter(States::Alaska);
    let value = value_in_cents(&coin);
    
    println!("value: {}", value);

    let sum = plus_one(Some(value));

    println!("sum: {}", sum.unwrap());
}

fn value_in_cents(coin: &Coins) -> i32 {
    let value = match coin {
        Coins::Penny => 1,
        Coins::Nickle => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("quarter is from state: {:?}", state);    
            25
        },
    };
    value
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn dice_game() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other represents any other i32 value rolled
        other => move_player(other),
    };

    
}

fn dice_game_2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // no value is passed on catch-all case
        _ => reroll()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_n: i32) {}
fn reroll() {}