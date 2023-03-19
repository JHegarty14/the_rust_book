#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Guess must be less than or equal to 100, got {}", value);
        }
        else if value < 1 {
            panic!("Guess must be greater than or equal to 1, got {}", value);
        }
        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Jack");
        assert!(
            result.contains("Jack"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn catch_out_of_bounds_guess() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

    // #[test]
    // fn another_test() {
    //     panic!("Force failure");
    // }
}
