// Encapsulation in Rust
// - the implementation details of an object aren't accessible to code using that object
// interaction with an object should go through it's public API, allowing changes to be
// made to the internals of an object without needing to change code that uses the object

use chapter17_1::AveragedCollection;

fn main() {
    let mut ac = AveragedCollection::new(
        vec![1, 2, 3, 4, 5, 6, 7]
    );

    println!("AveragedCollection: {:?}", ac);

    let mut avg = ac.average();

    println!("avg 1: {avg}");

    ac.remove();

    avg = ac.average();

    println!("avg 2: {avg}");

    ac.add(10);

    ac.add(50);

    avg = ac.average();

    println!("avg 3: {avg}");

    println!("ac final: {:?}", ac);

    // ac.update_average();
    // ^^ won't compile: update_average is not a public method
}
