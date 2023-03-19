fn main() {
    println!("Hello, world!");
}

// we can use match to trigger behavior when a value == config max,
// but we don't want to do anything if it doesn't, so we're left with an
// empty catch-all case
fn verbose_control_flow() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    };
}

// instead, we can use if-let to only check and execute code when the case is met
fn if_let_flow() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if-let-else is also a viable pattern
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Not the max");
    }
}
