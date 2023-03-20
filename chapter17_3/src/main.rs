// OOP design patterns in Rust: The State Pattern

use chapter17_3::Post;

fn main() {
    // With State Pattern
    // let mut post = Post::new();

    // post.add_text("I ate a salad for lunch today");
    // println!("Got: {}", post.content());
    // assert_eq!("", post.content());

    // post.request_review();
    // println!("Got: {}", post.content());
    // assert_eq!("", post.content());

    // post.approve();
    // println!("Got: {}", post.content());
    // assert_eq!("I ate a salad for lunch today", post.content());

    // Encoding States and Behavior as Types
    let mut post = Post::new();
    let str = "I ate a salad for lunch today";

    post.add_text(str);
    // assert_eq!("", post.content()); type DraftPost doesn't implement content method

    let post = post.request_review(); // post is now PendingReviewPost

    let post = post.approve(); // post is now Post with content method

    assert_eq!(str, post.content());
}
