use std::fmt::Display;

// structs can hold references, but if they do they must have a lifetime annotation on every reference
// in the struct's definition
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
    // author: &str,  <<< won't compile without lifetime 'b annotation
}

impl<'a> ImportantExcerpt<'a> {
    // we aren't required to annotate self with a lifetime param, see Ellison Rule #1
    fn level(&self) -> i32 {
        3
    }

    //  two input lifetimes, so per Ellison Rule #1 both self and announcement get their own lifetimes
    // because 1 param is &self, the return type gets the lifetime of &self and all lifetimes are covered
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // let r;                                           // ---------+-- 'a
                                                        //          |
    // {                                                //          |                                  
    //     let x = 5;                                   // -+-- 'b  |
    //     r = &x;                                      //  |       |
    // }                                                // -+       |
                                                        //          |
    // println!("r: {}", r);                            // ---------+

    let x = 5;                                     // ---------+-- 'b
                                                        //          |
    let r = &x;                                   // --+-- 'a |
                                                        //   |      |
    println!("r: {}", r);                               //   |      |
                                                        // --+      |
                                                        // ---------+
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("longer string for example");

    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("10-22 Example: The longest string is {}", result2);
    }

    // won't compile
    // let string5 = String::from("longer string for example");        // ----------------+-- 'a
    // let result3;                                                    // ---------+-- 'b |
    // {                                                               //          |      |
    //     let string6 = String::from("xyz");                          // -+-- 'c  |      |
    //     result3 = longest(string5.as_str(), string6.as_str());      //  |       |      |
    // }                                                               // -+~~~~~~~ << borrowed value dropped
                                                                       //                 |
    // println!("10-23 Example: The longest string is {}", result3);   // ----------------|

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find a .");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("excerpt: {:?}", excerpt);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }
    y
}

// won't compile: cannot return reference to local variable `result`
// returns a reference to data owned by the current function
// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("long string");
//     result.as_str()
// }
// ^^^^^ the return value lifetime is not related to the lifetime of the parameters


// Ellison Rules:
// 1. the compiler assigns a lifetime parameter to each parameter that’s a reference
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters
// If the compiler cannot automatically assign an input lifetime to all output lifetime parameters, we must explicitly specify them

// lifetimes are a type of generic, and so go in the angle brackets after a function's name
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        return x
    }
    y
}