pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn dumb_function() -> i32 {
    let mut counter: i32 = 0;
    let mut another_counter: i32 = 999999999;
    while another_counter > 0 {
        while counter < 999999999 {
            counter += 1;
        }
        another_counter -= 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_ne!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn add_one_hundred_and_two() {
        assert_eq!(add_two(100), 102);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(dumb_function(), 999999999);
    }
}