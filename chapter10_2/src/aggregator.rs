use std::fmt::Debug;

pub trait Summary {
    // trait definitions group metho signatures together to define a set of behavior
    // these method signatures can then be shared across impls to create common behavior
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
    pub date: String,
}

#[derive(Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Zine {
    pub issue: i32,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("{}, ({})", self.author, self.location)
    }
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for Zine {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Display for Tweet {
    fn display(&self) -> String {
        format!("displaying")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Trait bound! {}", item.summarize());
}

pub fn notify_multi(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news x2! {}, {}", item1.summarize(), item2.summarize());
}

pub fn notify_multi_trait_bound<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news trait bound x2! {}, {}", item1.summarize(), item2.summarize());
}

pub fn notify_multi_trait(item: &(impl Summary + Display)) {
    println!("{}, {}", item.summarize(), item.display());
}

pub fn notify_multi_trait_alt<T: Summary + Display>(item: &T) {
    println!("{}, {}", item.summarize(), item.display());
}

pub fn some_function_verbose<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    println!("{:#?} - {:#?}", t.display(), u);
}

pub fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{:#?} - {:#?}", t.display(), u);
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("wint"),
        content: String::from("no"),
        reply: false,
        retweet: false,
    }
}

// won't compile, NewsArticle and Tweet are of incompatible types despite both implementing Summary
// pub fn returns_summarizable_switch(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("headline"),
//             location: String::from("Los Angeles"),
//             author: String::from("Author Name"),
//             content: String::from("article content"),
//         }
//     } else {
//         Tweet {
//             username: String::from("wint"),
//             content: String::from("no"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }