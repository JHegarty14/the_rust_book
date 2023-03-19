pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0  {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: you've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    // struct MockMessenger {
    //     sent_messages: Vec<String>,
    // }

    // impl MockMessenger {
    //     fn new() -> MockMessenger {
    //         MockMessenger { sent_messages: vec![] }
    //     }
    // }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }
    // ^^^ won't compile: the send method takes an immutable reference to self, and we can't update it to take &mut self because it would
    // no longer match the signature of send in the Messenger trait definition
    // we can use the interior mutability pattern with RefCell<T> instead

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            //                 ^^^ use borrow_mut so we can push to vec (thereby mutating it)
        }
    }

    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();
    //         ^^^^^^^ creating two mutable references in the same scope will panic at runtime, but won't be caught at compile time.
    //                 this is the trade-off of `unsafe` code

    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        //                                      ^^ can use borrow instead of borrow_mut because we're just reading
    }
}