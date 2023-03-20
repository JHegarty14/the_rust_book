// Refutability: Whether a pattern might fail to match
// - refutable patterns: patterns that can fail to match for some possible value
// - irrefutable patterns: patterns that cannot fail to patch
// function params, let statements, and for loops can only accept irrefutable patterns

fn main() {
    // let Some(x) = Option::from(5);
    // ^^^ won't compile: refutable pattern because None case is not covered

    // works because the if block will be skipped in case of None, therefore case is covered
    if let Some(x) = Option::from(5) {
        println!("{}", x);
    }
}
