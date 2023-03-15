pub fn add(left: i32, right: i32) -> i32 {
    internal_adder(left, right)
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// using super::* brings parent items into scope, including private functions
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
