use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    let closure = |x| x;
    let s = closure(String::from("hello"));
    // let n = closure(5); 
    // ^^^^^ won't compile because closure infers one concrete type per parameter, so after the first
    // use with a String, the closure can't be re-typed

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure {:?}", list);

    println!("Before calling closure {:?}", list);
    only_borrows();
    println!("After calling closure {:?}", list);

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure 2: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    // println!("Before calling closure 2: {:?}", list2); <-- immutable borrower, won't compile
    borrows_mutably();
    println!("After calling closure 2: {:?}", list2);

    let list3 = vec![1, 2, 3];
    println!("Before defining closure 3: {:?}", list3);

    // move forces the new thread closure to take ownership of list3
    // without moving the reference, list3 might be dropped prematurely since
    // the new thread closure may outlive the current function
    thread::spawn(move || println!("From thread {:?}", list3))
        .join()
        .unwrap();

    // Closure traits
    // 1. FnOnce: applies to any closure that can be called once. A closure that moves captured values out of its body will implement it and no other Fn traits
    // 2. FnMut: applies to closures that don't move captured values out of their body, but might mutate captured values. Can be called more than once
    // 3. Fn: applies to closures that don't mutate or move captured values. includes closures that don't capture anything. Can be called more than once without side effects

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|c| c.width);
    println!("{:?}", list);

    list.sort_by_key(|c| c.height);
    println!("{:?}", list);

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value); <<<<<<<< won't compile b/c capturing a string and moving it 
    //     r.width
    // });
    // println!("{:?}", list);

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:?}, sorted in {num_sort_operations} operations", list);
}
